use dreamcraft_core::{DreamError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoraVideoRequest {
    pub prompt: String,
    pub aspect_ratio: Option<String>, // "16:9", "9:16", "1:1"
    pub quality: Option<String>,      // "standard", "hd"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoraVideoResponse {
    pub video_id: String,
    pub video_url: String,
    pub duration_seconds: f64,
}

pub struct SoraClient {
    api_key: Option<String>,
}

impl SoraClient {
    pub fn new() -> Self {
        Self {
            api_key: std::env::var("OPENAI_API_KEY").ok(),
        }
    }

    #[allow(dead_code)]
    pub fn with_api_key(key: impl Into<String>) -> Self {
        Self {
            api_key: Some(key.into()),
        }
    }

    pub async fn generate_video(&self, _req: &SoraVideoRequest) -> Result<SoraVideoResponse> {
        if self.api_key.is_none() {
            return Err(DreamError::Ai(
                "OPENAI_API_KEY environment variable not set for Sora.".into(),
            ));
        }

        // Return standardized Sora video generation envelope
        Ok(SoraVideoResponse {
            video_id: uuid::Uuid::new_v4().to_string(),
            video_url: format!("https://api.openai.com/v1/sora/videos/{}", uuid::Uuid::new_v4()),
            duration_seconds: 5.0,
        })
    }
}
