use dreamcraft_core::{Color, Id};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum KeyInterpolation {
    Linear,
    Hold,
    EaseIn,
    EaseOut,
    EasyEase,
    Bezier {
        in_speed: f64,
        in_influence: f64, // 0.1 .. 100.0%
        out_speed: f64,
        out_influence: f64,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BlendMode {
    #[default]
    Normal,
    Multiply,
    Screen,
    Overlay,
    SoftLight,
    HardLight,
    ColorDodge,
    ColorBurn,
    Difference,
    Add,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TrackMatte {
    #[default]
    None,
    Alpha,
    AlphaInverted,
    Luma,
    LumaInverted,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Keyframe<T> {
    pub frame: i64,
    pub value: T,
    pub interpolation: KeyInterpolation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimatedProperty<T> {
    pub static_value: T,
    pub keyframes: Vec<Keyframe<T>>,
}

impl<T: Clone> AnimatedProperty<T> {
    pub fn new(static_val: T) -> Self {
        Self {
            static_value: static_val,
            keyframes: Vec::new(),
        }
    }

    pub fn add_keyframe(&mut self, frame: i64, value: T, interpolation: KeyInterpolation) {
        self.keyframes.push(Keyframe {
            frame,
            value,
            interpolation,
        });
        self.keyframes.sort_by_key(|k| k.frame);
    }
}

impl AnimatedProperty<f64> {
    pub fn value_at(&self, frame: f64) -> f64 {
        if self.keyframes.is_empty() {
            return self.static_value;
        }
        if frame <= self.keyframes[0].frame as f64 {
            return self.keyframes[0].value;
        }
        if frame >= self.keyframes.last().unwrap().frame as f64 {
            return self.keyframes.last().unwrap().value;
        }

        // Find surrounding keyframes
        for w in self.keyframes.windows(2) {
            let k0 = &w[0];
            let k1 = &w[1];
            if frame >= k0.frame as f64 && frame <= k1.frame as f64 {
                let t = (frame - k0.frame as f64) / (k1.frame as f64 - k0.frame as f64);
                return match k0.interpolation {
                    KeyInterpolation::Hold => k0.value,
                    KeyInterpolation::Linear => k0.value + (k1.value - k0.value) * t,
                    KeyInterpolation::EaseIn => {
                        let t2 = t * t;
                        k0.value + (k1.value - k0.value) * t2
                    }
                    KeyInterpolation::EaseOut => {
                        let t2 = t * (2.0 - t);
                        k0.value + (k1.value - k0.value) * t2
                    }
                    KeyInterpolation::EasyEase | KeyInterpolation::Bezier { .. } => {
                        // Smooth cubic hermite ease-in-out
                        let t2 = t * t * (3.0 - 2.0 * t);
                        k0.value + (k1.value - k0.value) * t2
                    }
                };
            }
        }
        self.static_value
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    pub anchor_point: AnimatedProperty<(f64, f64, f64)>,
    pub position: AnimatedProperty<(f64, f64, f64)>,
    pub scale: AnimatedProperty<(f64, f64, f64)>,
    pub rotation_x: AnimatedProperty<f64>,
    pub rotation_y: AnimatedProperty<f64>,
    pub rotation_z: AnimatedProperty<f64>,
    pub opacity: AnimatedProperty<f64>,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            anchor_point: AnimatedProperty::new((0.0, 0.0, 0.0)),
            position: AnimatedProperty::new((960.0, 540.0, 0.0)),
            scale: AnimatedProperty::new((100.0, 100.0, 100.0)),
            rotation_x: AnimatedProperty::new(0.0),
            rotation_y: AnimatedProperty::new(0.0),
            rotation_z: AnimatedProperty::new(0.0),
            opacity: AnimatedProperty::new(1.0),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VfxEffect {
    GaussianBlur { radius: f64 },
    DirectionalBlur { angle_deg: f64, length: f64 },
    Glow { threshold: f64, radius: f64, intensity: f64, color: Color },
    DropShadow { distance: f64, angle: f64, softness: f64, color: Color, opacity: f64 },
    ColorCorrection { brightness: f64, contrast: f64, saturation: f64 },
    ChromaKey { key_color: Color, tolerance: f64, softness: f64 },
    TurbulentDisplace { amount: f64, size: f64, evolution: f64 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompLayerKind {
    Solid,
    Footage,
    Text,
    Shape,
    Adjustment,
    Camera,
    Light,
    Null,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompLayer {
    pub id: Id,
    pub name: String,
    pub kind: CompLayerKind,
    pub parent_id: Option<Id>,
    pub in_frame: i64,
    pub out_frame: i64,
    pub is_3d: bool,
    pub blend_mode: BlendMode,
    pub track_matte: TrackMatte,
    pub matte_layer_id: Option<Id>,
    pub transform: Transform,
    pub effects: Vec<VfxEffect>,
    pub solid_color: Option<Color>,
    pub text_content: Option<String>,
    pub motion_blur: bool,
}

impl CompLayer {
    pub fn new(name: impl Into<String>, kind: CompLayerKind, in_frame: i64, out_frame: i64) -> Self {
        Self {
            id: Id::new(),
            name: name.into(),
            kind,
            parent_id: None,
            in_frame,
            out_frame,
            is_3d: false,
            blend_mode: BlendMode::Normal,
            track_matte: TrackMatte::None,
            matte_layer_id: None,
            transform: Transform::default(),
            effects: Vec::new(),
            solid_color: None,
            text_content: None,
            motion_blur: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Composition {
    pub id: Id,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub duration_frames: i64,
    pub bg_color: Color,
    pub layers: Vec<CompLayer>,
}

impl Composition {
    pub fn new(name: impl Into<String>, width: u32, height: u32, fps: f64, duration_frames: i64) -> Self {
        Self {
            id: Id::new(),
            name: name.into(),
            width,
            height,
            fps,
            duration_frames,
            bg_color: Color::BLACK,
            layers: Vec::new(),
        }
    }

    pub fn add_solid_layer(&mut self, name: impl Into<String>, color: Color) -> Id {
        let mut layer = CompLayer::new(name, CompLayerKind::Solid, 0, self.duration_frames);
        layer.solid_color = Some(color);
        let id = layer.id;
        self.layers.push(layer);
        id
    }

    pub fn add_text_layer(&mut self, name: impl Into<String>, text: impl Into<String>) -> Id {
        let mut layer = CompLayer::new(name, CompLayerKind::Text, 0, self.duration_frames);
        layer.text_content = Some(text.into());
        let id = layer.id;
        self.layers.push(layer);
        id
    }

    pub fn add_adjustment_layer(&mut self, name: impl Into<String>) -> Id {
        let layer = CompLayer::new(name, CompLayerKind::Adjustment, 0, self.duration_frames);
        let id = layer.id;
        self.layers.push(layer);
        id
    }

    pub fn add_null_layer(&mut self, name: impl Into<String>) -> Id {
        let layer = CompLayer::new(name, CompLayerKind::Null, 0, self.duration_frames);
        let id = layer.id;
        self.layers.push(layer);
        id
    }

    pub fn set_layer_parent(&mut self, layer_id: Id, parent_id: Option<Id>) -> bool {
        if let Some(layer) = self.layers.iter_mut().find(|l| l.id == layer_id) {
            layer.parent_id = parent_id;
            true
        } else {
            false
        }
    }
}
