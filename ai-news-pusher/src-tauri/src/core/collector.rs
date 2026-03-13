//! 信息采集器模块
//!
//! 从多种来源采集资讯
//! 支持：RSS、Website
//!
//! 遵循 SOLID 原则 - 简化实现

use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CollectorError {
    #[error("请求错误: {0}")]
    Request(#[from] reqwest::Error),
    #[error("解析错误: {0}")]
    Parse(String),
    #[error("不支持的源类型")]
    UnsupportedSource,
}

/// 采集源配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceConfig {
    pub url: String,
    pub source_type: String,
}

/// 采集到的原始数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawData {
    pub title: String,
    pub content: String,
    pub url: String,
    pub published_at: String,
    pub category: String,
}

/// RSS 采集器
pub struct RSSCollector;

impl RSSCollector {
    pub fn new() -> Self {
        Self
    }

    pub async fn collect(&self, config: &SourceConfig) -> Result<Vec<RawData>, CollectorError> {
        let client = reqwest::Client::new();
        let response = client
            .get(&config.url)
            .send()
            .await?;

        let content = response.text().await?;
        let items = Self::parse_rss(&content, &config.source_type)?;

        Ok(items)
    }

    fn parse_rss(xml: &str, category: &str) -> Result<Vec<RawData>, CollectorError> {
        let mut items = Vec::new();

        let item_regex = Regex::new(r"<item>(.*?)</item>").unwrap();
        let title_regex = Regex::new(r"<title><!\[CDATA\[(.*?)\]\]></title>|<title>(.*?)</title>").unwrap();
        let link_regex = Regex::new(r"<link>(.*?)</link>").unwrap();
        let desc_regex = Regex::new(r"<description><!\[CDATA\[(.*?)\]\]></description>|<description>(.*?)</description>").unwrap();
        let date_regex = Regex::new(r"<pubDate>(.*?)</pubDate>").unwrap();

        for cap in item_regex.captures_iter(xml) {
            let item = &cap[1];

            let title = title_regex.captures(item)
                .map(|c| c.get(1).or(c.get(2)).map(|m| m.as_str().to_string()).unwrap_or_default())
                .unwrap_or_default();

            let url = link_regex.captures(item)
                .map(|c| c.get(1).map(|m| m.as_str().to_string()).unwrap_or_default())
                .unwrap_or_default();

            let content = desc_regex.captures(item)
                .map(|c| c.get(1).map(|m| m.as_str().to_string()).unwrap_or_default())
                .unwrap_or_default();

            let published_at = date_regex.captures(item)
                .map(|c| c.get(1).map(|m| m.as_str().to_string()).unwrap_or_default())
                .unwrap_or_else(|| chrono::Utc::now().to_rfc3339());

            if !title.is_empty() && !url.is_empty() {
                items.push(RawData {
                    title,
                    content,
                    url,
                    published_at,
                    category: category.to_string(),
                });
            }
        }

        Ok(items)
    }
}

/// 网站采集器（通用）
pub struct WebsiteCollector;

impl WebsiteCollector {
    pub fn new() -> Self {
        Self
    }

    pub async fn collect(&self, config: &SourceConfig) -> Result<Vec<RawData>, CollectorError> {
        let client = reqwest::Client::new();
        let response = client
            .get(&config.url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .send()
            .await?;

        let html = response.text().await?;
        let items = Self::parse_html(&html, &config.url);

        Ok(items)
    }

    fn parse_html(html: &str, base_url: &str) -> Vec<RawData> {
        let mut items = Vec::new();

        let title_regex = Regex::new(r"<title>(.*?)</title>").unwrap();
        let page_title = title_regex.captures(html)
            .map(|c| c.get(1).map(|m| m.as_str().to_string()).unwrap_or_default())
            .unwrap_or_default();

        let link_regex = Regex::new(r#"<a[^>]+href=["']([^"']+)["'][^>]*>([^<]*)</a>"#).unwrap();

        for cap in link_regex.captures_iter(html) {
            let url = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let text = cap.get(2).map(|m| m.as_str()).unwrap_or("").trim().to_string();

            if url.starts_with("http") && text.len() > 5 {
                items.push(RawData {
                    title: text,
                    content: String::new(),
                    url: url.to_string(),
                    published_at: chrono::Utc::now().to_rfc3339(),
                    category: "general".to_string(),
                });
            }

            if items.len() >= 20 {
                break;
            }
        }

        if items.is_empty() && !page_title.is_empty() {
            items.push(RawData {
                title: page_title,
                content: html.chars().take(500).collect(),
                url: base_url.to_string(),
                published_at: chrono::Utc::now().to_rfc3339(),
                category: "general".to_string(),
            });
        }

        items
    }
}

/// 采集器工厂
pub struct CollectorFactory;

impl CollectorFactory {
    pub fn create(source_type: &str) -> Result<Box<CollectorVariant>, CollectorError> {
        match source_type.to_lowercase().as_str() {
            "rss" => Ok(Box::new(CollectorVariant::RSS(RSSCollector::new()))),
            "website" => Ok(Box::new(CollectorVariant::Website(WebsiteCollector::new()))),
            _ => Err(CollectorError::UnsupportedSource),
        }
    }
}

/// 采集器变体（避免 dyn trait 问题）
pub enum CollectorVariant {
    RSS(RSSCollector),
    Website(WebsiteCollector),
}

impl CollectorVariant {
    pub async fn collect(&self, config: &SourceConfig) -> Result<Vec<RawData>, CollectorError> {
        match self {
            CollectorVariant::RSS(c) => c.collect(config).await,
            CollectorVariant::Website(c) => c.collect(config).await,
        }
    }
}
