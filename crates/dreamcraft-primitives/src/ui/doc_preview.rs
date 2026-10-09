#[cfg(feature = "ui")]
use egui::{Color32, RichText, ScrollArea, Ui, Vec2};

use crate::tool::word::{DocumentBlock, HeadingLevel, WordDocument};

pub struct DocPreview;

impl DocPreview {
    #[cfg(feature = "ui")]
    pub fn show(ui: &mut Ui, doc: &WordDocument) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.set_max_width(720.0);
            ui.add_space(20.0);

            for block in &doc.blocks {
                match block {
                    DocumentBlock::Paragraph(p) => {
                        let (size, bold, color) = match p.heading {
                            HeadingLevel::Title => (26.0, true, Color32::from_rgb(220, 220, 240)),
                            HeadingLevel::Heading1 => (22.0, true, Color32::from_rgb(180, 200, 240)),
                            HeadingLevel::Heading2 => (18.0, true, Color32::from_rgb(160, 180, 220)),
                            HeadingLevel::Heading3 => (15.0, true, Color32::from_rgb(140, 160, 200)),
                            HeadingLevel::Heading4 => (13.0, true, Color32::from_rgb(130, 150, 190)),
                            HeadingLevel::Bullet => (12.0, false, Color32::from_rgb(200, 200, 210)),
                            HeadingLevel::Numbered => (12.0, false, Color32::from_rgb(200, 200, 210)),
                            HeadingLevel::Quote => (12.0, false, Color32::from_rgb(160, 170, 180)),
                            HeadingLevel::Body => (12.0, false, Color32::from_rgb(210, 210, 220)),
                        };

                        ui.horizontal_wrapped(|ui| {
                            if matches!(p.heading, HeadingLevel::Bullet) {
                                ui.label(RichText::new("•").size(size).color(color));
                            } else if matches!(p.heading, HeadingLevel::Quote) {
                                ui.label(RichText::new("┃").size(size).color(Color32::from_rgb(100, 110, 130)));
                            }

                            for run in &p.runs {
                                let mut text = RichText::new(&run.text).size(size);
                                if run.props.bold || bold {
                                    text = text.strong();
                                }
                                if run.props.italic {
                                    text = text.italics();
                                }
                                if !matches!(run.props.underline, crate::tool::word::UnderlineStyle::None) {
                                    text = text.underline();
                                }
                                text = text.color(color);
                                ui.label(text);
                            }
                        });
                        ui.add_space(8.0);
                    }
                    DocumentBlock::Table(t) => {
                        ui.group(|ui| {
                            egui::Grid::new(t.id.0)
                                .striped(true)
                                .min_col_width(100.0)
                                .spacing(Vec2::new(12.0, 6.0))
                                .show(ui, |ui| {
                                    for (i, row) in t.rows.iter().enumerate() {
                                        for cell in row {
                                            let text = cell.plain_text();
                                            if i == 0 && t.has_header {
                                                ui.label(RichText::new(text).strong().color(Color32::from_rgb(180, 210, 255)));
                                            } else {
                                                ui.label(RichText::new(text).color(Color32::from_rgb(200, 200, 210)));
                                            }
                                        }
                                        ui.end_row();
                                    }
                                });
                        });
                        ui.add_space(12.0);
                    }
                    DocumentBlock::PageBreak => {
                        ui.add_space(8.0);
                        ui.separator();
                        ui.label(RichText::new("── Page Break ──").size(10.0).color(Color32::from_rgb(120, 130, 150)));
                        ui.separator();
                        ui.add_space(8.0);
                    }
                }
            }
        });
    }
}
