//! 飞书机器人适配器（预留）
//!
//! 预留用于未来实现飞书推送

use super::{PushMessage, PushResult};

/// 飞书配置
pub struct FeishuConfig {
    pub app_id: String,
    pub app_secret: String,
    pub webhook_url: Option<String>,
}

/// 飞书推送器（预留实现）
pub struct FeishuAdapter {
    config: FeishuConfig,
}

impl FeishuAdapter {
    pub fn new(config: FeishuConfig) -> Self {
        Self { config }
    }

    pub async fn send(&self, _message: &PushMessage, _target: &str) -> Result<PushResult, String> {
        // TODO: 实现飞书推送
        tracing::warn!("飞书推送功能尚未实现");
        Ok(PushResult {
            success: false,
            message_id: None,
            error: Some("飞书推送功能预留，尚未实现".to_string()),
        })
    }
}
