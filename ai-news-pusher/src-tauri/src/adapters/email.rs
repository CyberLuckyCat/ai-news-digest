//! 邮件推送适配器
//!
//! 使用 SMTP 协议发送邮件

use super::{PushMessage, PushResult};
use lettre::message::header::ContentType;
use lettre::message::Message;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::SmtpTransport;
use lettre::Transport;
use std::time::Duration;

/// 邮件配置
#[derive(Debug, Clone)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub username: String,
    pub password: String,
    pub from_email: String,
    pub from_name: String,
}

/// 邮件推送器
pub struct EmailAdapter {
    config: EmailConfig,
}

impl EmailAdapter {
    pub fn new(config: EmailConfig) -> Self {
        Self { config }
    }

    /// 发送邮件
    pub async fn send(&self, message: &PushMessage, target: &str) -> Result<PushResult, String> {
        tracing::info!("发送邮件到: {}", target);

        // 验证目标邮箱格式
        if !self::is_valid_email(target) {
            return Err("无效的邮箱地址".to_string());
        }

        // 构建邮件内容
        let email = Message::builder()
            .from(format!("{} <{}>", self.config.from_name, self.config.from_email).parse().unwrap())
            .to(target.parse().unwrap())
            .subject(&message.title)
            .header(ContentType::parse("text/html").unwrap())
            .body(format!(
                r#"<html>
<body>
<h1>{}</h1>
<p>{}</p>
<hr>
<p>来源: {}</p>
<p>分类: {}</p>
<p>原文链接: <a href="{}">{}</a></p>
</body>
</html>"#,
                message.title,
                message.content.replace('\n', "<br>"),
                message.source_name,
                message.category,
                message.url,
                message.url
            ))
            .map_err(|e| format!("构建邮件失败: {}", e))?;

        // 创建 SMTP 传输
        let creds = Credentials::new(
            self.config.username.clone(),
            self.config.password.clone(),
        );

        let mailer = SmtpTransport::starttls_relay(&self.config.smtp_host)
            .map_err(|e| format!("创建 SMTP 连接失败: {}", e))?
            .port(self.config.smtp_port)
            .credentials(creds)
            .timeout(Some(Duration::from_secs(30)))
            .build();

        // 发送邮件
        match mailer.send(&email) {
            Ok(_) => {
                tracing::info!("邮件发送成功到: {}", target);
                Ok(PushResult {
                    success: true,
                    message_id: Some(format!("email_{}", chrono::Utc::now().timestamp())),
                    error: None,
                })
            }
            Err(e) => {
                tracing::error!("邮件发送失败: {}", e);
                Err(format!("发送失败: {}", e))
            }
        }
    }
}

/// 验证邮箱格式
fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.contains('.') && email.len() > 3
}

/// 从 API Key 解析邮件配置
/// 格式: smtp_host:port:username:password:from_email:from_name
pub fn parse_email_config_from_key(api_key: &str) -> Option<EmailConfig> {
    let parts: Vec<&str> = api_key.split(':').collect();
    if parts.len() >= 6 {
        Some(EmailConfig {
            smtp_host: parts[0].to_string(),
            smtp_port: parts[1].parse().unwrap_or(587),
            username: parts[2].to_string(),
            password: parts[3].to_string(),
            from_email: parts[4].to_string(),
            from_name: parts[5].to_string(),
        })
    } else {
        None
    }
}
