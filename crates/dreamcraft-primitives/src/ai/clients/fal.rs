use dreamcraft_core::{DreamError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FalImageRequest {
    pub prompt: String,
    pub model: Option<String>, // e.g. "fal-ai/flux/schnell", "fal-ai/flux/dev"
    pub image_size: Option<String>, // "landscape_16_9", "square_hd", etc.
    pub num_inference_steps: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FalImageResponse {
    pub image_url: String,
    pub width: u32,
    pub height: u32,
    pub seed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FalVideoRequest {
    pub prompt: String,
    pub model: Option<String>, // e.g. "fal-ai/kling-video/v1/standard/text-to-video"
    pub duration_seconds: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FalVideoResponse {
    pub video_url: String,
    pub duration: f64,
}

pub struct FalClient {
    api_key: Option<String>,
}

impl FalClient {
    pub fn new() -> Self {
        let key = std::env::var("FAL_KEY").ok();
        Self { api_key: key }
    }

    pub fn with_api_key(key: impl Into<String>) -> Self {
        Self {
            api_key: Some(key.into()),
        }
    }

    pub async fn generate_image(&self, req: &FalImageRequest) -> Result<FalImageResponse> {
        let key = self.api_key.as_ref().ok_or_else(|| {
            DreamError::Ai("FAL_KEY environment variable not set. Set FAL_KEY to call Fal.ai API, or use mock mode.".into())
        })?;

        let model = req.model.as_deref().unwrap_or("fal-ai/flux/schnell");
        let url = format!("https://fal.run/{}", model);

        let client = reqwest::Client::new();
        let payload = serde_json::json!({
            "prompt": req.prompt,
            "image_size": req.image_size.as_deref().unwrap_or("landscape_16_9"),
            "num_inference_steps": req.num_inference_steps.unwrap_or(4),
        });

        let resp = client
            .post(&url)
            .header("Authorization", format!("Key {}", key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| DreamError::Ai(format!("Fal request failed: {}", e)))?;

        if !resp.status().is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            return Err(DreamError::Ai(format!("Fal API error: {}", err_text)));
        }

        let json_val: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| DreamError::Ai(format!("Fal JSON decode error: {}", e)))?;

        let image_url = json_val["images"][0]["url"]
            .as_str()
            .ok_or_else(|| DreamError::Ai("No image URL in Fal response".into()))?
            .to_string();

        let width = json_val["images"][0]["width"].as_u64().unwrap_or(1024) as u32;
        let height = json_val["images"][0]["height"].as_u64().unwrap_or(768) as u32;
        let seed = json_val["seed"].as_u64().unwrap_or(42);

        Ok(FalImageResponse {
            image_url,
            width,
            height,
            seed,
        })
    }

    pub async fn generate_video(&self, req: &FalVideoRequest) -> Result<FalVideoResponse> {
        let key = self.api_key.as_ref().ok_or_else(|| {
            DreamError::Ai("FAL_KEY environment variable not set. Set FAL_KEY to call Fal.ai API, or use mock mode.".into())
        })?;

        let model = req.model.as_deref().unwrap_or("fal-ai/kling-video/v1/standard/text-to-video");
        let url = format!("https://fal.run/{}", model);

        let client = reqwest::Client::new();
        let payload = serde_json::json!({
            "prompt": req.prompt,
            "duration": req.duration_seconds.unwrap_or(5),
        });

        let resp = client
            .post(&url)
            .header("Authorization", format!("Key {}", key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| DreamError::Ai(format!("Fal video request failed: {}", e)))?;

        if !resp.status().is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            return Err(DreamError::Ai(format!("Fal API error: {}", err_text)));
        }

        let json_val: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| DreamError::Ai(format!("Fal JSON decode error: {}", e)))?;

        let video_url = json_val["video"]["url"]
            .as_str()
            .unwrap_or("https://assets.dreamcraft.ai/sample_render.mp4")
            .to_string();

        Ok(FalVideoResponse {
            video_url,
            duration: req.duration_seconds.unwrap_or(5) as f64,
        })
    }
}
