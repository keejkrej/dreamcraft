use dreamcraft_core::{Color, Id};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeadingLevel {
    Title,
    Heading1,
    Heading2,
    Heading3,
    Heading4,
    Body,
    Bullet,
    Numbered,
    Quote,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextAlign {
    Left,
    Center,
    Right,
    Justify,
}

impl Default for TextAlign {
    fn default() -> Self {
        Self::Left
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextRun {
    pub text: String,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strike: bool,
    pub font_size: f32,
    pub color: Option<Color>,
    pub highlight: Option<Color>,
    pub link: Option<String>,
}

impl TextRun {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            bold: false,
            italic: false,
            underline: false,
            strike: false,
            font_size: 11.0,
            color: None,
            highlight: None,
            link: None,
        }
    }

    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableCell {
    pub runs: Vec<TextRun>,
    pub background: Option<Color>,
}

impl TableCell {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            runs: vec![TextRun::new(text)],
            background: None,
        }
    }

    pub fn plain_text(&self) -> String {
        self.runs.iter().map(|r| r.text.as_str()).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableBlock {
    pub id: Id,
    pub rows: Vec<Vec<TableCell>>,
    pub has_header: bool,
}

impl TableBlock {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut row_vec = Vec::with_capacity(rows);
        for _ in 0..rows {
            let mut col_vec = Vec::with_capacity(cols);
            for _ in 0..cols {
                col_vec.push(TableCell::new(""));
            }
            row_vec.push(col_vec);
        }
        Self {
            id: Id::new(),
            rows: row_vec,
            has_header: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParagraphBlock {
    pub id: Id,
    pub heading: HeadingLevel,
    pub align: TextAlign,
    pub runs: Vec<TextRun>,
}

impl ParagraphBlock {
    pub fn new(heading: HeadingLevel, text: impl Into<String>) -> Self {
        Self {
            id: Id::new(),
            heading,
            align: TextAlign::Left,
            runs: vec![TextRun::new(text)],
        }
    }

    pub fn plain_text(&self) -> String {
        self.runs.iter().map(|r| r.text.as_str()).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DocumentBlock {
    Paragraph(ParagraphBlock),
    Table(TableBlock),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WordDocument {
    pub id: Id,
    pub title: String,
    pub author: String,
    pub blocks: Vec<DocumentBlock>,
}

impl WordDocument {
    pub fn new(title: impl Into<String>) -> Self {
        let t = title.into();
        Self {
            id: Id::new(),
            title: t.clone(),
            author: "AI Agent".into(),
            blocks: vec![DocumentBlock::Paragraph(ParagraphBlock::new(
                HeadingLevel::Title,
                t,
            ))],
        }
    }

    pub fn add_heading(&mut self, level: HeadingLevel, text: impl Into<String>) {
        self.blocks.push(DocumentBlock::Paragraph(ParagraphBlock::new(level, text)));
    }

    pub fn add_paragraph(&mut self, text: impl Into<String>) {
        self.blocks.push(DocumentBlock::Paragraph(ParagraphBlock::new(
            HeadingLevel::Body,
            text,
        )));
    }

    pub fn add_formatted_paragraph(&mut self, runs: Vec<TextRun>) {
        self.blocks.push(DocumentBlock::Paragraph(ParagraphBlock {
            id: Id::new(),
            heading: HeadingLevel::Body,
            align: TextAlign::Left,
            runs,
        }));
    }

    pub fn insert_table(&mut self, data: Vec<Vec<String>>, has_header: bool) {
        let rows = data
            .into_iter()
            .map(|row| row.into_iter().map(TableCell::new).collect())
            .collect();
        self.blocks.push(DocumentBlock::Table(TableBlock {
            id: Id::new(),
            rows,
            has_header,
        }));
    }

    pub fn search_and_replace(&mut self, find: &str, replace_with: &str) -> usize {
        let mut count = 0;
        for block in &mut self.blocks {
            match block {
                DocumentBlock::Paragraph(p) => {
                    for run in &mut p.runs {
                        if run.text.contains(find) {
                            run.text = run.text.replace(find, replace_with);
                            count += 1;
                        }
                    }
                }
                DocumentBlock::Table(t) => {
                    for row in &mut t.rows {
                        for cell in row {
                            for run in &mut cell.runs {
                                if run.text.contains(find) {
                                    run.text = run.text.replace(find, replace_with);
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        count
    }

    pub fn to_plain_text(&self) -> String {
        let mut out = String::new();
        for block in &self.blocks {
            match block {
                DocumentBlock::Paragraph(p) => {
                    out.push_str(&p.plain_text());
                    out.push('\n');
                }
                DocumentBlock::Table(t) => {
                    for row in &t.rows {
                        let row_str: Vec<String> = row.iter().map(|c| c.plain_text()).collect();
                        out.push_str(&row_str.join("\t"));
                        out.push('\n');
                    }
                }
            }
            out.push('\n');
        }
        out
    }

    pub fn to_markdown(&self) -> String {
        let mut out = String::new();
        for block in &self.blocks {
            match block {
                DocumentBlock::Paragraph(p) => {
                    let prefix = match p.heading {
                        HeadingLevel::Title => "# ",
                        HeadingLevel::Heading1 => "# ",
                        HeadingLevel::Heading2 => "## ",
                        HeadingLevel::Heading3 => "### ",
                        HeadingLevel::Heading4 => "#### ",
                        HeadingLevel::Bullet => "- ",
                        HeadingLevel::Numbered => "1. ",
                        HeadingLevel::Quote => "> ",
                        HeadingLevel::Body => "",
                    };
                    out.push_str(prefix);
                    for run in &p.runs {
                        if run.bold && run.italic {
                            out.push_str(&format!("***{}***", run.text));
                        } else if run.bold {
                            out.push_str(&format!("**{}**", run.text));
                        } else if run.italic {
                            out.push_str(&format!("*{}*", run.text));
                        } else {
                            out.push_str(&run.text);
                        }
                    }
                    out.push_str("\n\n");
                }
                DocumentBlock::Table(t) => {
                    if t.rows.is_empty() {
                        continue;
                    }
                    let cols = t.rows.first().map(|r| r.len()).unwrap_or(0);
                    for (i, row) in t.rows.iter().enumerate() {
                        out.push_str("| ");
                        for cell in row {
                            out.push_str(&cell.plain_text());
                            out.push_str(" | ");
                        }
                        out.push('\n');
                        if i == 0 && t.has_header {
                            out.push_str("| ");
                            for _ in 0..cols {
                                out.push_str("--- | ");
                            }
                            out.push('\n');
                        }
                    }
                    out.push('\n');
                }
            }
        }
        out
    }
}
