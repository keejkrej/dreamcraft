use dreamcraft_core::{Color, Id, Rect};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SlideLayout {
    #[default]
    TitleSlide,
    TitleAndContent,
    SectionHeader,
    TwoColumns,
    Comparison,
    KeyMetric,
    Blank,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DeckThemeKind {
    #[default]
    Harbor,    // Navy & Cyan
    Ember,     // Charcoal & Warm Amber
    Meadow,    // Forest Green & Sage
    Nocturne,  // Dark Cyberpunk Violet & Neon Blue
    Paper,     // Editorial Ivory & Charcoal
    Slate,     // Modern Monochrome
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeckTheme {
    pub kind: DeckThemeKind,
    pub name: String,
    pub bg_color: Color,
    pub surface_color: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub accent: Color,
    pub accent_subtle: Color,
    pub font_heading: String,
    pub font_body: String,
}

impl Default for DeckTheme {
    fn default() -> Self {
        Self::from_kind(DeckThemeKind::Harbor)
    }
}

impl DeckTheme {
    pub fn from_kind(kind: DeckThemeKind) -> Self {
        match kind {
            DeckThemeKind::Harbor => Self {
                kind,
                name: "Harbor".into(),
                bg_color: Color::rgb(245, 248, 252),
                surface_color: Color::rgb(255, 255, 255),
                text_primary: Color::rgb(15, 30, 60),
                text_secondary: Color::rgb(80, 100, 130),
                accent: Color::rgb(14, 116, 224),
                accent_subtle: Color::rgb(215, 235, 255),
                font_heading: "Inter".into(),
                font_body: "Inter".into(),
            },
            DeckThemeKind::Ember => Self {
                kind,
                name: "Ember".into(),
                bg_color: Color::rgb(26, 26, 29),
                surface_color: Color::rgb(38, 38, 43),
                text_primary: Color::rgb(245, 245, 247),
                text_secondary: Color::rgb(180, 180, 185),
                accent: Color::rgb(240, 110, 45),
                accent_subtle: Color::rgb(80, 45, 25),
                font_heading: "Syne".into(),
                font_body: "Inter".into(),
            },
            DeckThemeKind::Meadow => Self {
                kind,
                name: "Meadow".into(),
                bg_color: Color::rgb(246, 249, 246),
                surface_color: Color::rgb(255, 255, 255),
                text_primary: Color::rgb(20, 45, 30),
                text_secondary: Color::rgb(85, 115, 95),
                accent: Color::rgb(35, 145, 85),
                accent_subtle: Color::rgb(220, 245, 230),
                font_heading: "Plus Jakarta Sans".into(),
                font_body: "Plus Jakarta Sans".into(),
            },
            DeckThemeKind::Nocturne => Self {
                kind,
                name: "Nocturne".into(),
                bg_color: Color::rgb(15, 15, 24),
                surface_color: Color::rgb(28, 28, 42),
                text_primary: Color::rgb(240, 242, 255),
                text_secondary: Color::rgb(160, 165, 195),
                accent: Color::rgb(145, 90, 255),
                accent_subtle: Color::rgb(45, 30, 80),
                font_heading: "Cabinet Grotesk".into(),
                font_body: "Inter".into(),
            },
            DeckThemeKind::Paper => Self {
                kind,
                name: "Paper".into(),
                bg_color: Color::rgb(252, 250, 246),
                surface_color: Color::rgb(255, 255, 255),
                text_primary: Color::rgb(32, 32, 35),
                text_secondary: Color::rgb(105, 105, 112),
                accent: Color::rgb(180, 50, 40),
                accent_subtle: Color::rgb(248, 230, 228),
                font_heading: "Lora".into(),
                font_body: "Source Serif Pro".into(),
            },
            DeckThemeKind::Slate => Self {
                kind,
                name: "Slate".into(),
                bg_color: Color::rgb(245, 246, 248),
                surface_color: Color::rgb(255, 255, 255),
                text_primary: Color::rgb(15, 23, 42),
                text_secondary: Color::rgb(71, 85, 105),
                accent: Color::rgb(51, 65, 85),
                accent_subtle: Color::rgb(226, 232, 240),
                font_heading: "Inter".into(),
                font_body: "Inter".into(),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SlideTransition {
    #[default]
    None,
    Fade,
    PushRight,
    PushLeft,
    Wipe,
    Zoom,
    Dissolve,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ShapeKind {
    Rectangle,
    RoundedRect { radius: f64 },
    Circle,
    Ellipse,
    ArrowRight,
    ArrowDouble,
    Star,
    Diamond,
    Triangle,
    Callout,
    Line,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FillStyle {
    Solid(Color),
    LinearGradient {
        angle_deg: f32,
        stops: Vec<(f32, Color)>,
    },
    Transparent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SlideShape {
    pub id: Id,
    pub kind: ShapeKind,
    pub bounds: Rect,
    pub fill: FillStyle,
    pub fill_color: Color, // Backwards compatibility for preview
    pub stroke_color: Option<Color>,
    pub stroke_width: f64,
    pub corner_radius: f64,
    pub shadow: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SlideText {
    pub id: Id,
    pub text: String,
    pub bounds: Rect,
    pub font_size: f32,
    pub font_family: Option<String>,
    pub bold: bool,
    pub italic: bool,
    pub color: Color,
    pub is_bullet: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SlideImage {
    pub id: Id,
    pub source_path: String,
    pub bounds: Rect,
    pub alt_text: Option<String>,
    pub corner_radius: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Slide {
    pub id: Id,
    pub layout: SlideLayout,
    pub title: String,
    pub subtitle: Option<String>,
    pub background: Color,
    pub shapes: Vec<SlideShape>,
    pub text_boxes: Vec<SlideText>,
    pub images: Vec<SlideImage>,
    pub notes: String,
    pub transition: SlideTransition,
}

impl Slide {
    pub fn new(layout: SlideLayout, title: impl Into<String>) -> Self {
        Self {
            id: Id::new(),
            layout,
            title: title.into(),
            subtitle: None,
            background: Color::WHITE,
            shapes: Vec::new(),
            text_boxes: Vec::new(),
            images: Vec::new(),
            notes: String::new(),
            transition: SlideTransition::None,
        }
    }

    pub fn with_theme(mut self, theme: &DeckTheme) -> Self {
        self.background = theme.bg_color;
        self
    }

    pub fn add_bullet(&mut self, text: impl Into<String>, y_offset: f64) {
        self.text_boxes.push(SlideText {
            id: Id::new(),
            text: text.into(),
            bounds: Rect::new(80.0, 160.0 + y_offset, 800.0, 40.0),
            font_size: 20.0,
            font_family: None,
            bold: false,
            italic: false,
            color: Color::rgb(40, 40, 50),
            is_bullet: true,
        });
    }

    pub fn add_shape(&mut self, kind: ShapeKind, bounds: Rect, fill_color: Color) {
        self.shapes.push(SlideShape {
            id: Id::new(),
            kind,
            bounds,
            fill: FillStyle::Solid(fill_color),
            fill_color,
            stroke_color: None,
            stroke_width: 1.0,
            corner_radius: 8.0,
            shadow: false,
        });
    }

    pub fn add_image(&mut self, source_path: impl Into<String>, bounds: Rect) {
        self.images.push(SlideImage {
            id: Id::new(),
            source_path: source_path.into(),
            bounds,
            alt_text: None,
            corner_radius: 8.0,
        });
    }

    pub fn set_speaker_notes(&mut self, notes: impl Into<String>) {
        self.notes = notes.into();
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Presentation {
    pub id: Id,
    pub title: String,
    pub theme: DeckTheme,
    pub slides: Vec<Slide>,
    pub current_slide: usize,
    pub width: f64,
    pub height: f64,
}

impl Default for Presentation {
    fn default() -> Self {
        Self::new("Untitled Presentation")
    }
}

impl Presentation {
    pub fn new(title: impl Into<String>) -> Self {
        let t = title.into();
        let theme = DeckTheme::default();
        let mut first_slide = Slide::new(SlideLayout::TitleSlide, t.clone()).with_theme(&theme);
        first_slide.subtitle = Some("Generated by DreamCraft AI".into());

        Self {
            id: Id::new(),
            title: t,
            theme,
            slides: vec![first_slide],
            current_slide: 0,
            width: 1920.0,
            height: 1080.0,
        }
    }

    pub fn set_theme(&mut self, kind: DeckThemeKind) {
        self.theme = DeckTheme::from_kind(kind);
        for slide in &mut self.slides {
            slide.background = self.theme.bg_color;
        }
    }

    pub fn add_slide(&mut self, layout: SlideLayout, title: impl Into<String>) -> usize {
        let mut slide = Slide::new(layout, title);
        slide.background = self.theme.bg_color;
        self.slides.push(slide);
        self.slides.len() - 1
    }

    pub fn current_slide_mut(&mut self) -> Option<&mut Slide> {
        self.slides.get_mut(self.current_slide)
    }

    pub fn select_slide(&mut self, index: usize) -> bool {
        if index < self.slides.len() {
            self.current_slide = index;
            true
        } else {
            false
        }
    }

    /// Helper to compose a high-impact Key Metric slide
    pub fn add_metric_slide(
        &mut self,
        title: impl Into<String>,
        metric_value: impl Into<String>,
        metric_label: impl Into<String>,
        description: impl Into<String>,
    ) -> usize {
        let mut slide = Slide::new(SlideLayout::KeyMetric, title).with_theme(&self.theme);

        // Huge metric text box
        slide.text_boxes.push(SlideText {
            id: Id::new(),
            text: metric_value.into(),
            bounds: Rect::new(120.0, 320.0, 800.0, 160.0),
            font_size: 96.0,
            font_family: Some(self.theme.font_heading.clone()),
            bold: true,
            italic: false,
            color: self.theme.accent,
            is_bullet: false,
        });

        // Metric subtitle
        slide.text_boxes.push(SlideText {
            id: Id::new(),
            text: metric_label.into(),
            bounds: Rect::new(125.0, 500.0, 800.0, 50.0),
            font_size: 24.0,
            font_family: Some(self.theme.font_heading.clone()),
            bold: true,
            italic: false,
            color: self.theme.text_primary,
            is_bullet: false,
        });

        // Supporting description
        slide.text_boxes.push(SlideText {
            id: Id::new(),
            text: description.into(),
            bounds: Rect::new(125.0, 560.0, 1000.0, 80.0),
            font_size: 18.0,
            font_family: Some(self.theme.font_body.clone()),
            bold: false,
            italic: false,
            color: self.theme.text_secondary,
            is_bullet: false,
        });

        self.slides.push(slide);
        self.slides.len() - 1
    }

    /// Export slide deck outline to structured markdown
    pub fn to_markdown(&self) -> String {
        let mut md = format!("# {}\n\n", self.title);
        for (idx, slide) in self.slides.iter().enumerate() {
            md.push_str(&format!("## Slide {}: {}\n\n", idx + 1, slide.title));
            if let Some(sub) = &slide.subtitle {
                md.push_str(&format!("*{ }*\n\n", sub));
            }
            for tb in &slide.text_boxes {
                if tb.is_bullet {
                    md.push_str(&format!("- {}\n", tb.text));
                } else {
                    md.push_str(&format!("{}\n\n", tb.text));
                }
            }
            if !slide.notes.is_empty() {
                md.push_str(&format!("\n> **Speaker Notes:** {}\n", slide.notes));
            }
            md.push_str("\n---\n\n");
        }
        md
    }
}
