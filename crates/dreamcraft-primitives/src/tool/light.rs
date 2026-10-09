use dreamcraft_core::Id;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhiteBalance {
    pub temperature: f64, // -100.0 (cool/blue) .. +100.0 (warm/yellow)
    pub tint: f64,        // -100.0 (green) .. +100.0 (magenta)
}

impl Default for WhiteBalance {
    fn default() -> Self {
        Self { temperature: 0.0, tint: 0.0 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToneSettings {
    pub exposure: f64,   // -5.0 .. +5.0 EV
    pub contrast: f64,   // -100.0 .. +100.0
    pub highlights: f64, // -100.0 .. +100.0
    pub shadows: f64,    // -100.0 .. +100.0
    pub whites: f64,     // -100.0 .. +100.0
    pub blacks: f64,     // -100.0 .. +100.0
}

impl Default for ToneSettings {
    fn default() -> Self {
        Self {
            exposure: 0.0,
            contrast: 0.0,
            highlights: 0.0,
            shadows: 0.0,
            whites: 0.0,
            blacks: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PresenceSettings {
    pub texture: f64,
    pub clarity: f64,
    pub dehaze: f64,
    pub vibrance: f64,
    pub saturation: f64,
}

impl Default for PresenceSettings {
    fn default() -> Self {
        Self {
            texture: 0.0,
            clarity: 0.0,
            dehaze: 0.0,
            vibrance: 0.0,
            saturation: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DenoiseSettings {
    pub luminance: f64, // 0.0 .. 100.0
    pub color: f64,     // 0.0 .. 100.0
}

impl Default for DenoiseSettings {
    fn default() -> Self {
        Self { luminance: 0.0, color: 25.0 }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DevelopSettings {
    pub wb: WhiteBalance,
    pub tone: ToneSettings,
    pub presence: PresenceSettings,
    pub denoise: DenoiseSettings,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LightPhoto {
    pub id: Id,
    pub path: String,
    pub rating: u8, // 0..5 stars
    pub flag: String, // "pick", "reject", "unflagged"
    pub iso: Option<u32>,
    pub shutter_speed: Option<String>,
    pub aperture: Option<f64>,
    pub settings: DevelopSettings,
}

impl LightPhoto {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            id: Id::new(),
            path: path.into(),
            rating: 0,
            flag: "unflagged".into(),
            iso: Some(400),
            shutter_speed: Some("1/250".into()),
            aperture: Some(2.8),
            settings: DevelopSettings::default(),
        }
    }

    pub fn set_exposure(&mut self, ev: f64) {
        self.settings.tone.exposure = ev.clamp(-5.0, 5.0);
    }

    pub fn set_white_balance(&mut self, temp: f64, tint: f64) {
        self.settings.wb.temperature = temp.clamp(-100.0, 100.0);
        self.settings.wb.tint = tint.clamp(-100.0, 100.0);
    }

    pub fn set_highlights_shadows(&mut self, highlights: f64, shadows: f64) {
        self.settings.tone.highlights = highlights.clamp(-100.0, 100.0);
        self.settings.tone.shadows = shadows.clamp(-100.0, 100.0);
    }
}
