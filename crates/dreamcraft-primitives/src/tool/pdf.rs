use dreamcraft_core::{Color, DreamError, Id, Rect, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PdfAnnotation {
    Highlight {
        bounds: Rect,
        color: Color,
    },
    Underline {
        bounds: Rect,
        color: Color,
    },
    StickyNote {
        x: f64,
        y: f64,
        author: String,
        content: String,
    },
    FreehandInk {
        points: Vec<(f64, f64)>,
        stroke_width: f64,
        color: Color,
    },
    Stamp {
        title: String, // e.g. "APPROVED", "CONFIDENTIAL", "DRAFT"
        bounds: Rect,
        color: Color,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FormFieldKind {
    Text {
        value: String,
        multiline: bool,
    },
    Checkbox {
        checked: bool,
    },
    Dropdown {
        options: Vec<String>,
        selected: usize,
    },
    Signature {
        signed: bool,
        signer_name: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PdfFormField {
    pub id: Id,
    pub name: String,
    pub bounds: Rect,
    pub required: bool,
    pub kind: FormFieldKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Watermark {
    pub text: String,
    pub font_size: f64,
    pub opacity: f32,
    pub rotation_deg: f64,
    pub color: Color,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PdfPage {
    pub page_number: usize,
    pub width: f64,
    pub height: f64,
    pub rotation_deg: u16, // 0, 90, 180, 270
    pub text_blocks: Vec<String>,
    pub annotations: Vec<PdfAnnotation>,
    pub form_fields: Vec<PdfFormField>,
}

impl PdfPage {
    pub fn new(page_number: usize, width: f64, height: f64) -> Self {
        Self {
            page_number,
            width,
            height,
            rotation_deg: 0,
            text_blocks: Vec::new(),
            annotations: Vec::new(),
            form_fields: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PdfDocument {
    pub id: Id,
    pub title: String,
    pub author: Option<String>,
    pub watermark: Option<Watermark>,
    pub pages: Vec<PdfPage>,
}

impl PdfDocument {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            id: Id::new(),
            title: title.into(),
            author: None,
            watermark: None,
            pages: Vec::new(),
        }
    }

    pub fn add_page(&mut self, width: f64, height: f64) -> usize {
        let page_num = self.pages.len() + 1;
        self.pages.push(PdfPage::new(page_num, width, height));
        page_num
    }

    pub fn add_text_to_page(&mut self, page_idx: usize, text: impl Into<String>) -> Result<()> {
        let page = self
            .pages
            .get_mut(page_idx)
            .ok_or_else(|| DreamError::Document(format!("Page {} not found", page_idx + 1)))?;
        page.text_blocks.push(text.into());
        Ok(())
    }

    pub fn rotate_page(&mut self, page_idx: usize, degrees: u16) -> Result<()> {
        let page = self
            .pages
            .get_mut(page_idx)
            .ok_or_else(|| DreamError::Document(format!("Page {} not found", page_idx + 1)))?;
        page.rotation_deg = (page.rotation_deg + degrees) % 360;
        Ok(())
    }

    pub fn delete_page(&mut self, page_idx: usize) -> Result<()> {
        if page_idx < self.pages.len() {
            self.pages.remove(page_idx);
            for (idx, p) in self.pages.iter_mut().enumerate() {
                p.page_number = idx + 1;
            }
            Ok(())
        } else {
            Err(DreamError::Document(format!("Page {} out of bounds", page_idx + 1)))
        }
    }

    pub fn add_annotation(&mut self, page_idx: usize, annot: PdfAnnotation) -> Result<()> {
        let page = self
            .pages
            .get_mut(page_idx)
            .ok_or_else(|| DreamError::Document(format!("Page {} not found", page_idx + 1)))?;
        page.annotations.push(annot);
        Ok(())
    }

    pub fn add_form_field(&mut self, page_idx: usize, name: impl Into<String>, bounds: Rect, kind: FormFieldKind) -> Result<Id> {
        let page = self
            .pages
            .get_mut(page_idx)
            .ok_or_else(|| DreamError::Document(format!("Page {} not found", page_idx + 1)))?;

        let field = PdfFormField {
            id: Id::new(),
            name: name.into(),
            bounds,
            required: false,
            kind,
        };
        let id = field.id;
        page.form_fields.push(field);
        Ok(id)
    }

    pub fn set_watermark(&mut self, text: impl Into<String>) {
        self.watermark = Some(Watermark {
            text: text.into(),
            font_size: 48.0,
            opacity: 0.15,
            rotation_deg: -45.0,
            color: Color::rgb(180, 0, 0),
        });
    }

    pub fn merge(&mut self, other: PdfDocument) {
        for mut page in other.pages {
            page.page_number = self.pages.len() + 1;
            self.pages.push(page);
        }
    }

    pub fn extract_all_text(&self) -> String {
        let mut out = String::new();
        for page in &self.pages {
            out.push_str(&format!("--- Page {} ---\n", page.page_number));
            for block in &page.text_blocks {
                out.push_str(block);
                out.push('\n');
            }
            out.push('\n');
        }
        out
    }
}
