use dreamcraft_core::Result;

pub struct MockAiProvider;

impl MockAiProvider {
    pub fn generate_image_mock(prompt: &str, width: u32, height: u32) -> Result<String> {
        // Generates a mock SVG or base64 data URI representing the requested image
        let svg = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\
            <rect width=\"100%\" height=\"100%\" fill=\"#1e1e2e\"/>\
            <circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"#89b4fa\" opacity=\"0.6\"/>\
            <text x=\"50%\" y=\"50%\" font-family=\"sans-serif\" font-size=\"24\" fill=\"#cdd6f4\" dominant-baseline=\"middle\" text-anchor=\"middle\">DreamCraft AI: {}</text>\
            </svg>",
            width, height, width, height, width / 2, height / 2, width.min(height) / 4, prompt
        );
        Ok(format!("data:image/svg+xml;utf8,{}", svg))
    }

    pub fn generate_video_mock(_prompt: &str, duration_secs: f64) -> Result<String> {
        Ok(format!("mock://video/{}.mp4?dur={}", uuid::Uuid::new_v4(), duration_secs))
    }

    pub fn generate_3d_mock(_prompt: &str) -> Result<String> {
        Ok(format!("mock://world/{}.splat", uuid::Uuid::new_v4()))
    }
}
