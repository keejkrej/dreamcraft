use dreamcraft_core::{AssetEntry, AssetKind, DreamProject, EventBus, Id};
use dreamcraft_primitives::tool::{
    CadDrawing, Composition, DesignDocument, LightPhoto, PdfDocument, PhotoCanvas, Presentation,
    Sequence, SoundProject, VectorDocument, WordDocument, Workbook,
};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct DreamSession {
    // Unified Project Container & Cross-Domain Asset Catalog
    pub project: Arc<Mutex<DreamProject>>,

    // 12 Native Creative & Office Engines
    pub word_doc: Arc<Mutex<Option<WordDocument>>>,
    pub workbook: Arc<Mutex<Option<Workbook>>>,
    pub deck: Arc<Mutex<Option<Presentation>>>,
    pub film_seq: Arc<Mutex<Option<Sequence>>>,
    pub composition: Arc<Mutex<Option<Composition>>>,
    pub photo_canvas: Arc<Mutex<Option<PhotoCanvas>>>,
    pub vector_doc: Arc<Mutex<Option<VectorDocument>>>,
    pub light_photo: Arc<Mutex<Option<LightPhoto>>>,
    pub cad_drawing: Arc<Mutex<Option<CadDrawing>>>,
    pub design_doc: Arc<Mutex<Option<DesignDocument>>>,
    pub pdf_doc: Arc<Mutex<Option<PdfDocument>>>,
    pub sound_project: Arc<Mutex<Option<SoundProject>>>,

    pub event_bus: EventBus,
}

impl Default for DreamSession {
    fn default() -> Self {
        Self::new()
    }
}

impl DreamSession {
    pub fn new() -> Self {
        Self {
            project: Arc::new(Mutex::new(DreamProject::new("DreamCraft Workspace"))),
            word_doc: Arc::new(Mutex::new(Some(WordDocument::new("Untitled Document")))),
            workbook: Arc::new(Mutex::new(Some(Workbook::new("Untitled Workbook")))),
            deck: Arc::new(Mutex::new(Some(Presentation::new("Untitled Presentation")))),
            film_seq: Arc::new(Mutex::new(Some(Sequence::new("Sequence 01")))),
            composition: Arc::new(Mutex::new(Some(Composition::new("Comp 01", 1920, 1080, 30.0, 300)))),
            photo_canvas: Arc::new(Mutex::new(Some(PhotoCanvas::new("Canvas 01", 1920, 1080)))),
            vector_doc: Arc::new(Mutex::new(Some(VectorDocument::new("Artwork 01", 1920.0, 1080.0)))),
            light_photo: Arc::new(Mutex::new(Some(LightPhoto::new("photo_raw.dng")))),
            cad_drawing: Arc::new(Mutex::new(Some(CadDrawing::new("Drawing 01")))),
            design_doc: Arc::new(Mutex::new(Some(DesignDocument::new("Publication 01")))),
            pdf_doc: Arc::new(Mutex::new(Some(PdfDocument::new("Document 01")))),
            sound_project: Arc::new(Mutex::new(Some(SoundProject::new("Track Session 01")))),
            event_bus: EventBus::new(),
        }
    }

    /// Register an asset into the unified workspace catalog, available to all craft engines.
    pub fn register_asset(&self, name: impl Into<String>, kind: AssetKind, uri: impl Into<String>) -> Id {
        let mut proj = self.project.lock().unwrap();
        let asset = AssetEntry::new(name, kind, uri);
        proj.asset_catalog.register(asset)
    }

    /// High-level Semantic Inspection: provides clean, token-efficient structural outlines
    /// of the active creative workspace for AI agent context windows.
    pub fn inspect_semantic(&self, domain: &str) -> Value {
        match domain.to_lowercase().as_str() {
            "word" | "doc" => {
                let lock = self.word_doc.lock().unwrap();
                if let Some(doc) = lock.as_ref() {
                    let mut outline = Vec::new();
                    let mut word_count = 0;
                    for block in &doc.blocks {
                        if let dreamcraft_primitives::tool::DocumentBlock::Paragraph(p) = block {
                            let text = p.plain_text();
                            word_count += text.split_whitespace().count();
                            if !matches!(p.heading, dreamcraft_primitives::tool::HeadingLevel::Body) {
                                outline.push(json!({
                                    "heading": format!("{:?}", p.heading),
                                    "text": text
                                }));
                            }
                        }
                    }
                    json!({
                        "domain": "word",
                        "title": doc.title,
                        "blocks_count": doc.blocks.len(),
                        "estimated_words": word_count,
                        "heading_outline": outline
                    })
                } else {
                    json!({ "domain": "word", "status": "empty" })
                }
            }

            "grid" | "sheet" => {
                let lock = self.workbook.lock().unwrap();
                if let Some(wb) = lock.as_ref() {
                    let sheet_summaries: Vec<Value> = wb.sheets.iter().map(|s| {
                        json!({
                            "sheet_name": s.name,
                            "max_col": s.max_col,
                            "max_row": s.max_row,
                            "non_empty_cells": s.cells.len(),
                        })
                    }).collect();

                    json!({
                        "domain": "grid",
                        "title": wb.title,
                        "sheets_count": wb.sheets.len(),
                        "active_sheet": wb.active_sheet().name,
                        "sheets": sheet_summaries
                    })
                } else {
                    json!({ "domain": "grid", "status": "empty" })
                }
            }

            "film" | "timeline" => {
                let lock = self.film_seq.lock().unwrap();
                if let Some(seq) = lock.as_ref() {
                    let video_tracks: Vec<Value> = seq.video_tracks.iter().map(|t| {
                        json!({
                            "track_name": t.name,
                            "clips_count": t.items.len(),
                            "clips": t.items.iter().map(|i| json!({
                                "id": i.id.to_string(),
                                "name": i.name,
                                "start_seconds": i.start.to_seconds(),
                                "duration_seconds": i.duration.to_seconds(),
                            })).collect::<Vec<_>>()
                        })
                    }).collect();

                    let audio_tracks: Vec<Value> = seq.audio_tracks.iter().map(|t| {
                        json!({
                            "track_name": t.name,
                            "clips_count": t.items.len(),
                        })
                    }).collect();

                    json!({
                        "domain": "film",
                        "name": seq.name,
                        "fps": seq.settings.fps,
                        "total_duration_seconds": seq.total_duration().to_seconds(),
                        "video_tracks": video_tracks,
                        "audio_tracks": audio_tracks
                    })
                } else {
                    json!({ "domain": "film", "status": "empty" })
                }
            }

            "deck" | "presentation" => {
                let lock = self.deck.lock().unwrap();
                if let Some(deck) = lock.as_ref() {
                    let slide_outlines: Vec<Value> = deck.slides.iter().enumerate().map(|(idx, s)| {
                        json!({
                            "slide_index": idx + 1,
                            "layout": format!("{:?}", s.layout),
                            "title": s.title,
                            "subtitle": s.subtitle,
                            "bullets_count": s.text_boxes.iter().filter(|b| b.is_bullet).count(),
                            "has_notes": !s.notes.is_empty(),
                        })
                    }).collect();

                    json!({
                        "domain": "deck",
                        "title": deck.title,
                        "theme": deck.theme.name,
                        "slides_count": deck.slides.len(),
                        "current_slide": deck.current_slide + 1,
                        "slides": slide_outlines
                    })
                } else {
                    json!({ "domain": "deck", "status": "empty" })
                }
            }

            "sound" | "audio" => {
                let lock = self.sound_project.lock().unwrap();
                if let Some(sound) = lock.as_ref() {
                    let tracks: Vec<Value> = sound.tracks.iter().map(|t| {
                        json!({
                            "name": t.name,
                            "volume_db": t.volume_db,
                            "pan": t.pan,
                            "muted": t.muted,
                            "clips_count": t.clips.len(),
                            "effects_count": t.effects.len(),
                            "automation_lanes": t.automations.len(),
                        })
                    }).collect();

                    json!({
                        "domain": "sound",
                        "name": sound.name,
                        "bpm": sound.bpm,
                        "sample_rate": sound.sample_rate,
                        "master_volume_db": sound.master_volume_db,
                        "tracks": tracks
                    })
                } else {
                    json!({ "domain": "sound", "status": "empty" })
                }
            }

            "assets" | "catalog" => {
                let proj = self.project.lock().unwrap();
                let assets: Vec<Value> = proj.asset_catalog.assets.values().map(|a| {
                    json!({
                        "id": a.id.to_string(),
                        "name": a.name,
                        "kind": format!("{:?}", a.kind),
                        "uri": a.uri_or_path,
                        "tags": a.tags,
                    })
                }).collect();

                json!({
                    "domain": "assets",
                    "total_assets": assets.len(),
                    "assets": assets
                })
            }

            _ => {
                // Return high-level summary of the entire unified workspace
                json!({
                    "workspace": "DreamCraft AI Studio",
                    "project_name": self.project.lock().unwrap().name,
                    "active_engines": [
                        "wordcraft (documents & typography)",
                        "gridcraft (spreadsheets & formulas)",
                        "filmcraft (premiere NLE timeline)",
                        "deckcraft (presentation slides)",
                        "soundcraft (multitrack DAW)",
                        "lightcraft (RAW photo develop)",
                        "effectcraft (motion graphics compositing)",
                        "designcraft (desktop publishing & spreads)",
                        "photocraft (raster canvas & layer styles)",
                        "vectorcraft (illustrator paths & SVG)",
                        "cadcraft (drafting & DXF)",
                        "pdfcraft (acrobat page manipulation)",
                        "artcraft (generative multi-modal AI)"
                    ],
                    "asset_catalog_count": self.project.lock().unwrap().asset_catalog.len(),
                    "command_history_count": self.project.lock().unwrap().command_history.past.len()
                })
            }
        }
    }
}
