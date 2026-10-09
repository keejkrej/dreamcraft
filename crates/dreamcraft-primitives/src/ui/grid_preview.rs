#[cfg(feature = "ui")]
use egui::{Color32, RichText, ScrollArea, Ui, Vec2};

use crate::tool::grid::{CellCoord, Workbook};

pub struct GridPreview;

impl GridPreview {
    #[cfg(feature = "ui")]
    pub fn show(ui: &mut Ui, workbook: &Workbook) {
        let sheet = workbook.active_sheet();

        ui.horizontal(|ui| {
            ui.label(RichText::new(&format!("Workbook: {}", workbook.title)).strong().size(15.0));
            ui.separator();
            ui.label(RichText::new(&format!("Sheet: {}", sheet.name)).color(Color32::from_rgb(140, 200, 255)));
        });
        ui.add_space(8.0);

        ScrollArea::both().show(ui, |ui| {
            let max_c = sheet.max_col.max(8);
            let max_r = sheet.max_row.max(16);

            egui::Grid::new("sheet_preview_grid")
                .striped(true)
                .min_col_width(70.0)
                .spacing(Vec2::new(1.0, 1.0))
                .show(ui, |ui| {
                    // Header row
                    ui.label(RichText::new("#").weak());
                    for c in 0..max_c {
                        let col_name = CellCoord::new(c, 0).to_string_coord();
                        let col_letter: String = col_name.chars().take_while(|ch| ch.is_alphabetic()).collect();
                        ui.label(RichText::new(col_letter).strong().color(Color32::from_rgb(160, 170, 190)));
                    }
                    ui.end_row();

                    for r in 0..max_r {
                        ui.label(RichText::new(format!("{}", r + 1)).weak());
                        for c in 0..max_c {
                            let cell = sheet.get_cell(&CellCoord::new(c, r));
                            let val_str = cell.computed.display_string();
                            let is_empty = val_str.is_empty();

                            let mut text = RichText::new(if is_empty { " " } else { &val_str });
                            if cell.bold {
                                text = text.strong();
                            }
                            ui.label(text);
                        }
                        ui.end_row();
                    }
                });
        });
    }
}
