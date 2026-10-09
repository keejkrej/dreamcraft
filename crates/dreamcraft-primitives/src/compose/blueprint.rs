use crate::tool::deck::DeckThemeKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartupLaunchBlueprint {
    pub name: String,
    pub pitch: String,
    pub founder: String,
    pub initial_arr: f64,
    pub growth_rate: f64,
    pub target_valuation: f64,
    pub team_size: usize,
    pub theme: DeckThemeKind,
}

impl Default for StartupLaunchBlueprint {
    fn default() -> Self {
        Self {
            name: "HyperScale AI".to_string(),
            pitch: "Autonomous AI-native operating system for creative enterprise".to_string(),
            founder: "Alex Vance".to_string(),
            initial_arr: 1_200_000.0,
            growth_rate: 3.2,
            target_valuation: 35_000_000.0,
            team_size: 12,
            theme: DeckThemeKind::Harbor,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneSpec {
    pub scene_index: usize,
    pub name: String,
    pub prompt: String,
    pub duration_seconds: f64,
    pub visual_style: String,
    pub audio_mood: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CinematicProductionBlueprint {
    pub title: String,
    pub logline: String,
    pub target_duration_seconds: f64,
    pub genre: String,
    pub scenes: Vec<SceneSpec>,
}

impl Default for CinematicProductionBlueprint {
    fn default() -> Self {
        Self {
            title: "Neon Horizon 2049".to_string(),
            logline: "An autonomous cyber-operative discovers the last biological archive in a silicon metropolis.".to_string(),
            target_duration_seconds: 45.0,
            genre: "Cyberpunk Thriller".to_string(),
            scenes: vec![
                SceneSpec {
                    scene_index: 1,
                    name: "Aerial Megacity".to_string(),
                    prompt: "Drone shot descending through rainy neon-lit skyscraper canyons, volumetric reflections, cinematic 8k".to_string(),
                    duration_seconds: 15.0,
                    visual_style: "Anamorphic Teal and Orange".to_string(),
                    audio_mood: "Deep synth drone with low rumble".to_string(),
                },
                SceneSpec {
                    scene_index: 2,
                    name: "The Neon Alley".to_string(),
                    prompt: "Close up of cybernetic agent walking under holographic neon ads, rain puddles, lens flare".to_string(),
                    duration_seconds: 15.0,
                    visual_style: "High contrast neon glow".to_string(),
                    audio_mood: "Percussive industrial pulse, footsteps in water".to_string(),
                },
                SceneSpec {
                    scene_index: 3,
                    name: "Vault Discovery".to_string(),
                    prompt: "Ancient brass vault doors opening revealing a bioluminescent seedling tree inside a glass chamber".to_string(),
                    duration_seconds: 15.0,
                    visual_style: "Warm golden bioluminescence amidst dark brushed steel".to_string(),
                    audio_mood: "Ethereal choir swell and resonant bell chime".to_string(),
                },
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleSpec {
    pub headline: String,
    pub author: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorialPublicationBlueprint {
    pub title: String,
    pub issue_number: usize,
    pub date_label: String,
    pub articles: Vec<ArticleSpec>,
    pub security_watermark: Option<String>,
}

impl Default for EditorialPublicationBlueprint {
    fn default() -> Self {
        Self {
            title: "Future of Autonomous Systems".to_string(),
            issue_number: 42,
            date_label: "Q4 2026 Special Edition".to_string(),
            articles: vec![
                ArticleSpec {
                    headline: "The Zero-Editing Paradigm in AI Engineering".to_string(),
                    author: "Dr. Evelyn Reed".to_string(),
                    content: "Autonomous synthesis replaces manual software tweaking. When deterministic tool primitives are orchestrated by high-level reasoning engines, software ceases to be a human UI tool and transforms into a pure compilation target for creative intention. The developer specifies the dream; the engine realizes the artifacts.".to_string(),
                },
                ArticleSpec {
                    headline: "Unified Cross-Craft Timebases and Geometry".to_string(),
                    author: "Marcus Chen".to_string(),
                    content: "Harmonizing video frames, audio sample clocks, vector coordinate spaces, and typographic leading into a single reactive execution graph enables unprecedented cross-domain velocity. Assets flow without lossy serialization or manual conversion friction.".to_string(),
                },
            ],
            security_watermark: Some("PRE-RELEASE EMBARGOED".to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentSpec {
    pub name: String,
    pub width_mm: f64,
    pub height_mm: f64,
    pub unit_cost: f64,
    pub quantity: usize,
    pub material: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineeringSpecBlueprint {
    pub title: String,
    pub revision: String,
    pub author: String,
    pub components: Vec<ComponentSpec>,
    pub tolerance_mm: f64,
}

impl Default for EngineeringSpecBlueprint {
    fn default() -> Self {
        Self {
            title: "Autonomous Drone Chassis v4".to_string(),
            revision: "REV-B".to_string(),
            author: "Chief Systems Architect".to_string(),
            components: vec![
                ComponentSpec {
                    name: "Carbon Main Spar".to_string(),
                    width_mm: 450.0,
                    height_mm: 35.0,
                    unit_cost: 85.50,
                    quantity: 4,
                    material: "Toray T800 Carbon Fiber".to_string(),
                },
                ComponentSpec {
                    name: "Avionics Core Enclosure".to_string(),
                    width_mm: 120.0,
                    height_mm: 80.0,
                    unit_cost: 140.00,
                    quantity: 1,
                    material: "CNC Aluminum 7075-T6".to_string(),
                },
                ComponentSpec {
                    name: "Optical Gimbal Ring".to_string(),
                    width_mm: 65.0,
                    height_mm: 65.0,
                    unit_cost: 45.00,
                    quantity: 2,
                    material: "Anodized Titanium Alloy".to_string(),
                },
            ],
            tolerance_mm: 0.05,
        }
    }
}
