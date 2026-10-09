use crate::ai::clients::fal::{FalClient, FalImageRequest, FalVideoRequest};
use crate::ai::mock::MockAiProvider;
use crate::tool::deck::Presentation;
use crate::tool::film::Sequence;
use crate::tool::word::{HeadingLevel, WordDocument};
use dreamcraft_core::{Rect, Result, Tick};

pub struct CreativePipeline;

impl CreativePipeline {
    /// Generates an image using Fal or Mock fallback.
    pub async fn generate_image(prompt: &str, width: u32, height: u32) -> Result<String> {
        let fal = FalClient::new();
        if std::env::var("FAL_KEY").is_ok() {
            match fal
                .generate_image(&FalImageRequest {
                    prompt: prompt.to_string(),
                    model: Some("fal-ai/flux/schnell".to_string()),
                    image_size: Some("landscape_16_9".to_string()),
                    num_inference_steps: Some(4),
                })
                .await
            {
                Ok(resp) => return Ok(resp.image_url),
                Err(_) => {} // fallback to mock
            }
        }
        MockAiProvider::generate_image_mock(prompt, width, height)
    }

    /// Generates a video clip using Fal or Mock fallback.
    pub async fn generate_video(prompt: &str, duration_secs: f64) -> Result<String> {
        let fal = FalClient::new();
        if std::env::var("FAL_KEY").is_ok() {
            match fal
                .generate_video(&FalVideoRequest {
                    prompt: prompt.to_string(),
                    model: Some("fal-ai/kling-video/v1/standard/text-to-video".to_string()),
                    duration_seconds: Some(duration_secs.round() as u32),
                })
                .await
            {
                Ok(resp) => return Ok(resp.video_url),
                Err(_) => {} // fallback to mock
            }
        }
        MockAiProvider::generate_video_mock(prompt, duration_secs)
    }

    /// Composes an AI illustrated report into a WordDocument.
    pub async fn compose_illustrated_report(topic: &str) -> Result<WordDocument> {
        let mut doc = WordDocument::new(format!("Executive Report: {}", topic));
        doc.add_heading(HeadingLevel::Heading1, "Overview & Vision");
        doc.add_paragraph(format!(
            "This report synthesizes key insights, market trends, and technical architectures for {}.",
            topic
        ));

        let img_uri = Self::generate_image(&format!("High quality infographic visualization of {}", topic), 1024, 768).await?;
        doc.add_paragraph(format!("[Visual Asset: {}]", img_uri));

        doc.add_heading(HeadingLevel::Heading2, "Action Plan");
        doc.insert_table(
            vec![
                vec!["Phase".into(), "Milestone".into(), "Status".into()],
                vec!["Phase 1".into(), "Core Primitives Unification".into(), "Completed".into()],
                vec!["Phase 2".into(), "AI Generation Pipelines".into(), "Active".into()],
                vec!["Phase 3".into(), "Cross-App Autonomous Composition".into(), "Planned".into()],
            ],
            true,
        );

        Ok(doc)
    }

    /// Composes an AI generated multi-slide pitch presentation into a Presentation.
    pub async fn compose_pitch_deck(title: &str, num_slides: usize) -> Result<Presentation> {
        let mut deck = Presentation::new(title);
        for i in 1..=num_slides {
            let slide_idx = deck.add_slide(
                crate::tool::deck::SlideLayout::TitleAndContent,
                format!("Topic {}: Deep Dive", i),
            );
            if let Some(slide) = deck.slides.get_mut(slide_idx) {
                slide.add_bullet(format!("Strategic advantage in sector {}", i), 0.0);
                slide.add_bullet("High performance Rust core with deterministic execution", 50.0);
                slide.add_bullet("Fully autonomous AI agent manipulation via MCP", 100.0);

                let img = Self::generate_image(&format!("Abstract modern 3D icon for topic {}", i), 500, 300).await?;
                slide.add_image(img, Rect::new(450.0, 160.0, 400.0, 240.0));
            }
        }
        Ok(deck)
    }

    /// Composes an AI generated film sequence into a Film Sequence timeline.
    pub async fn compose_ai_film(title: &str, scenes: Vec<&str>) -> Result<Sequence> {
        let mut seq = Sequence::new(title);
        let v1_id = seq.video_tracks[0].id;
        let mut current_time = Tick::ZERO;

        for (idx, prompt) in scenes.iter().enumerate() {
            let video_url = Self::generate_video(prompt, 4.0).await?;
            let duration = Tick::from_seconds(4.0);
            seq.insert_clip(v1_id, format!("Scene {:02}", idx + 1), video_url, current_time, duration)?;
            current_time = current_time + duration;
        }

        Ok(seq)
    }
}
