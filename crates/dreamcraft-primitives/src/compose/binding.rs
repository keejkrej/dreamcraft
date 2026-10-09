use dreamcraft_core::{Id, Result};
use crate::tool::deck::Presentation;
use crate::tool::grid::{CellCoord, Workbook};
use crate::tool::word::WordDocument;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BindingSource {
    GridCell {
        sheet_name: String,
        coord: CellCoord,
    },
    GridFormula {
        formula: String,
    },
    StaticText {
        text: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BindingTarget {
    DeckSlideMetric {
        slide_index: usize,
        metric_box_index: usize,
    },
    WordDocTableCell {
        table_index: usize,
        row: usize,
        col: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiveBinding {
    pub id: Id,
    pub name: String,
    pub source: BindingSource,
    pub target: BindingTarget,
}

impl LiveBinding {
    pub fn new(name: impl Into<String>, source: BindingSource, target: BindingTarget) -> Self {
        Self {
            id: Id::new(),
            name: name.into(),
            source,
            target,
        }
    }
}

pub struct BindingEngine {
    pub bindings: Vec<LiveBinding>,
}

impl Default for BindingEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl BindingEngine {
    pub fn new() -> Self {
        Self {
            bindings: Vec::new(),
        }
    }

    pub fn add_binding(&mut self, binding: LiveBinding) -> Id {
        let id = binding.id;
        self.bindings.push(binding);
        id
    }

    /// Synchronize all bound data between the spreadsheet workbook and presentations/documents.
    pub fn sync_all(
        &self,
        workbook: &mut Workbook,
        deck: &mut Presentation,
        doc: &mut WordDocument,
    ) -> Result<usize> {
        workbook.recalculate_all();
        let mut synced_count = 0;

        for binding in &self.bindings {
            let val_str = match &binding.source {
                BindingSource::GridCell { sheet_name, coord } => {
                    let sheet = workbook.get_sheet_by_name(sheet_name).unwrap_or_else(|| workbook.active_sheet());
                    sheet.get_cell(coord).format.format_value(&sheet.get_cell(coord).computed)
                }
                BindingSource::GridFormula { formula } => {
                    let sheet = workbook.active_sheet();
                    let computed = sheet.eval_formula_str(formula);
                    computed.display_string()
                }
                BindingSource::StaticText { text } => text.clone(),
            };

            match &binding.target {
                BindingTarget::DeckSlideMetric { slide_index, metric_box_index } => {
                    if let Some(slide) = deck.slides.get_mut(*slide_index) {
                        if let Some(tb) = slide.text_boxes.get_mut(*metric_box_index) {
                            tb.text = val_str;
                            synced_count += 1;
                        }
                    }
                }
                BindingTarget::WordDocTableCell { table_index, row, col } => {
                    let mut table_counter = 0;
                    for block in &mut doc.blocks {
                        if let crate::tool::word::DocumentBlock::Table(t) = block {
                            if table_counter == *table_index {
                                if let Some(r) = t.rows.get_mut(*row) {
                                    if let Some(c) = r.get_mut(*col) {
                                        c.runs = vec![crate::tool::word::TextRun::new(&val_str)];
                                        synced_count += 1;
                                    }
                                }
                                break;
                            }
                            table_counter += 1;
                        }
                    }
                }
            }
        }

        Ok(synced_count)
    }
}
