use dreamcraft_core::{Color, Id, Rect};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Layer {
    pub id: Id,
    pub name: String,
    pub visible: bool,
    pub opacity: f32,
    pub blend_mode: BlendMode,
    pub bounds: Rect,
    pub fill_color: Option<Color>,
    pub image_path: Option<String>,
}

impl Layer {
    pub fn new(name: impl Into<String>, bounds: Rect) -> Self {
        Self {
            id: Id::new(),
            name: name.into(),
            visible: true,
            opacity: 1.0,
            blend_mode: BlendMode::Normal,
            bounds,
            fill_color: None,
            image_path: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Adjustments {
    pub brightness: f32, // -1.0 .. 1.0
    pub contrast: f32,   // -1.0 .. 1.0
    pub saturation: f32, // -1.0 .. 1.0
    pub invert: bool,
    pub blur_radius: f32,
}

impl Default for Adjustments {
    fn default() -> Self {
        Self {
            brightness: 0.0,
            contrast: 0.0,
            saturation: 0.0,
            invert: false,
            blur_radius: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhotoCanvas {
    pub id: Id,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub layers: Vec<Layer>,
    pub adjustments: Adjustments,
}

impl PhotoCanvas {
    pub fn new(name: impl Into<String>, width: u32, height: u32) -> Self {
        let mut base_layer = Layer::new("Background", Rect::new(0.0, 0.0, width as f64, height as f64));
        base_layer.fill_color = Some(Color::WHITE);
        Self {
            id: Id::new(),
            name: name.into(),
            width,
            height,
            layers: vec![base_layer],
            adjustments: Adjustments::default(),
        }
    }

    pub fn add_color_layer(&mut self, name: impl Into<String>, color: Color, bounds: Rect) -> Id {
        let mut layer = Layer::new(name, bounds);
        layer.fill_color = Some(color);
        let id = layer.id;
        self.layers.push(layer);
        id
    }

    pub fn add_image_layer(&mut self, name: impl Into<String>, image_path: impl Into<String>, bounds: Rect) -> Id {
        let mut layer = Layer::new(name, bounds);
        layer.image_path = Some(image_path.into());
        let id = layer.id;
        self.layers.push(layer);
        id
    }

    pub fn set_adjustment_brightness(&mut self, val: f32) {
        self.adjustments.brightness = val.clamp(-1.0, 1.0);
    }

    pub fn set_adjustment_contrast(&mut self, val: f32) {
        self.adjustments.contrast = val.clamp(-1.0, 1.0);
    }
}
