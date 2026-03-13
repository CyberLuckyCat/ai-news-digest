//! Tauri 命令模块
//!
//! 定义前端与 Rust 后端交互的命令

use tauri::State;
use crate::AppState;
use crate::core::storage::{Source, News, Task, Settings};
use serde::Deserialize;

/// 添加订阅源请求
#[derive(Debug, Deserialize)]
pub struct AddSourceRequest {
    pub name: String,
    pub url: String,
    pub source_type: String,
    pub category: String,
}

/// 添加任务请求
#[derive(Debug, Deserialize)]
pub struct AddTaskRequest {
    pub name: String,
    pub cron_expression: String,
    pub action: String,
}

/// 获取所有订阅源
#[tauri::command]
pub fn get_sources(state: State<AppState>) -> Result<Vec<Source>, String> {
    let storage = state.storage.lock().map_err(|e| e.to_string())?;
    storage.get_sources().map_err(|e| e.to_string())
}

/// 添加订阅源
#[tauri::command]
pub fn add_source(state: State<AppState>, request: AddSourceRequest) -> Result<Source, String> {
    let storage = state.storage.lock().map_err(|e| e.to_string())?;
    storage.add_source(&request.name, &request.url, &request.source_type, &request.category)
        .map_err(|e| e.to_string())
}

/// 删除订阅源
#[tauri::command]
pub fn delete_source(state: State<AppState>, id: i64) -> Result<(), String> {
    let storage = state.storage.lock().map_err(|e| e.to_string())?;
    storage.delete_source(id).map_err(|e| e.to_string())
}

/// 获取新闻列表
#[tauri::command]
pub fn get_news(state: State<AppState>, limit: i64, offset: i64) -> Result<Vec<News>, String> {
    let storage = state.storage.lock().map_err(|e| e.to_string())?;
    storage.get_news(limit, offset).map_err(|e| e.to_string())
}

/// 手动触发采集
#[tauri::command]
pub async fn trigger_collect() -> Result<String, String> {
    // 这里可以调用实际的采集逻辑
    tracing::info!("手动触发采集");
    Ok("采集任务已启动".to_string())
}

/// 获取设置
#[tauri::command]
pub fn get_settings(state: State<AppState>) -> Result<Settings, String> {
    let storage = state.storage.lock().map_err(|e| e.to_string())?;
    storage.get_settings().map_err(|e| e.to_string())
}

/// 保存设置
#[tauri::command]
pub fn save_settings(state: State<AppState>, settings: Settings) -> Result<(), String> {
    let storage = state.storage.lock().map_err(|e| e.to_string())?;
    storage.save_settings(&settings).map_err(|e| e.to_string())
}

/// 获取所有任务
#[tauri::command]
pub fn get_tasks(state: State<AppState>) -> Result<Vec<Task>, String> {
    let storage = state.storage.lock().map_err(|e| e.to_string())?;
    storage.get_tasks().map_err(|e| e.to_string())
}

/// 添加任务
#[tauri::command]
pub fn add_task(state: State<AppState>, request: AddTaskRequest) -> Result<Task, String> {
    let storage = state.storage.lock().map_err(|e| e.to_string())?;
    storage.add_task(&request.name, &request.cron_expression, &request.action)
        .map_err(|e| e.to_string())
}

/// 删除任务
#[tauri::command]
pub fn delete_task(state: State<AppState>, id: i64) -> Result<(), String> {
    let storage = state.storage.lock().map_err(|e| e.to_string())?;
    storage.delete_task(id).map_err(|e| e.to_string())
}
