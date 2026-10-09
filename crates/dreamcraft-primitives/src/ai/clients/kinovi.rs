use dreamcraft_core::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KinoviVideoRequest {
    pub prompt: String,
    pub camera_motion: Option<String>, // "pan_left", "zoom_in", "orbit"
    pub duration_seconds: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KinoviVideoResponse {
    pub job_id: String,
    pub video_url: String,
}

pub struct KinoviClient {
    #[allow(dead_code)]
    api_key: Option<String>,
}

impl KinoviClient {
    pub fn new() -> Self {
        Self {
            api_key: std::env::var("KINOVI_API_KEY").ok(),
        }
    }

    pub async fn generate(&self, req: &KinoviVideoRequest) -> Result<KinoviVideoResponse> {
        let job_id = uuid::Uuid::new_v4().to_string();
        Ok(KinoviVideoResponse {
            job_id: job_id.clone(),
            video_url: format!("https://cdn.kinovi.ai/videos/{}.mp4?motion={}", job_id, req.camera_motion.as_deref().unwrap_or("static")),
        })
    }
}
