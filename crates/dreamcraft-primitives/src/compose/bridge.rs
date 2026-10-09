use dreamcraft_core::types::Rect;
use dreamcraft_core::{AssetCatalog, AssetEntry, AssetKind, Color, Id, Result, Tick};
use crate::tool::cad::CadDrawing;
use crate::tool::deck::{ShapeKind, Slide, SlideText};
use crate::tool::design::{DesignDocument, DesignPage, MasterPage, TextFrame};
use crate::tool::film::{Sequence, TrackKind};
use crate::tool::grid::{CellCoord, Workbook};
use crate::tool::sound::{AudioTrack, SoundProject};
use crate::tool::vector::VectorDocument;
use crate::tool::word::{DocumentBlock, TableBlock, TableCell, TextRun, WordDocument};

/// MediaBridge facilitates lossless asset passing between Vector, Photo, CAD, Design, Deck, and Film engines.
pub struct MediaBridge;

impl MediaBridge {
    /// Export a VectorDocument to an SVG asset and register it in the shared AssetCatalog.
    pub fn vector_to_asset(
        catalog: &mut AssetCatalog,
        vector_doc: &VectorDocument,
        name: &str,
    ) -> Result<AssetEntry> {
        let svg_data = vector_doc.to_svg().into_bytes();
        let entry = catalog.register_bytes(
            name,
            AssetKind::Vector,
            "image/svg+xml",
            svg_data,
        );
        Ok(entry)
    }

    /// Export a CadDrawing to a DXF asset and register it in the shared AssetCatalog.
    pub fn cad_to_asset(
        catalog: &mut AssetCatalog,
        cad_doc: &CadDrawing,
        name: &str,
    ) -> Result<AssetEntry> {
        let dxf_data = cad_doc.to_dxf().into_bytes();
        let entry = catalog.register_bytes(
            name,
            AssetKind::Vector,
            "application/dxf",
            dxf_data,
        );
        Ok(entry)
    }

    /// Embed an SVG vector asset into a Deck presentation slide as a shape/graphic box.
    pub fn embed_svg_in_slide(
        slide: &mut Slide,
        asset_id: Id,
        label: &str,
        bounds: Rect,
    ) {
        slide.add_shape(
            ShapeKind::Rectangle,
            bounds,
            Color::rgba(45, 120, 240, 40),
        );
        slide.text_boxes.push(SlideText {
            id: Id::new(),
            text: format!("[Asset: {} | ID: {}]", label, asset_id),
            bounds: Rect::new(bounds.x + 10.0, bounds.y + bounds.height - 24.0, bounds.width - 20.0, 20.0),
            font_size: 12.0,
            font_family: None,
            bold: false,
            italic: false,
            color: Color::rgb(100, 160, 240),
            is_bullet: false,
        });
    }

    /// Embed an asset into a DesignCraft Master Page.
    pub fn embed_in_master_page(
        master: &mut MasterPage,
        asset_id: Id,
        label: &str,
    ) {
        master.header_text = format!("{} | Asset: {}", label, asset_id);
    }
}

/// AudioVideoBridge links FilmCraft NLE timeline tracks with SoundCraft DAW processing.
pub struct AudioVideoBridge;

impl AudioVideoBridge {
    /// Populate a SoundCraft project with matching audio stems from a FilmCraft sequence.
    pub fn sync_film_audio_to_soundcraft(
        film: &Sequence,
        sound: &mut SoundProject,
    ) {
        for track in &film.audio_tracks {
            let mut sound_track = AudioTrack::new(format!("Film {}", track.name));
            sound_track.volume_db = track.volume_db;
            sound.tracks.push(sound_track);
        }
    }

    /// Create synchronized audio track in FilmCraft matching SoundCraft master duration.
    pub fn attach_sound_mix_to_film(
        sound: &SoundProject,
        film: &mut Sequence,
        track_name: &str,
        duration_seconds: f64,
    ) -> Result<Id> {
        let track_id = film.add_track(TrackKind::Audio, track_name);
        let duration = Tick::from_seconds(duration_seconds);
        film.insert_clip(
            track_id,
            format!("{} Master Mix", sound.name),
            "master_mix.wav",
            Tick::ZERO,
            duration,
        )?;
        Ok(track_id)
    }
}

/// EditorialBridge maps narrative manuscript paragraphs into DesignCraft multi-column pages with threaded text.
pub struct EditorialBridge;

impl EditorialBridge {
    /// Flow WordDocument blocks across threaded columns in DesignCraft pages.
    pub fn flow_word_to_design(
        doc: &WordDocument,
        layout: &mut DesignDocument,
        chars_per_column: usize,
    ) {
        let mut full_text = String::new();
        for block in &doc.blocks {
            if let DocumentBlock::Paragraph(p) = block {
                for run in &p.runs {
                    full_text.push_str(&run.text);
                }
                full_text.push_str("\n\n");
            }
        }

        let chunks: Vec<&str> = full_text
            .as_bytes()
            .chunks(chars_per_column.max(100))
            .filter_map(|c| std::str::from_utf8(c).ok())
            .collect();

        let page_width = 612.0;
        let page_height = 792.0;
        let col_width = (page_width - 72.0 * 2.0 - 18.0) / 2.0;

        let story_id = Id::new();
        let mut prev_id: Option<Id> = None;
        let mut chunk_idx = 0;

        let num_pages = ((chunks.len() as f64) / 2.0).ceil() as usize;
        layout.pages.clear();

        for p_idx in 0..num_pages {
            let mut page = DesignPage::new(p_idx + 1, page_width, page_height);

            for col in 0..2 {
                if chunk_idx < chunks.len() {
                    let frame_id = Id::new();
                    let x = 72.0 + (col as f64) * (col_width + 18.0);
                    let y = 72.0;

                    let frame = TextFrame {
                        id: frame_id,
                        bounds: Rect::new(x, y, col_width, page_height - 144.0),
                        columns: 1,
                        column_gutter: 0.0,
                        story_id,
                        prev_frame_id: prev_id,
                        next_frame_id: None,
                        content: chunks[chunk_idx].to_string(),
                        paragraph_style_id: None,
                        overflow: false,
                    };

                    if let Some(prev) = prev_id {
                        // Link previous frame's next pointer
                        for p in &mut layout.pages {
                            for f in &mut p.text_frames {
                                if f.id == prev {
                                    f.next_frame_id = Some(frame_id);
                                }
                            }
                        }
                    }

                    prev_id = Some(frame_id);
                    page.text_frames.push(frame);
                    chunk_idx += 1;
                }
            }

            layout.pages.push(page);
        }
    }
}

/// FinancialBridge extracts computed numbers from GridCraft sheets into Word and Deck summaries.
pub struct FinancialBridge;

impl FinancialBridge {
    /// Format a grid cell matrix into a WordDocument TableBlock.
    pub fn grid_range_to_word_table(
        workbook: &Workbook,
        sheet_name: &str,
        start_row: usize,
        start_col: usize,
        end_row: usize,
        end_col: usize,
    ) -> TableBlock {
        let sheet = workbook.get_sheet_by_name(sheet_name).unwrap_or_else(|| workbook.active_sheet());
        let num_rows = end_row.saturating_sub(start_row) + 1;
        let num_cols = end_col.saturating_sub(start_col) + 1;
        let mut table = TableBlock::new(num_rows, num_cols);

        for (r_idx, r) in (start_row..=end_row).enumerate() {
            for (c_idx, c) in (start_col..=end_col).enumerate() {
                let cell = sheet.get_cell(&CellCoord::new(c as u32, r as u32));
                let val_str = cell.format.format_value(&cell.computed);
                let is_header = r_idx == 0;
                
                table.rows[r_idx][c_idx] = if is_header {
                    TableCell {
                        runs: vec![TextRun::new(val_str).bold()],
                        background: None,
                        col_span: 1,
                        row_span: 1,
                    }
                } else {
                    TableCell::new(val_str)
                };
            }
        }

        table
    }
}
