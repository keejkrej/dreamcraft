use dreamcraft_core::Id;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhiteBalance {
    pub kelvin: u32,      // 2000K .. 12000K (5500K default)
    pub temperature: f64, // -100.0 (cool/blue) .. +100.0 (warm/yellow)
    pub tint: f64,        // -100.0 (green) .. +100.0 (magenta)
}

impl Default for WhiteBalance {
    fn default() -> Self {
        Self {
            kelvin: 5500,
            temperature: 0.0,
            tint: 0.0,
        }
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
pub struct CurvePoint {
    pub input: f64,  // 0.0 .. 1.0
    pub output: f64, // 0.0 .. 1.0
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ToneCurve {
    pub highlights: f64, // -100.0 .. 100.0
    pub lights: f64,
    pub darks: f64,
    pub shadows: f64,
    pub rgb_points: Vec<CurvePoint>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct HslChannel {
    pub hue: f64,        // -100.0 .. +100.0
    pub saturation: f64, // -100.0 .. +100.0
    pub luminance: f64,  // -100.0 .. +100.0
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct HslMixer {
    pub red: HslChannel,
    pub orange: HslChannel,
    pub yellow: HslChannel,
    pub green: HslChannel,
    pub aqua: HslChannel,
    pub blue: HslChannel,
    pub purple: HslChannel,
    pub magenta: HslChannel,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ColorGradingZone {
    pub hue: f64,        // 0.0 .. 360.0 degrees
    pub saturation: f64, // 0.0 .. 100.0
    pub luminance: f64,  // -100.0 .. +100.0
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ColorGrading {
    pub shadows: ColorGradingZone,
    pub midtones: ColorGradingZone,
    pub highlights: ColorGradingZone,
    pub balance: f64, // -100.0 .. +100.0
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DenoiseSettings {
    pub luminance: f64, // 0.0 .. 100.0
    pub detail: f64,
    pub color: f64, // 0.0 .. 100.0
}

impl Default for DenoiseSettings {
    fn default() -> Self {
        Self {
            luminance: 0.0,
            detail: 50.0,
            color: 25.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharpeningSettings {
    pub amount: f64, // 0.0 .. 150.0
    pub radius: f64, // 0.5 .. 3.0
    pub detail: f64, // 0.0 .. 100.0
    pub masking: f64, // 0.0 .. 100.0
}

impl Default for SharpeningSettings {
    fn default() -> Self {
        Self {
            amount: 40.0,
            radius: 1.0,
            detail: 25.0,
            masking: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DevelopSettings {
    pub wb: WhiteBalance,
    pub tone: ToneSettings,
    pub presence: PresenceSettings,
    pub curve: ToneCurve,
    pub hsl: HslMixer,
    pub grading: ColorGrading,
    pub sharpening: SharpeningSettings,
    pub denoise: DenoiseSettings,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LightPhoto {
    pub id: Id,
    pub path: String,
    pub rating: u8,   // 0..5 stars
    pub flag: String, // "pick", "reject", "unflagged"
    pub camera_make: Option<String>,
    pub camera_model: Option<String>,
    pub iso: Option<u32>,
    pub shutter_speed: Option<String>,
    pub aperture: Option<f64>,
    pub focal_length_mm: Option<f64>,
    pub settings: DevelopSettings,
}

impl LightPhoto {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            id: Id::new(),
            path: path.into(),
            rating: 0,
            flag: "unflagged".into(),
            camera_make: Some("Sony".into()),
            camera_model: Some("A7 IV".into()),
            iso: Some(400),
            shutter_speed: Some("1/250".into()),
            aperture: Some(2.8),
            focal_length_mm: Some(50.0),
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

    pub fn apply_preset(&mut self, preset_name: &str) {
        match preset_name.to_lowercase().as_str() {
            "cinematic_warm" => {
                self.settings.wb.temperature = 15.0;
                self.settings.tone.contrast = 20.0;
                self.settings.tone.highlights = -25.0;
                self.settings.tone.shadows = 30.0;
                self.settings.presence.clarity = 12.0;
                self.settings.grading.shadows.hue = 210.0;
                self.settings.grading.shadows.saturation = 18.0;
                self.settings.grading.highlights.hue = 42.0;
                self.settings.grading.highlights.saturation = 24.0;
            }
            "moody_cold" => {
                self.settings.wb.temperature = -20.0;
                self.settings.wb.tint = 5.0;
                self.settings.tone.contrast = 35.0;
                self.settings.tone.blacks = -15.0;
                self.settings.presence.dehaze = 15.0;
                self.settings.presence.vibrance = -10.0;
                self.settings.grading.shadows.hue = 220.0;
                self.settings.grading.shadows.saturation = 30.0;
            }
            "vibrant_landscape" => {
                self.settings.tone.exposure = 0.3;
                self.settings.tone.highlights = -40.0;
                self.settings.tone.shadows = 40.0;
                self.settings.tone.whites = 15.0;
                self.settings.presence.texture = 15.0;
                self.settings.presence.clarity = 15.0;
                self.settings.presence.dehaze = 10.0;
                self.settings.presence.vibrance = 25.0;
                self.settings.hsl.blue.saturation = 20.0;
                self.settings.hsl.green.saturation = 15.0;
            }
            "clean_portrait" => {
                self.settings.wb.temperature = 5.0;
                self.settings.tone.highlights = -15.0;
                self.settings.tone.shadows = 20.0;
                self.settings.presence.texture = -10.0;
                self.settings.presence.clarity = -5.0;
                self.settings.presence.vibrance = 10.0;
                self.settings.hsl.orange.luminance = 10.0;
                self.settings.hsl.orange.saturation = -5.0;
            }
            _ => {}
        }
    }
}
