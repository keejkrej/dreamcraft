use dreamcraft_core::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldLabsRequest {
    pub prompt: String,
    pub input_image_url: Option<String>,
    pub format: Option<String>, // "gaussian_splat", "mesh_obj", "nerf"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldLabsResponse {
    pub world_id: String,
    pub splat_ply_url: String,
    pub viewer_url: String,
}

pub struct WorldLabsClient {
    #[allow(dead_code)]
    api_key: Option<String>,
}

impl WorldLabsClient {
    pub fn new() -> Self {
        Self {
            api_key: std::env::var("WORLDLABS_API_KEY").ok(),
        }
    }

    #[allow(dead_code)]
    pub fn with_api_key(key: impl Into<String>) -> Self {
        Self {
            api_key: Some(key.into()),
        }
    }

    pub async fn generate_world(&self, _req: &WorldLabsRequest) -> Result<WorldLabsResponse> {
        let world_id = uuid::Uuid::new_v4().to_string();
        Ok(WorldLabsResponse {
            world_id: world_id.clone(),
            splat_ply_url: format!("https://storage.worldlabs.ai/{}/scene.splat", world_id),
            viewer_url: format!("https://view.worldlabs.ai/{}", world_id),
        })
    }
}
