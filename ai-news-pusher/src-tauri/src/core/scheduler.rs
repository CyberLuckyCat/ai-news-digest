//! 定时任务调度器模块
//!
//! 基于 cron 表达式实现定时任务调度
//!
//! 遵循 SOLID 原则 - 单一职责

use chrono::Timelike;
use cron::Schedule;
use std::str::FromStr;
use std::time::Duration;
use tauri::AppHandle;
use tokio::sync::mpsc;
use tracing::{info, error};

/// 调度器消息
#[derive(Debug)]
pub enum SchedulerMessage {
    TriggerCollect,
    TriggerProcess,
    TriggerPush,
    TriggerAll,
}

/// 启动调度器
pub fn start_scheduler(_app_handle: AppHandle) {
    let (_tx, mut rx) = mpsc::channel::<SchedulerMessage>(100);

    // 启动定时检查任务
    tokio::spawn(async move {
        info!("调度器启动");

        loop {
            // 每分钟检查一次
            tokio::time::sleep(Duration::from_secs(60)).await;

            // 获取当前时间
            let now = chrono::Utc::now();

            // 示例：每天 8 点执行
            if now.hour() == 8 && now.minute() == 0 {
                info!("触发每日新闻采集");
                let _ = _tx.send(SchedulerMessage::TriggerAll).await;
            }

            // 示例：每 6 小时执行一次
            if now.hour() % 6 == 0 && now.minute() == 0 {
                info!("触发定时采集");
                let _ = _tx.send(SchedulerMessage::TriggerCollect).await;
            }
        }
    });

    // 处理调度消息
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            info!("收到调度消息: {:?}", msg);
            match msg {
                SchedulerMessage::TriggerCollect => {
                    if let Err(e) = execute_collect().await {
                        error!("采集失败: {}", e);
                    }
                }
                SchedulerMessage::TriggerProcess => {
                    if let Err(e) = execute_process().await {
                        error!("处理失败: {}", e);
                    }
                }
                SchedulerMessage::TriggerPush => {
                    if let Err(e) = execute_push().await {
                        error!("推送失败: {}", e);
                    }
                }
                SchedulerMessage::TriggerAll => {
                    info!("执行完整流程: 采集 -> 处理 -> 推送");
                    if let Err(e) = execute_collect().await {
                        error!("采集失败: {}", e);
                    }
                    if let Err(e) = execute_process().await {
                        error!("处理失败: {}", e);
                    }
                    if let Err(e) = execute_push().await {
                        error!("推送失败: {}", e);
                    }
                }
            }
        }
    });
}

/// 执行采集任务
async fn execute_collect() -> Result<(), String> {
    info!("开始执行采集任务");
    // 实际实现中：调用采集器
    info!("采集任务完成");
    Ok(())
}

/// 执行处理任务
async fn execute_process() -> Result<(), String> {
    info!("开始执行处理任务");
    // 实际实现中：调用 AI 处理器
    info!("处理任务完成");
    Ok(())
}

/// 执行推送任务
async fn execute_push() -> Result<(), String> {
    info!("开始执行推送任务");
    // 实际实现中：调用推送适配器
    info!("推送任务完成");
    Ok(())
}

/// Cron 调度器辅助类
pub struct CronScheduler {
    schedule: Schedule,
    action: String,
}

impl CronScheduler {
    /// 从 cron 表达式创建调度器
    pub fn new(cron_expr: &str, action: &str) -> Result<Self, String> {
        let schedule = Schedule::from_str(cron_expr)
            .map_err(|e| format!("无效的 cron 表达式: {}", e))?;

        Ok(Self {
            schedule,
            action: action.to_string(),
        })
    }

    /// 检查是否应该执行
    pub fn should_run(&self, now: &chrono::DateTime<chrono::Utc>) -> bool {
        self.schedule.upcoming(chrono::Utc).next()
            .map(|next| *now >= next)
            .unwrap_or(false)
    }
}
