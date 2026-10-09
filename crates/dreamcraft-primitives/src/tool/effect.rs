use dreamcraft_core::{Color, Id};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyInterpolation {
    Linear,
    Bezier,
    Hold,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    pub position: AnimatedProperty<(f64, f64)>,
    pub scale: AnimatedProperty<(f64, f64)>,
    pub rotation: AnimatedProperty<f64>,
    pub opacity: AnimatedProperty<f64>,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: AnimatedProperty::new((960.0, 540.0)),
            scale: AnimatedProperty::new((100.0, 100.0)),
            rotation: AnimatedProperty::new(0.0),
            opacity: AnimatedProperty::new(1.0),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VfxEffect {
    GaussianBlur { radius: f64 },
    Glow { threshold: f64, radius: f64, intensity: f64 },
    DropShadow { distance: f64, angle: f64, softness: f64, color: Color },
    ColorCorrection { brightness: f64, contrast: f64, saturation: f64 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompLayerKind {
    Solid,
    Footage,
    Text,
    Shape,
    Adjustment,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompLayer {
    pub id: Id,
    pub name: String,
    pub kind: CompLayerKind,
    pub in_frame: i64,
    pub out_frame: i64,
    pub transform: Transform,
    pub effects: Vec<VfxEffect>,
    pub solid_color: Option<Color>,
    pub text_content: Option<String>,
}

impl CompLayer {
    pub fn new(name: impl Into<String>, kind: CompLayerKind, in_frame: i64, out_frame: i64) -> Self {
        Self {
            id: Id::new(),
            name: name.into(),
            kind,
            in_frame,
            out_frame,
            transform: Transform::default(),
            effects: Vec::new(),
            solid_color: None,
            text_content: None,
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
}
