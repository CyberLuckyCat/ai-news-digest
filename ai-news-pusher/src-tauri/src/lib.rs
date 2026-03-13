//! AI News Pusher - 库入口文件
//! 核心逻辑引擎模块
//!
//! 架构设计参考 ai-news-push-system-architecture-v2.md
//!
//! 模块划分：
//! - core::ai_provider: AI 提供商桥接
//! - core::collector: 信息采集器
//! - core::processor: 内容处理器
//! - core::scheduler: 定时任务调度器
//! - core::storage: 本地数据存储
//! - adapters::*: 多平台适配器

pub mod core;
pub mod adapters;
pub mod commands;

use std::sync::Mutex;

use tracing_subscriber::prelude::*;

use crate::core::storage::Storage;

/// 全局应用状态
pub struct AppState {
    pub storage: Mutex<Storage>,
}

/// 初始化日志系统
fn init_logging() {
    let log_dir = dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("ai-news-pusher")
        .join("logs");

    std::fs::create_dir_all(&log_dir).ok();

    let file_appender = tracing_appender::rolling::daily(&log_dir, "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stdout))
        .with(tracing_subscriber::fmt::layer().with_writer(non_blocking).with_ansi(false))
        .init();

    tracing::info!("AI News Pusher 启动中...");
}

/// 运行应用
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_logging();

    // 初始化存储
    let storage = Storage::new().expect("Failed to initialize storage");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            storage: Mutex::new(storage),
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_sources,
            commands::add_source,
            commands::delete_source,
            commands::get_news,
            commands::trigger_collect,
            commands::get_settings,
            commands::save_settings,
            commands::get_tasks,
            commands::add_task,
            commands::delete_task,
        ])
        .setup(|app| {
            tracing::info!("应用设置完成");

            // 启动定时任务调度器
            let handle = app.handle().clone();
            std::thread::spawn(move || {
                core::scheduler::start_scheduler(handle);
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
