use dreamcraft_core::{DreamError, Id, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PdfPage {
    pub page_number: usize,
    pub width: f64,
    pub height: f64,
    pub text_blocks: Vec<String>,
    pub annotations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PdfDocument {
    pub id: Id,
    pub title: String,
    pub pages: Vec<PdfPage>,
}

impl PdfDocument {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            id: Id::new(),
            title: title.into(),
            pages: Vec::new(),
        }
    }

    pub fn add_page(&mut self, width: f64, height: f64) -> usize {
        let page_num = self.pages.len() + 1;
        self.pages.push(PdfPage {
            page_number: page_num,
            width,
            height,
            text_blocks: Vec::new(),
            annotations: Vec::new(),
        });
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
