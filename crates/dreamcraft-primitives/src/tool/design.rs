use dreamcraft_core::{Color, Id, Rect};
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
pub struct BleedAndSlug {
    pub bleed_top: f64,
    pub bleed_bottom: f64,
    pub bleed_inside: f64,
    pub bleed_outside: f64,
    pub slug_bottom: f64,
}

impl Default for BleedAndSlug {
    fn default() -> Self {
        Self {
            bleed_top: 9.0,    // 0.125 in (9 pt)
            bleed_bottom: 9.0,
            bleed_inside: 9.0,
            bleed_outside: 9.0,
            slug_bottom: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TextWrapMode {
    None,
    BoundingBox {
        top: f64,
        bottom: f64,
        left: f64,
        right: f64,
    },
    Contour {
        offset: f64,
    },
    JumpObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParagraphStyle {
    pub id: Id,
    pub name: String,
    pub font_family: String,
    pub font_size: f64,
    pub leading: f64,
    pub align: String, // "left", "center", "right", "justify"
    pub space_before: f64,
    pub space_after: f64,
    pub first_line_indent: f64,
    pub drop_cap_lines: u32,
    pub color: Color,
}

impl Default for ParagraphStyle {
    fn default() -> Self {
        Self {
            id: Id::new(),
            name: "Body Text".into(),
            font_family: "Minion Pro".into(),
            font_size: 10.0,
            leading: 13.0,
            align: "justify".into(),
            space_before: 0.0,
            space_after: 4.0,
            first_line_indent: 12.0,
            drop_cap_lines: 0,
            color: Color::BLACK,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextFrame {
    pub id: Id,
    pub bounds: Rect,
    pub columns: u32,
    pub column_gutter: f64,
    pub story_id: Id,
    pub prev_frame_id: Option<Id>,
    pub next_frame_id: Option<Id>,
    pub content: String,
    pub paragraph_style_id: Option<Id>,
    pub overflow: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphicFrame {
    pub id: Id,
    pub bounds: Rect,
    pub image_path: String,
    pub fit: String, // "proportional", "fill", "center"
    pub text_wrap: TextWrapMode,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MasterPage {
    pub id: Id,
    pub prefix: String, // "A"
    pub name: String,   // "A-Master"
    pub header_text: String,
    pub footer_text: String,
    pub show_page_numbers: bool,
}

impl Default for MasterPage {
    fn default() -> Self {
        Self {
            id: Id::new(),
            prefix: "A".into(),
            name: "A-Master".into(),
            header_text: "".into(),
            footer_text: "".into(),
            show_page_numbers: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DesignPage {
    pub page_number: usize,
    pub width: f64,
    pub height: f64,
    pub margins: Margins,
    pub columns: u32,
    pub gutter: f64,
    pub master_page_id: Option<Id>,
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
            master_page_id: None,
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
    pub bleed: BleedAndSlug,
    pub master_pages: Vec<MasterPage>,
    pub paragraph_styles: Vec<ParagraphStyle>,
    pub pages: Vec<DesignPage>,
}

impl DesignDocument {
    pub fn new(title: impl Into<String>) -> Self {
        let master = MasterPage::default();
        let body_style = ParagraphStyle::default();

        let mut doc = Self {
            id: Id::new(),
            title: title.into(),
            facing_pages: true,
            bleed: BleedAndSlug::default(),
            master_pages: vec![master],
            paragraph_styles: vec![body_style],
            pages: Vec::new(),
        };
        doc.add_page(612.0, 792.0); // Standard US Letter in points
        doc
    }

    pub fn add_page(&mut self, width: f64, height: f64) -> usize {
        let pnum = self.pages.len() + 1;
        let mut page = DesignPage::new(pnum, width, height);
        if let Some(first_master) = self.master_pages.first() {
            page.master_page_id = Some(first_master.id);
        }
        self.pages.push(page);
        pnum - 1
    }

    pub fn add_text_frame(&mut self, page_idx: usize, bounds: Rect, text: impl Into<String>) -> Option<Id> {
        let page = self.pages.get_mut(page_idx)?;
        let frame = TextFrame {
            id: Id::new(),
            bounds,
            columns: page.columns,
            column_gutter: page.gutter,
            story_id: Id::new(),
            prev_frame_id: None,
            next_frame_id: None,
            content: text.into(),
            paragraph_style_id: self.paragraph_styles.first().map(|s| s.id),
            overflow: false,
        };
        let id = frame.id;
        page.text_frames.push(frame);
        Some(id)
    }

    /// Thread two text frames together into a single story
    pub fn thread_frames(&mut self, from_frame_id: Id, to_frame_id: Id) -> bool {
        let mut story_to_assign = None;

        for page in &mut self.pages {
            if let Some(from_frame) = page.text_frames.iter_mut().find(|f| f.id == from_frame_id) {
                from_frame.next_frame_id = Some(to_frame_id);
                story_to_assign = Some(from_frame.story_id);
            }
        }

        if let Some(story_id) = story_to_assign {
            for page in &mut self.pages {
                if let Some(to_frame) = page.text_frames.iter_mut().find(|f| f.id == to_frame_id) {
                    to_frame.prev_frame_id = Some(from_frame_id);
                    to_frame.story_id = story_id;
                    return true;
                }
            }
        }
        false
    }

    pub fn add_graphic_frame(&mut self, page_idx: usize, bounds: Rect, image_path: impl Into<String>) -> Option<Id> {
        let page = self.pages.get_mut(page_idx)?;
        let frame = GraphicFrame {
            id: Id::new(),
            bounds,
            image_path: image_path.into(),
            fit: "proportional".into(),
            text_wrap: TextWrapMode::BoundingBox { top: 6.0, bottom: 6.0, left: 6.0, right: 6.0 },
        };
        let id = frame.id;
        page.graphic_frames.push(frame);
        Some(id)
    }
}
