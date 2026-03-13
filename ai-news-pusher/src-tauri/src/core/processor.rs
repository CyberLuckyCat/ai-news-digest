//! 内容处理器模块
//!
//! 对采集的原始内容进行 AI 处理、分类、摘要
//!
//! 遵循 SOLID 原则 - 简化实现

use crate::core::ai_provider::{create_ai_wrapper, AIRequest, Message};

/// 处理结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProcessedContent {
    pub title: String,
    pub content: String,
    pub summary: String,
    pub category: String,
    pub tags: Vec<String>,
    pub quality_score: f32,
}

/// AI 摘要处理器
pub struct AISummarizer {
    api_key: String,
    provider: String,
    model: String,
}

impl AISummarizer {
    pub fn new(api_key: String, provider: String, model: String) -> Self {
        Self { api_key, provider, model }
    }

    pub async fn process(&self, content: &str) -> Result<ProcessedContent, String> {
        let prompt = format!(
            r#"请分析以下内容，生成一个结构化的摘要。要求：
1. 提取关键信息
2. 分类（AI/机器人/游戏/科技/其他）
3. 生成不超过200字的摘要
4. 提取标签

内容：
{}

请按以下 JSON 格式返回：
{{
    "summary": "摘要内容",
    "category": "分类",
    "tags": ["标签1", "标签2"]
}}"#,
            content
        );

        let request = AIRequest {
            model: self.model.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
            temperature: 0.7,
            max_tokens: Some(1000),
        };

        let wrapper = create_ai_wrapper(&self.api_key, &self.provider, &self.model)
            .map_err(|e| e.to_string())?;
        
        let response = wrapper.chat(&self.provider, &request)
            .await
            .map_err(|e| e.to_string())?;

        // 解析 JSON 响应
        let result: serde_json::Value = serde_json::from_str(&response.content)
            .unwrap_or_else(|_| {
                serde_json::json!({
                    "summary": response.content,
                    "category": "other",
                    "tags": []
                })
            });

        Ok(ProcessedContent {
            title: String::new(),
            content: content.to_string(),
            summary: result["summary"].as_str().unwrap_or("").to_string(),
            category: result["category"].as_str().unwrap_or("other").to_string(),
            tags: result["tags"]
                .as_array()
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            quality_score: 0.8,
        })
    }
}

/// 简单文本处理器（无需 AI）
pub struct SimpleProcessor;

impl SimpleProcessor {
    pub fn new() -> Self {
        Self
    }

    pub async fn process(&self, content: &str) -> Result<ProcessedContent, String> {
        let first_line = content.lines().next().unwrap_or("").to_string();
        let summary = content.chars().take(200).collect::<String>();

        Ok(ProcessedContent {
            title: first_line.clone(),
            content: content.to_string(),
            summary,
            category: "general".to_string(),
            tags: vec![],
            quality_score: 0.5,
        })
    }
}

/// 质量评估器 - LLM-as-a-Judge
pub struct QualityAssessor {
    api_key: String,
    provider: String,
    model: String,
}

impl QualityAssessor {
    pub fn new(api_key: String, provider: String, model: String) -> Self {
        Self { api_key, provider, model }
    }

    /// 评估内容质量
    pub async fn assess(&self, content: &str) -> Result<QualityScore, String> {
        let prompt = format!(
            r#"请评估以下内容的技术质量，从以下维度打分（1-5分）：

1. 技术深度 - 内容的技术价值
2. 新颖性 - 是否为最新资讯
3. 可信度 - 来源是否可靠
4. 实操性 - 是否有实用价值

内容标题/摘要：
{}

请按以下 JSON 格式返回：
{{
    "technical_depth": 3,
    "novelty": 4,
    "credibility": 4,
    "practicality": 3,
    "overall": 3.5,
    "recommendation": "approve"
}}"#,
            content
        );

        let request = AIRequest {
            model: self.model.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
            temperature: 0.3,
            max_tokens: Some(500),
        };

        let wrapper = create_ai_wrapper(&self.api_key, &self.provider, &self.model)
            .map_err(|e| e.to_string())?;
        
        let response = wrapper.chat(&self.provider, &request)
            .await
            .map_err(|e| e.to_string())?;

        // 解析响应
        let result: serde_json::Value = serde_json::from_str(&response.content)
            .unwrap_or_else(|_| {
                serde_json::json!({
                    "technical_depth": 3,
                    "novelty": 3,
                    "credibility": 3,
                    "practicality": 3,
                    "overall": 3.0,
                    "recommendation": "manual_review"
                })
            });

        Ok(QualityScore {
            technical_depth: result["technical_depth"].as_f64().unwrap_or(3.0) as f32,
            novelty: result["novelty"].as_f64().unwrap_or(3.0) as f32,
            credibility: result["credibility"].as_f64().unwrap_or(3.0) as f32,
            practicality: result["practicality"].as_f64().unwrap_or(3.0) as f32,
            overall: result["overall"].as_f64().unwrap_or(3.0) as f32,
            recommendation: result["recommendation"].as_str().unwrap_or("manual_review").to_string(),
        })
    }
}

/// 质量评分
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QualityScore {
    pub technical_depth: f32,
    pub novelty: f32,
    pub credibility: f32,
    pub practicality: f32,
    pub overall: f32,
    pub recommendation: String,
}
