use dreamcraft_core::{DreamError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrokChatRequest {
    pub prompt: String,
    pub system_instruction: Option<String>,
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrokChatResponse {
    pub text: String,
}

pub struct GrokClient {
    api_key: Option<String>,
}

impl GrokClient {
    pub fn new() -> Self {
        Self {
            api_key: std::env::var("XAI_API_KEY").ok(),
        }
    }

    pub fn with_api_key(key: impl Into<String>) -> Self {
        Self {
            api_key: Some(key.into()),
        }
    }

    pub async fn chat(&self, req: &GrokChatRequest) -> Result<GrokChatResponse> {
        let key = match &self.api_key {
            Some(k) => k,
            None => {
                return Ok(GrokChatResponse {
                    text: format!("AI Creative Reasoning Response for: {}", req.prompt),
                });
            }
        };

        let client = reqwest::Client::new();
        let payload = serde_json::json!({
            "model": "grok-2-latest",
            "messages": [
                {
                    "role": "system",
                    "content": req.system_instruction.as_deref().unwrap_or("You are an AI creative director.")
                },
                {
                    "role": "user",
                    "content": req.prompt
                }
            ],
            "temperature": req.temperature.unwrap_or(0.7)
        });

        let resp = client
            .post("https://api.x.ai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", key))
            .json(&payload)
            .send()
            .await
            .map_err(|e| DreamError::Ai(format!("Grok request error: {}", e)))?;

        if !resp.status().is_success() {
            let err_txt = resp.text().await.unwrap_or_default();
            return Err(DreamError::Ai(format!("Grok API error: {}", err_txt)));
        }

        let val: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| DreamError::Ai(format!("Grok json error: {}", e)))?;

        let text = val["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        Ok(GrokChatResponse { text })
    }
}
