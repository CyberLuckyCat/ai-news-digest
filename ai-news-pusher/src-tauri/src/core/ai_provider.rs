//! AI 提供商桥接模块
//!
//! 统一管理多种 AI 服务的接入
//!
//! 支持的提供商：
//! - OpenAI (GPT-4o, GPT-4o-mini, O1)
//! - Anthropic (Claude)
//! - MiniMax
//! - MoonShot (月之暗面)
//! - Google Gemini

use reqwest;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AIError {
    #[error("请求错误: {0}")]
    Request(#[from] reqwest::Error),
    #[error("API 错误: {0}")]
    Api(String),
    #[error("配置错误: {0}")]
    Config(String),
    #[error("不支持的提供商: {0}")]
    UnsupportedProvider(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// AI 请求
pub struct AIRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32,
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 消息
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// AI 响应
pub struct AIResponse {
    pub content: String,
    pub model: String,
    pub usage: Usage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 使用量
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// AI 提供商类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIProviderType {
    OpenAI,
    Anthropic,
    MiniMax,
    MoonShot,
    Gemini,
}

/// AI 提供商 trait
#[allow(unused_variables)]
pub trait AIProvider: Send + Sync {
    fn provider_type(&self) -> AIProviderType;
    fn base_url(&self) -> &str;
    fn api_key(&self) -> &str;
    
    #[allow(unused_variables)]
    fn build_request(&self, model: &str, messages: &[Message], temperature: f32, max_tokens: Option<u32>) -> serde_json::Value {
        serde_json::json!({
            "model": model,
            "messages": messages,
            "temperature": temperature,
            "max_tokens": max_tokens.unwrap_or(4096),
        })
    }
    
    fn parse_response(&self, response: serde_json::Value) -> Result<AIResponse, AIError>;
}

/// OpenAI 提供商
pub struct OpenAIProvider {
    api_key: String,
    base_url: String,
}

impl OpenAIProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://api.openai.com/v1".to_string(),
        }
    }
}

impl AIProvider for OpenAIProvider {
    fn provider_type(&self) -> AIProviderType {
        AIProviderType::OpenAI
    }
    
    fn base_url(&self) -> &str {
        &self.base_url
    }
    
    fn api_key(&self) -> &str {
        &self.api_key
    }
    
    fn parse_response(&self, response: serde_json::Value) -> Result<AIResponse, AIError> {
        let content = response["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        let usage = response["usage"].clone();
        let usage = Usage {
            prompt_tokens: usage["prompt_tokens"].as_u64().unwrap_or(0) as u32,
            completion_tokens: usage["completion_tokens"].as_u64().unwrap_or(0) as u32,
            total_tokens: usage["total_tokens"].as_u64().unwrap_or(0) as u32,
        };

        Ok(AIResponse {
            content,
            model: response["model"].as_str().unwrap_or("").to_string(),
            usage,
        })
    }
}

/// MoonShot (月之暗面) 提供商
pub struct MoonShotProvider {
    api_key: String,
    base_url: String,
}

impl MoonShotProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://api.moonshot.cn/v1".to_string(),
        }
    }
}

impl AIProvider for MoonShotProvider {
    fn provider_type(&self) -> AIProviderType {
        AIProviderType::MoonShot
    }
    
    fn base_url(&self) -> &str {
        &self.base_url
    }
    
    fn api_key(&self) -> &str {
        &self.api_key
    }
    
    fn parse_response(&self, response: serde_json::Value) -> Result<AIResponse, AIError> {
        let content = response["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        let usage = response["usage"].clone();
        let usage = Usage {
            prompt_tokens: usage["prompt_tokens"].as_u64().unwrap_or(0) as u32,
            completion_tokens: usage["completion_tokens"].as_u64().unwrap_or(0) as u32,
            total_tokens: usage["total_tokens"].as_u64().unwrap_or(0) as u32,
        };

        Ok(AIResponse {
            content,
            model: response["model"].as_str().unwrap_or("").to_string(),
            usage,
        })
    }
}

/// Google Gemini 提供商
pub struct GeminiProvider {
    api_key: String,
    base_url: String,
}

impl GeminiProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://generativelanguage.googleapis.com/v1beta".to_string(),
        }
    }
    
    fn build_request(&self, _model: &str, messages: &[Message], temperature: f32, max_tokens: Option<u32>) -> serde_json::Value {
        // Gemini 使用不同的请求格式
        let contents: Vec<serde_json::Value> = messages.iter().map(|msg| {
            serde_json::json!({
                "role": if msg.role == "user" { "user" } else { "model" },
                "parts": [{"text": msg.content}]
            })
        }).collect();
        
        serde_json::json!({
            "contents": contents,
            "generationConfig": {
                "temperature": temperature,
                "maxOutputTokens": max_tokens.unwrap_or(4096),
            }
        })
    }
}

impl AIProvider for GeminiProvider {
    fn provider_type(&self) -> AIProviderType {
        AIProviderType::Gemini
    }
    
    fn base_url(&self) -> &str {
        &self.base_url
    }
    
    fn api_key(&self) -> &str {
        &self.api_key
    }
    
    fn parse_response(&self, response: serde_json::Value) -> Result<AIResponse, AIError> {
        let content = response["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("")
            .to_string();

        let usage = response["usageMetadata"].clone();
        let usage = Usage {
            prompt_tokens: usage["promptTokenCount"].as_u64().unwrap_or(0) as u32,
            completion_tokens: usage["candidatesTokenCount"].as_u64().unwrap_or(0) as u32,
            total_tokens: usage["totalTokenCount"].as_u64().unwrap_or(0) as u32,
        };

        Ok(AIResponse {
            content,
            model: response["modelVersion"].as_str().unwrap_or("gemini").to_string(),
            usage,
        })
    }
}

/// AI 包装器 - 防腐层设计
pub struct AIWrapper {
    providers: std::collections::HashMap<String, Box<dyn AIProvider>>,
    max_retries: u32,
}

impl AIWrapper {
    pub fn new() -> Self {
        Self {
            providers: std::collections::HashMap::new(),
            max_retries: 3,
        }
    }
    
    pub fn add_provider(&mut self, name: String, provider: Box<dyn AIProvider>) {
        self.providers.insert(name, provider);
    }
    
    pub fn get_provider(&self, name: &str) -> Option<&Box<dyn AIProvider>> {
        self.providers.get(name)
    }
    
    pub async fn chat(&self, provider_name: &str, request: &AIRequest) -> Result<AIResponse, AIError> {
        let provider = self.providers.get(provider_name)
            .ok_or_else(|| AIError::UnsupportedProvider(provider_name.to_string()))?;
            
        let client = reqwest::Client::new();
        let url = format!("{}/chat/completions", provider.base_url());
        
        let body = provider.build_request(
            &request.model,
            &request.messages,
            request.temperature,
            request.max_tokens
        );
        
        let mut last_error = None;
        
        for attempt in 0..self.max_retries {
            let response = client
                .post(&url)
                .header("Authorization", format!("Bearer {}", provider.api_key()))
                .header("Content-Type", "application/json")
                .json(&body)
                .send()
                .await;
            
            match response {
                Ok(resp) => {
                    if !resp.status().is_success() {
                        let error_text = resp.text().await.unwrap_or_default();
                        tracing::warn!("API 返回错误 (尝试 {}/{}): {}", attempt + 1, self.max_retries, error_text);
                        last_error = Some(AIError::Api(error_text));
                    } else {
                        let response_json: serde_json::Value = resp.json().await?;
                        return provider.parse_response(response_json);
                    }
                }
                Err(e) => {
                    tracing::warn!("请求失败 (尝试 {}/{}): {}", attempt + 1, self.max_retries, e);
                    last_error = Some(AIError::Request(e));
                }
            }
            
            // 指数退避
            tokio::time::sleep(tokio::time::Duration::from_millis(
                2u64.pow(attempt) * 100,
            )).await;
        }
        
        Err(last_error.unwrap_or(AIError::Api("重试失败".to_string())))
    }
}

impl Default for AIWrapper {
    fn default() -> Self {
        Self::new()
    }
}

/// 创建默认 AI 包装器
#[allow(unused_variables)]
pub fn create_ai_wrapper(api_key: &str, provider: &str, model: &str) -> Result<AIWrapper, AIError> {
    let mut wrapper = AIWrapper::new();
    
    match provider {
        "openai" => {
            wrapper.add_provider("openai".to_string(), Box::new(OpenAIProvider::new(api_key.to_string())));
        }
        "moonshot" => {
            wrapper.add_provider("moonshot".to_string(), Box::new(MoonShotProvider::new(api_key.to_string())));
        }
        "gemini" => {
            wrapper.add_provider("gemini".to_string(), Box::new(GeminiProvider::new(api_key.to_string())));
        }
        "anthropic" | "minimax" => {
            // 这些需要额外实现，先返回不支持
            return Err(AIError::UnsupportedProvider(provider.to_string()));
        }
        _ => {
            return Err(AIError::UnsupportedProvider(provider.to_string()));
        }
    }
    
    Ok(wrapper)
}
