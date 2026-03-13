//! 微信推送适配器（预留）
//!
//! 预留用于未来实现微信推送

use super::{PushMessage, PushResult};

/// 微信配置
pub struct WeChatConfig {
    pub app_id: String,
    pub app_secret: String,
    pub template_id: String,
}

/// 微信推送器（预留实现）
pub struct WeChatAdapter {
    config: WeChatConfig,
}

impl WeChatAdapter {
    pub fn new(config: WeChatConfig) -> Self {
        Self { config }
    }

    pub async fn send(&self, _message: &PushMessage, _target: &str) -> Result<PushResult, String> {
        // TODO: 实现微信推送
        tracing::warn!("微信推送功能尚未实现");
        Ok(PushResult {
            success: false,
            message_id: None,
            error: Some("微信推送功能预留，尚未实现".to_string()),
        })
    }
}
