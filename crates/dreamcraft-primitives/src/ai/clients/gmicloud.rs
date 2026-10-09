use dreamcraft_core::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GmiCloudRequest {
    pub model: String, // e.g. "llama-3-70b-instruct"
    pub prompt: String,
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GmiCloudResponse {
    pub text: String,
    pub latency_ms: u64,
}

pub struct GmiCloudClient {
    #[allow(dead_code)]
    api_key: Option<String>,
}

impl GmiCloudClient {
    pub fn new() -> Self {
        Self {
            api_key: std::env::var("GMICLOUD_API_KEY").ok(),
        }
    }

    pub async fn infer(&self, req: &GmiCloudRequest) -> Result<GmiCloudResponse> {
        Ok(GmiCloudResponse {
            text: format!("GMICloud inference [{}] on: {}", req.model, req.prompt),
            latency_ms: 120,
        })
    }
}
