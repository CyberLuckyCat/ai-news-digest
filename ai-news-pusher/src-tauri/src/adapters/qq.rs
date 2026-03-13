//! QQ 群机器人适配器（预留）
//!
//! 预留用于未来实现 QQ 群推送

use super::{PushMessage, PushResult};

/// QQ 配置
pub struct QQConfig {
    pub server_url: String,
    pub group_id: String,
}

/// QQ 推送器（预留实现）
pub struct QQAdapter {
    config: QQConfig,
}

impl QQAdapter {
    pub fn new(config: QQConfig) -> Self {
        Self { config }
    }

    pub async fn send(&self, _message: &PushMessage, _target: &str) -> Result<PushResult, String> {
        // TODO: 实现 QQ 推送
        tracing::warn!("QQ 推送功能尚未实现");
        Ok(PushResult {
            success: false,
            message_id: None,
            error: Some("QQ 推送功能预留，尚未实现".to_string()),
        })
    }
}
