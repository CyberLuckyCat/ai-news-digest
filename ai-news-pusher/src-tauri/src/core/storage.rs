//! 本地数据存储模块
//!
//! 使用 SQLite 实现本地数据持久化
//! 遵循 SOLID 原则 - 单一职责

use rusqlite::{Connection, Result as SqliteResult, params};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;
use chrono::Utc;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("数据库错误: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("数据不存在")]
    NotFound,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 订阅源
pub struct Source {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub source_type: SourceType,
    pub category: String,
    pub enabled: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SourceType {
    Rss,
    Website,
    Firecrawl,
    Jina,
}

impl std::fmt::Display for SourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceType::Rss => write!(f, "rss"),
            SourceType::Website => write!(f, "website"),
            SourceType::Firecrawl => write!(f, "firecrawl"),
            SourceType::Jina => write!(f, "jina"),
        }
    }
}

impl std::str::FromStr for SourceType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rss" => Ok(SourceType::Rss),
            "website" => Ok(SourceType::Website),
            "firecrawl" => Ok(SourceType::Firecrawl),
            "jina" => Ok(SourceType::Jina),
            _ => Err(format!("Unknown source type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 新闻条目
pub struct News {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub summary: Option<String>,
    pub url: String,
    pub source_id: i64,
    pub category: String,
    pub published_at: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 定时任务
pub struct Task {
    pub id: i64,
    pub name: String,
    pub cron_expression: String,
    pub enabled: bool,
    pub action: TaskAction,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskAction {
    Collect,
    Process,
    Push,
    All,
}

impl std::fmt::Display for TaskAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskAction::Collect => write!(f, "collect"),
            TaskAction::Process => write!(f, "process"),
            TaskAction::Push => write!(f, "push"),
            TaskAction::All => write!(f, "all"),
        }
    }
}

impl std::str::FromStr for TaskAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "collect" => Ok(TaskAction::Collect),
            "process" => Ok(TaskAction::Process),
            "push" => Ok(TaskAction::Push),
            "all" => Ok(TaskAction::All),
            _ => Err(format!("Unknown task action: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 应用设置
pub struct Settings {
    pub ai_provider: String,
    pub api_key: String,
    pub model: String,
    pub push_channels: Vec<String>,
    pub channel_targets: std::collections::HashMap<String, String>,  // 渠道目标地址
    pub timezone: String,
}

/// 存储管理器
pub struct Storage {
    conn: Connection,
}

impl Storage {
    /// 创建新的存储实例
    pub fn new() -> Result<Self, StorageError> {
        let db_path = Self::get_db_path()?;
        let conn = Connection::open(&db_path)?;

        let storage = Self { conn };
        storage.init_tables()?;

        tracing::info!("数据库初始化完成: {:?}", db_path);
        Ok(storage)
    }

    fn get_db_path() -> SqliteResult<PathBuf> {
        let data_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("ai-news-pusher");

        std::fs::create_dir_all(&data_dir).ok();
        Ok(data_dir.join("data.db"))
    }

    fn init_tables(&self) -> SqliteResult<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS sources (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                url TEXT NOT NULL,
                source_type TEXT NOT NULL,
                category TEXT NOT NULL,
                enabled INTEGER DEFAULT 1,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS news (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                summary TEXT,
                url TEXT NOT NULL UNIQUE,
                source_id INTEGER NOT NULL,
                category TEXT NOT NULL,
                published_at TEXT NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY (source_id) REFERENCES sources(id)
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                cron_expression TEXT NOT NULL,
                enabled INTEGER DEFAULT 1,
                action TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )?;

        Ok(())
    }

    // ========== 订阅源操作 ==========

    /// 获取所有订阅源
    pub fn get_sources(&self) -> Result<Vec<Source>, StorageError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, url, source_type, category, enabled, created_at, updated_at FROM sources"
        )?;

        let sources = stmt.query_map([], |row| {
            let source_type_str: String = row.get(3)?;
            Ok(Source {
                id: row.get(0)?,
                name: row.get(1)?,
                url: row.get(2)?,
                source_type: source_type_str.parse().unwrap_or(SourceType::Website),
                category: row.get(4)?,
                enabled: row.get::<_, i32>(5)? == 1,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(sources)
    }

    /// 添加订阅源
    pub fn add_source(&self, name: &str, url: &str, source_type: &str, category: &str) -> Result<Source, StorageError> {
        let now = Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT INTO sources (name, url, source_type, category, enabled, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, 1, ?5, ?5)",
            params![name, url, source_type, category, now],
        )?;

        let id = self.conn.last_insert_rowid();
        Ok(Source {
            id,
            name: name.to_string(),
            url: url.to_string(),
            source_type: source_type.parse().unwrap_or(SourceType::Website),
            category: category.to_string(),
            enabled: true,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    /// 删除订阅源
    pub fn delete_source(&self, id: i64) -> Result<(), StorageError> {
        self.conn.execute("DELETE FROM sources WHERE id = ?1", params![id])?;
        Ok(())
    }

    // ========== 新闻操作 ==========

    /// 获取新闻列表
    pub fn get_news(&self, limit: i64, offset: i64) -> Result<Vec<News>, StorageError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, content, summary, url, source_id, category, published_at, created_at
             FROM news ORDER BY published_at DESC LIMIT ?1 OFFSET ?2"
        )?;

        let news = stmt.query_map(params![limit, offset], |row| {
            Ok(News {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                summary: row.get(3)?,
                url: row.get(4)?,
                source_id: row.get(5)?,
                category: row.get(6)?,
                published_at: row.get(7)?,
                created_at: row.get(8)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(news)
    }

    /// 添加新闻
    pub fn add_news(&self, news: &News) -> Result<i64, StorageError> {
        let now = Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT OR IGNORE INTO news (title, content, summary, url, source_id, category, published_at, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![news.title, news.content, news.summary, news.url, news.source_id, news.category, news.published_at, now],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    // ========== 任务操作 ==========

    /// 获取所有任务
    pub fn get_tasks(&self) -> Result<Vec<Task>, StorageError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, cron_expression, enabled, action, created_at FROM tasks"
        )?;

        let tasks = stmt.query_map([], |row| {
            let action_str: String = row.get(4)?;
            Ok(Task {
                id: row.get(0)?,
                name: row.get(1)?,
                cron_expression: row.get(2)?,
                enabled: row.get::<_, i32>(3)? == 1,
                action: action_str.parse().unwrap_or(TaskAction::All),
                created_at: row.get(5)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(tasks)
    }

    /// 添加任务
    pub fn add_task(&self, name: &str, cron_expression: &str, action: &str) -> Result<Task, StorageError> {
        let now = Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT INTO tasks (name, cron_expression, enabled, action, created_at) VALUES (?1, ?2, 1, ?3, ?4)",
            params![name, cron_expression, action, now],
        )?;

        let id = self.conn.last_insert_rowid();
        Ok(Task {
            id,
            name: name.to_string(),
            cron_expression: cron_expression.to_string(),
            enabled: true,
            action: action.parse().unwrap_or(TaskAction::All),
            created_at: now,
        })
    }

    /// 删除任务
    pub fn delete_task(&self, id: i64) -> Result<(), StorageError> {
        self.conn.execute("DELETE FROM tasks WHERE id = ?1", params![id])?;
        Ok(())
    }

    // ========== 设置操作 ==========

    /// 获取设置
    pub fn get_settings(&self) -> Result<Settings, StorageError> {
        let mut stmt = self.conn.prepare("SELECT key, value FROM settings")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;

        let mut settings = Settings {
            ai_provider: "openai".to_string(),
            api_key: String::new(),
            model: "gpt-4o-mini".to_string(),
            push_channels: vec![],
            channel_targets: std::collections::HashMap::new(),
            timezone: "Asia/Shanghai".to_string(),
        };

        for row in rows {
            let (key, value) = row?;
            match key.as_str() {
                "ai_provider" => settings.ai_provider = value,
                "api_key" => settings.api_key = value,
                "model" => settings.model = value,
                "push_channels" => settings.push_channels = serde_json::from_str(&value).unwrap_or_default(),
                "channel_targets" => settings.channel_targets = serde_json::from_str(&value).unwrap_or_default(),
                "timezone" => settings.timezone = value,
                _ => {}
            }
        }

        Ok(settings)
    }

    /// 保存设置
    pub fn save_settings(&self, settings: &Settings) -> Result<(), StorageError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES ('ai_provider', ?1)",
            params![settings.ai_provider],
        )?;
        self.conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES ('api_key', ?1)",
            params![settings.api_key],
        )?;
        self.conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES ('model', ?1)",
            params![settings.model],
        )?;
        self.conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES ('push_channels', ?1)",
            params![serde_json::to_string(&settings.push_channels).unwrap_or_default()],
        )?;
        self.conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES ('channel_targets', ?1)",
            params![serde_json::to_string(&settings.channel_targets).unwrap_or_default()],
        )?;
        self.conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES ('timezone', ?1)",
            params![settings.timezone],
        )?;

        Ok(())
    }
}
