use dreamcraft_core::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidjourneyRequest {
    pub prompt: String,
    pub aspect_ratio: Option<String>, // "--ar 16:9"
    pub stylize: Option<u32>,        // "--s 250"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidjourneyResponse {
    pub task_id: String,
    pub image_urls: Vec<String>,
}

pub struct MidjourneyClient {
    #[allow(dead_code)]
    api_key: Option<String>,
}

impl MidjourneyClient {
    pub fn new() -> Self {
        Self {
            api_key: std::env::var("MIDJOURNEY_API_KEY").ok(),
        }
    }

    #[allow(dead_code)]
    pub fn with_api_key(key: impl Into<String>) -> Self {
        Self {
            api_key: Some(key.into()),
        }
    }

    pub async fn imagine(&self, _req: &MidjourneyRequest) -> Result<MidjourneyResponse> {
        let task_id = uuid::Uuid::new_v4().to_string();
        Ok(MidjourneyResponse {
            task_id: task_id.clone(),
            image_urls: vec![
                format!("https://cdn.midjourney.com/{}/0_0.png", task_id),
                format!("https://cdn.midjourney.com/{}/0_1.png", task_id),
                format!("https://cdn.midjourney.com/{}/0_2.png", task_id),
                format!("https://cdn.midjourney.com/{}/0_3.png", task_id),
            ],
        })
    }
}
