//! Telegram 推送适配器
//!
//! 使用 Telegram Bot API 发送消息

use super::{PushMessage, PushResult};

/// Telegram 配置
pub struct TelegramConfig {
    pub bot_token: String,
    pub chat_id: String,
}

/// Telegram 推送器
pub struct TelegramAdapter {
    config: TelegramConfig,
    client: reqwest::Client,
}

impl TelegramAdapter {
    pub fn new(config: TelegramConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    /// 发送消息
    pub async fn send(&self, message: &PushMessage, _target: &str) -> Result<PushResult, String> {
        let text = format!(
            "📰 *{}*\n\n{}\n\n🔗 [查看原文]({})",
            message.title, message.summary, message.url
        );

        let url = format!(
            "https://api.telegram.org/bot{}/sendMessage",
            self.config.bot_token
        );

        let params = serde_json::json!({
            "chat_id": self.config.chat_id,
            "text": text,
            "parse_mode": "Markdown"
        });

        let response = self.client
            .post(&url)
            .json(&params)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if response.status().is_success() {
            let result: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
            let message_id = result["result"]["message_id"].as_i64();

            Ok(PushResult {
                success: true,
                message_id: message_id.map(|id| id.to_string()),
                error: None,
            })
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Ok(PushResult {
                success: false,
                message_id: None,
                error: Some(error_text),
            })
        }
    }
}
