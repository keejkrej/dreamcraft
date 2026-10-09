use dreamcraft_core::{Id, Rect};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Margins {
    pub top: f64,
    pub bottom: f64,
    pub inside: f64,
    pub outside: f64,
}

impl Default for Margins {
    fn default() -> Self {
        Self {
            top: 36.0,    // 0.5 in
            bottom: 36.0,
            inside: 48.0,
            outside: 36.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextFrame {
    pub id: Id,
    pub bounds: Rect,
    pub columns: u32,
    pub story_id: Id,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphicFrame {
    pub id: Id,
    pub bounds: Rect,
    pub image_path: String,
    pub fit: String, // "proportional", "fill", "center"
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DesignPage {
    pub page_number: usize,
    pub width: f64,
    pub height: f64,
    pub margins: Margins,
    pub columns: u32,
    pub gutter: f64,
    pub text_frames: Vec<TextFrame>,
    pub graphic_frames: Vec<GraphicFrame>,
}

impl DesignPage {
    pub fn new(page_number: usize, width: f64, height: f64) -> Self {
        Self {
            page_number,
            width,
            height,
            margins: Margins::default(),
            columns: 2,
            gutter: 12.0,
            text_frames: Vec::new(),
            graphic_frames: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DesignDocument {
    pub id: Id,
    pub title: String,
    pub facing_pages: bool,
    pub pages: Vec<DesignPage>,
}

impl DesignDocument {
    pub fn new(title: impl Into<String>) -> Self {
        let mut doc = Self {
            id: Id::new(),
            title: title.into(),
            facing_pages: true,
            pages: Vec::new(),
        };
        doc.add_page(612.0, 792.0); // Standard US Letter in points
        doc
    }

    pub fn add_page(&mut self, width: f64, height: f64) -> usize {
        let pnum = self.pages.len() + 1;
        self.pages.push(DesignPage::new(pnum, width, height));
        pnum - 1
    }

    pub fn add_text_frame(&mut self, page_idx: usize, bounds: Rect, text: impl Into<String>) -> Option<Id> {
        let page = self.pages.get_mut(page_idx)?;
        let frame = TextFrame {
            id: Id::new(),
            bounds,
            columns: 1,
            story_id: Id::new(),
            content: text.into(),
        };
        let id = frame.id;
        page.text_frames.push(frame);
        Some(id)
    }

    pub fn add_graphic_frame(&mut self, page_idx: usize, bounds: Rect, image_path: impl Into<String>) -> Option<Id> {
        let page = self.pages.get_mut(page_idx)?;
        let frame = GraphicFrame {
            id: Id::new(),
            bounds,
            image_path: image_path.into(),
            fit: "proportional".into(),
        };
        let id = frame.id;
        page.graphic_frames.push(frame);
        Some(id)
    }
}
