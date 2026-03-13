//! 多平台适配器模块
//!
//! 用于将新闻推送到不同平台
//!
//! 支持的平台：
//! - Email: 邮件推送
//! - Telegram: Telegram Bot
//! - WeChat: 微信 (预留)
//! - QQ: QQ 群机器人 (预留)
//! - Feishu: 飞书机器人 (预留)

pub mod email;
pub mod telegram;
pub mod wechat;
pub mod qq;
pub mod feishu;

use serde::{Deserialize, Serialize};

/// 推送消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushMessage {
    pub title: String,
    pub content: String,
    pub summary: String,
    pub url: String,
    pub category: String,
    pub source_name: String,  // 来源名称
}

/// 推送结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushResult {
    pub success: bool,
    pub message_id: Option<String>,
    pub error: Option<String>,
}

/// 推送渠道类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChannelType {
    Email,
    Telegram,
    WeChat,
    QQ,
    Feishu,
    Webhook,
}

impl std::fmt::Display for ChannelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChannelType::Email => write!(f, "email"),
            ChannelType::Telegram => write!(f, "telegram"),
            ChannelType::WeChat => write!(f, "wechat"),
            ChannelType::QQ => write!(f, "qq"),
            ChannelType::Feishu => write!(f, "feishu"),
            ChannelType::Webhook => write!(f, "webhook"),
        }
    }
}
