use dreamcraft_core::{Color, Id, Rect};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BlendMode {
    #[default]
    Normal,
    Dissolve,
    Darken,
    Multiply,
    ColorBurn,
    LinearBurn,
    Lighten,
    Screen,
    ColorDodge,
    LinearDodge,
    Overlay,
    SoftLight,
    HardLight,
    VividLight,
    LinearLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayerEffects {
    pub drop_shadow: Option<DropShadowEffect>,
    pub inner_shadow: Option<InnerShadowEffect>,
    pub stroke: Option<StrokeEffect>,
    pub color_overlay: Option<ColorOverlayEffect>,
    pub outer_glow: Option<OuterGlowEffect>,
}

impl Default for LayerEffects {
    fn default() -> Self {
        Self {
            drop_shadow: None,
            inner_shadow: None,
            stroke: None,
            color_overlay: None,
            outer_glow: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DropShadowEffect {
    pub color: Color,
    pub opacity: f32,
    pub angle_deg: f32,
    pub distance: f32,
    pub spread: f32,
    pub size: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InnerShadowEffect {
    pub color: Color,
    pub opacity: f32,
    pub angle_deg: f32,
    pub distance: f32,
    pub size: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StrokeEffect {
    pub color: Color,
    pub size: f32,
    pub position: String, // "outside", "inside", "center"
    pub opacity: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorOverlayEffect {
    pub color: Color,
    pub opacity: f32,
    pub blend_mode: BlendMode,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OuterGlowEffect {
    pub color: Color,
    pub opacity: f32,
    pub size: f32,
    pub spread: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AdjustmentKind {
    Levels {
        black_in: u8,
        white_in: u8,
        gamma: f32,
        black_out: u8,
        white_out: u8,
    },
    Curves {
        points: Vec<(f32, f32)>,
    },
    HueSaturation {
        hue: f32,        // -180..+180
        saturation: f32, // -100..+100
        lightness: f32,  // -100..+100
    },
    BrightnessContrast {
        brightness: f32, // -100..+100
        contrast: f32,   // -100..+100
    },
    BlackAndWhite,
    Invert,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayerMask {
    pub enabled: bool,
    pub inverted: bool,
    pub density: f32, // 0.0 .. 1.0
    pub feather: f32, // px
}

impl Default for LayerMask {
    fn default() -> Self {
        Self {
            enabled: true,
            inverted: false,
            density: 1.0,
            feather: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum LayerKind {
    #[default]
    Raster,
    Adjustment,
    Group,
    Text,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Layer {
    pub id: Id,
    pub name: String,
    pub kind: LayerKind,
    pub visible: bool,
    pub locked: bool,
    pub opacity: f32,      // 0.0 .. 1.0
    pub fill_opacity: f32, // 0.0 .. 1.0
    pub blend_mode: BlendMode,
    pub bounds: Rect,
    pub fill_color: Option<Color>,
    pub image_path: Option<String>,
    pub mask: Option<LayerMask>,
    pub effects: LayerEffects,
    pub adjustment: Option<AdjustmentKind>,
}

impl Layer {
    pub fn new(name: impl Into<String>, bounds: Rect) -> Self {
        Self {
            id: Id::new(),
            name: name.into(),
            kind: LayerKind::Raster,
            visible: true,
            locked: false,
            opacity: 1.0,
            fill_opacity: 1.0,
            blend_mode: BlendMode::Normal,
            bounds,
            fill_color: None,
            image_path: None,
            mask: None,
            effects: LayerEffects::default(),
            adjustment: None,
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
    pub dpi: f32,
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
            dpi: 300.0,
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

    pub fn add_adjustment_layer(&mut self, name: impl Into<String>, adj: AdjustmentKind) -> Id {
        let bounds = Rect::new(0.0, 0.0, self.width as f64, self.height as f64);
        let mut layer = Layer::new(name, bounds);
        layer.kind = LayerKind::Adjustment;
        layer.adjustment = Some(adj);
        let id = layer.id;
        self.layers.push(layer);
        id
    }

    pub fn add_drop_shadow(&mut self, layer_id: Id, distance: f32, size: f32, opacity: f32) -> bool {
        if let Some(layer) = self.layers.iter_mut().find(|l| l.id == layer_id) {
            layer.effects.drop_shadow = Some(DropShadowEffect {
                color: Color::BLACK,
                opacity,
                angle_deg: 90.0,
                distance,
                spread: 0.0,
                size,
            });
            true
        } else {
            false
        }
    }

    pub fn set_adjustment_brightness(&mut self, val: f32) {
        self.adjustments.brightness = val.clamp(-1.0, 1.0);
    }

    pub fn set_adjustment_contrast(&mut self, val: f32) {
        self.adjustments.contrast = val.clamp(-1.0, 1.0);
    }
}
