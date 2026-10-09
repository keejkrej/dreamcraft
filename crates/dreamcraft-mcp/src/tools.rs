use crate::state::DreamSession;
use dreamcraft_core::{ChangeEvent, Color, Id, Rect, Tick};
use dreamcraft_primitives::ai::CreativePipeline;
use dreamcraft_primitives::tool::{
    AudioEffect, CellCoord, Compressor, DeckThemeKind, Delay, HeadingLevel, ParametricEq, Reverb, SlideLayout,
    TrackKind, TransitionKind, WordDocument, Workbook,
};
use serde_json::{Value, json};

pub struct ToolResult {
    pub content: Vec<Value>,
    pub is_error: bool,
}

impl ToolResult {
    pub fn text(t: impl Into<String>) -> Self {
        Self {
            content: vec![json!({"type": "text", "text": t.into()})],
            is_error: false,
        }
    }

    pub fn json(v: &Value) -> Self {
        Self::text(serde_json::to_string_pretty(v).unwrap_or_default())
    }

    pub fn error(t: impl Into<String>) -> Self {
        Self {
            content: vec![json!({"type": "text", "text": t.into()})],
            is_error: true,
        }
    }

    pub fn to_value(&self) -> Value {
        json!({"content": self.content, "isError": self.is_error})
    }
}

fn tool_spec(name: &str, title: &str, desc: &str, schema: Value) -> Value {
    json!({
        "name": name,
        "title": title,
        "description": desc,
        "inputSchema": schema
    })
}

pub fn tool_definitions() -> Value {
    json!([
        // --- 1. WORD PRIMITIVES (WordCraft) ---
        tool_spec(
            "word_new_document",
            "New Word Document",
            "Create a new clean Word document with a title.",
            json!({
                "type": "object",
                "properties": { "title": { "type": "string", "description": "Document title" } },
                "required": ["title"]
            })
        ),
        tool_spec(
            "word_add_heading",
            "Add Heading to Word Document",
            "Add a heading (level 1-4, Title) to the active Word document.",
            json!({
                "type": "object",
                "properties": {
                    "level": { "type": "integer", "description": "Heading level 1, 2, 3, or 4" },
                    "text": { "type": "string", "description": "Heading text" }
                },
                "required": ["level", "text"]
            })
        ),
        tool_spec(
            "word_add_paragraph",
            "Add Paragraph to Word Document",
            "Add a body paragraph to the active Word document with optional styling.",
            json!({
                "type": "object",
                "properties": {
                    "text": { "type": "string", "description": "Paragraph text content" },
                    "bold": { "type": "boolean" },
                    "italic": { "type": "boolean" }
                },
                "required": ["text"]
            })
        ),
        tool_spec(
            "word_insert_table",
            "Insert Table to Word Document",
            "Insert a 2D structured table into the active Word document.",
            json!({
                "type": "object",
                "properties": {
                    "data": {
                        "type": "array",
                        "items": { "type": "array", "items": { "type": "string" } },
                        "description": "2D array of table row cells"
                    },
                    "has_header": { "type": "boolean" }
                },
                "required": ["data"]
            })
        ),
        tool_spec(
            "word_get_content",
            "Get Word Document Content",
            "Get full Word document as formatted Markdown or plain text.",
            json!({
                "type": "object",
                "properties": {
                    "format": { "type": "string", "enum": ["markdown", "text"] }
                }
            })
        ),
        tool_spec(
            "word_search_replace",
            "Search and Replace in Word Document",
            "Search and replace text occurrences in the active Word document.",
            json!({
                "type": "object",
                "properties": {
                    "find": { "type": "string" },
                    "replace_with": { "type": "string" }
                },
                "required": ["find", "replace_with"]
            })
        ),

        // --- 2. GRID / SPREADSHEET PRIMITIVES (GridCraft) ---
        tool_spec(
            "grid_new_workbook",
            "New Excel Workbook",
            "Create a new blank Excel-style workbook.",
            json!({
                "type": "object",
                "properties": { "title": { "type": "string" } },
                "required": ["title"]
            })
        ),
        tool_spec(
            "grid_set_cell",
            "Set Cell in Spreadsheet",
            "Set a cell value or formula (e.g. coord='A1', value='100', or coord='B5', value='=SUM(A1:A4)').",
            json!({
                "type": "object",
                "properties": {
                    "coord": { "type": "string", "description": "Cell coordinate, e.g. A1, B2" },
                    "value": { "type": "string", "description": "Number, text, or formula starting with '='" }
                },
                "required": ["coord", "value"]
            })
        ),
        tool_spec(
            "grid_read_range",
            "Read Range from Spreadsheet",
            "Read a 2D range of cell values, e.g. 'A1:C5' or 'B2'.",
            json!({
                "type": "object",
                "properties": { "range": { "type": "string" } },
                "required": ["range"]
            })
        ),
        tool_spec(
            "grid_write_range",
            "Write Range to Spreadsheet",
            "Write a 2D array of rows starting at a top-left cell coordinate.",
            json!({
                "type": "object",
                "properties": {
                    "start": { "type": "string" },
                    "values": {
                        "type": "array",
                        "items": { "type": "array", "items": { "type": "string" } }
                    }
                },
                "required": ["start", "values"]
            })
        ),
        tool_spec(
            "grid_export_csv",
            "Export Spreadsheet as CSV",
            "Export the active spreadsheet data to CSV format.",
            json!({ "type": "object", "properties": {} })
        ),

        // --- 3. FILM / VIDEO EDITING PRIMITIVES (FilmCraft) ---
        tool_spec(
            "film_new_sequence",
            "New Video Sequence",
            "Create a new video editing sequence / timeline.",
            json!({
                "type": "object",
                "properties": {
                    "name": { "type": "string" },
                    "fps": { "type": "number" }
                },
                "required": ["name"]
            })
        ),
        tool_spec(
            "film_insert_clip",
            "Insert Clip to Timeline",
            "Insert a video/audio clip on a track with start time and duration.",
            json!({
                "type": "object",
                "properties": {
                    "track_kind": { "type": "string", "enum": ["video", "audio"] },
                    "track_index": { "type": "integer" },
                    "name": { "type": "string" },
                    "source_path": { "type": "string" },
                    "start_seconds": { "type": "number" },
                    "duration_seconds": { "type": "number" }
                },
                "required": ["track_kind", "name", "source_path", "start_seconds", "duration_seconds"]
            })
        ),
        tool_spec(
            "film_razor_cut",
            "Razor Cut Clip",
            "Perform a razor cut / split on a track item at a specified time.",
            json!({
                "type": "object",
                "properties": {
                    "track_kind": { "type": "string", "enum": ["video", "audio"] },
                    "track_index": { "type": "integer" },
                    "cut_time_seconds": { "type": "number" }
                },
                "required": ["track_kind", "cut_time_seconds"]
            })
        ),
        tool_spec(
            "film_ripple_delete",
            "Ripple Delete Clip",
            "Delete a clip by ID and ripple close the gap by shifting following clips earlier.",
            json!({
                "type": "object",
                "properties": { "clip_id": { "type": "string" } },
                "required": ["clip_id"]
            })
        ),
        tool_spec(
            "film_trim_clip",
            "Trim Clip Duration",
            "Trim a clip's head or tail duration.",
            json!({
                "type": "object",
                "properties": {
                    "clip_id": { "type": "string" },
                    "delta_seconds": { "type": "number" },
                    "trim_tail": { "type": "boolean" }
                },
                "required": ["clip_id", "delta_seconds", "trim_tail"]
            })
        ),
        tool_spec(
            "film_export_edl",
            "Export Edit Decision List (EDL)",
            "Export the sequence as an industry-standard EDL (Edit Decision List) file for Premiere/DaVinci.",
            json!({ "type": "object", "properties": {} })
        ),

        // --- 4. DECK / PRESENTATION PRIMITIVES (DeckCraft) ---
        tool_spec(
            "deck_new_presentation",
            "New Slide Presentation",
            "Create a new PowerPoint-style slide presentation.",
            json!({
                "type": "object",
                "properties": { "title": { "type": "string" } },
                "required": ["title"]
            })
        ),
        tool_spec(
            "deck_add_slide",
            "Add Slide to Presentation",
            "Add a slide with a layout and title.",
            json!({
                "type": "object",
                "properties": {
                    "layout": { "type": "string", "enum": ["TitleSlide", "TitleAndContent", "TwoColumns", "Blank"] },
                    "title": { "type": "string" },
                    "subtitle": { "type": "string" }
                },
                "required": ["title"]
            })
        ),
        tool_spec(
            "deck_add_bullet",
            "Add Bullet Point to Slide",
            "Add a bullet point text item to the current slide.",
            json!({
                "type": "object",
                "properties": {
                    "slide_index": { "type": "integer" },
                    "text": { "type": "string" },
                    "y_offset": { "type": "number" }
                },
                "required": ["text"]
            })
        ),
        tool_spec(
            "deck_add_image",
            "Insert Image to Slide",
            "Insert an image (path or generated URL) to a slide with position and size.",
            json!({
                "type": "object",
                "properties": {
                    "slide_index": { "type": "integer" },
                    "source_path": { "type": "string" },
                    "x": { "type": "number" },
                    "y": { "type": "number" },
                    "width": { "type": "number" },
                    "height": { "type": "number" }
                },
                "required": ["source_path", "x", "y", "width", "height"]
            })
        ),

        // --- 5. SOUND / AUDIO MIXER PRIMITIVES (SoundCraft / Pro Tools) ---
        tool_spec(
            "sound_new_project",
            "New Audio Project (DAW)",
            "Create a new Pro Tools-style audio mixing project with tracks.",
            json!({
                "type": "object",
                "properties": { "name": { "type": "string" } },
                "required": ["name"]
            })
        ),
        tool_spec(
            "sound_add_track",
            "Add Audio Track",
            "Add a named audio track to the DAW session (e.g. Vocal, Drums, Synth).",
            json!({
                "type": "object",
                "properties": { "name": { "type": "string" } },
                "required": ["name"]
            })
        ),
        tool_spec(
            "sound_set_fader",
            "Set Track Volume Fader",
            "Adjust a track's volume fader in dB (-inf to +12 dB).",
            json!({
                "type": "object",
                "properties": {
                    "track_index": { "type": "integer" },
                    "volume_db": { "type": "number" }
                },
                "required": ["track_index", "volume_db"]
            })
        ),
        tool_spec(
            "sound_add_effect",
            "Add DSP Effect to Audio Track",
            "Add an EQ, Compressor, Reverb, or Delay effect to an audio track.",
            json!({
                "type": "object",
                "properties": {
                    "track_index": { "type": "integer" },
                    "effect_kind": { "type": "string", "enum": ["eq", "compressor", "reverb", "delay"] }
                },
                "required": ["track_index", "effect_kind"]
            })
        ),

        // --- 6. LIGHT / PHOTO RAW DEVELOP PRIMITIVES (LightCraft / Lightroom) ---
        tool_spec(
            "light_develop_photo",
            "Develop Photo (Lightroom)",
            "Non-destructively develop a RAW photo: exposure, white balance (temp, tint), highlights/shadows.",
            json!({
                "type": "object",
                "properties": {
                    "exposure_ev": { "type": "number", "description": "Exposure EV adjustment (-5.0 .. +5.0)" },
                    "temperature": { "type": "number", "description": "White balance temperature (-100 .. +100)" },
                    "tint": { "type": "number", "description": "White balance tint (-100 .. +100)" },
                    "highlights": { "type": "number", "description": "Highlights recovery (-100 .. +100)" },
                    "shadows": { "type": "number", "description": "Shadows lift (-100 .. +100)" }
                }
            })
        ),
        tool_spec(
            "light_inspect_photo",
            "Inspect Photo Develop State",
            "Inspect active photo metadata (ISO, shutter, aperture) and develop settings.",
            json!({ "type": "object", "properties": {} })
        ),

        // --- 7. EFFECT / MOTION GRAPHICS & VFX PRIMITIVES (EffectCraft / After Effects) ---
        tool_spec(
            "effect_new_comp",
            "New Motion Graphics Composition",
            "Create an After Effects-style composition with fps and duration.",
            json!({
                "type": "object",
                "properties": {
                    "name": { "type": "string" },
                    "width": { "type": "integer" },
                    "height": { "type": "integer" },
                    "fps": { "type": "number" },
                    "duration_frames": { "type": "integer" }
                },
                "required": ["name", "width", "height", "fps", "duration_frames"]
            })
        ),
        tool_spec(
            "effect_add_layer",
            "Add Layer to Composition",
            "Add a Solid or Text layer to the active composition.",
            json!({
                "type": "object",
                "properties": {
                    "name": { "type": "string" },
                    "kind": { "type": "string", "enum": ["solid", "text"] },
                    "content_or_hex": { "type": "string" }
                },
                "required": ["name", "kind", "content_or_hex"]
            })
        ),

        // --- 8. DESIGN / DESKTOP PUBLISHING PRIMITIVES (DesignCraft / InDesign) ---
        tool_spec(
            "design_new_document",
            "New Print Publication (InDesign)",
            "Create a multi-page publication document with spreads and margins.",
            json!({
                "type": "object",
                "properties": { "title": { "type": "string" } },
                "required": ["title"]
            })
        ),
        tool_spec(
            "design_add_text_frame",
            "Add Text Frame to Page",
            "Add a multi-column text frame with threaded story flow to a page.",
            json!({
                "type": "object",
                "properties": {
                    "page_index": { "type": "integer" },
                    "x": { "type": "number" },
                    "y": { "type": "number" },
                    "width": { "type": "number" },
                    "height": { "type": "number" },
                    "text": { "type": "string" }
                },
                "required": ["page_index", "x", "y", "width", "height", "text"]
            })
        ),

        // --- 9. PHOTO / RASTER PRIMITIVES (PhotoCraft) ---
        tool_spec(
            "photo_create_canvas",
            "Create Photo Canvas",
            "Create a raster image canvas with width and height.",
            json!({
                "type": "object",
                "properties": {
                    "name": { "type": "string" },
                    "width": { "type": "integer" },
                    "height": { "type": "integer" }
                },
                "required": ["name", "width", "height"]
            })
        ),

        // --- 10. VECTOR PRIMITIVES (VectorCraft) ---
        tool_spec(
            "vector_create",
            "Create Vector Document",
            "Create an Illustrator-style vector SVG document.",
            json!({
                "type": "object",
                "properties": {
                    "name": { "type": "string" },
                    "width": { "type": "number" },
                    "height": { "type": "number" }
                },
                "required": ["name", "width", "height"]
            })
        ),
        tool_spec(
            "vector_add_rect",
            "Add Rectangle to Vector Document",
            "Add a rectangle to the active vector document.",
            json!({
                "type": "object",
                "properties": {
                    "x": { "type": "number" },
                    "y": { "type": "number" },
                    "width": { "type": "number" },
                    "height": { "type": "number" },
                    "fill_hex": { "type": "string" }
                },
                "required": ["x", "y", "width", "height"]
            })
        ),
        tool_spec(
            "vector_export_svg",
            "Export Vector Document to SVG",
            "Get the vector document as SVG markup.",
            json!({ "type": "object", "properties": {} })
        ),

        // --- 11. CAD PRIMITIVES (CADCraft) ---
        tool_spec(
            "cad_create",
            "Create CAD Drawing",
            "Create a 2D AutoCAD-style CAD drawing.",
            json!({
                "type": "object",
                "properties": { "name": { "type": "string" } },
                "required": ["name"]
            })
        ),
        tool_spec(
            "cad_add_line",
            "Add Line to CAD Drawing",
            "Add a 2D geometric line (x1, y1) to (x2, y2).",
            json!({
                "type": "object",
                "properties": {
                    "x1": { "type": "number" }, "y1": { "type": "number" },
                    "x2": { "type": "number" }, "y2": { "type": "number" }
                },
                "required": ["x1", "y1", "x2", "y2"]
            })
        ),
        tool_spec(
            "cad_export_dxf",
            "Export CAD to DXF",
            "Export the CAD drawing to DXF string.",
            json!({ "type": "object", "properties": {} })
        ),

        // --- 12. PDF PRIMITIVES (PdfCraft) ---
        tool_spec(
            "pdf_create",
            "Create PDF Document",
            "Create a multi-page PDF document.",
            json!({
                "type": "object",
                "properties": { "title": { "type": "string" } },
                "required": ["title"]
            })
        ),

        // --- 13. AI GENERATIVE PRIMITIVES (ArtCraft) ---
        tool_spec(
            "ai_generate_image",
            "AI Image Generation",
            "Generate an image using Fal.ai / Flux / SDXL or deterministic mock provider.",
            json!({
                "type": "object",
                "properties": {
                    "prompt": { "type": "string" },
                    "width": { "type": "integer" },
                    "height": { "type": "integer" }
                },
                "required": ["prompt"]
            })
        ),
        tool_spec(
            "ai_generate_video",
            "AI Video Generation",
            "Generate a video using Sora / Fal / Kling / Kinovi or mock provider.",
            json!({
                "type": "object",
                "properties": {
                    "prompt": { "type": "string" },
                    "duration_seconds": { "type": "number" }
                },
                "required": ["prompt"]
            })
        ),

        // --- 14. CROSS-APP COMPOSITION PRIMITIVES ---
        tool_spec(
            "compose_illustrated_document",
            "Compose Illustrated Word Document",
            "Autonomous composition: Generates AI illustration, outlines sections, builds Word document with tables.",
            json!({
                "type": "object",
                "properties": { "topic": { "type": "string" } },
                "required": ["topic"]
            })
        ),
        tool_spec(
            "compose_pitch_deck",
            "Compose Pitch Slide Deck",
            "Autonomous composition: Generates full multi-slide pitch presentation with illustrations and bullets.",
            json!({
                "type": "object",
                "properties": {
                    "title": { "type": "string" },
                    "num_slides": { "type": "integer" }
                },
                "required": ["title"]
            })
        ),
        tool_spec(
            "compose_ai_film",
            "Compose AI Movie Sequence",
            "Autonomous composition: Generates video clips from scene prompts and cuts them onto the Film timeline with EDL export.",
            json!({
                "type": "object",
                "properties": {
                    "title": { "type": "string" },
                    "scenes": {
                        "type": "array",
                        "items": { "type": "string" }
                    }
                },
                "required": ["title", "scenes"]
            })
        ),
        // --- ADVANCED SUITE POWER PRIMITIVES ---
        tool_spec(
            "grid_sort_range",
            "Sort Spreadsheet Range",
            "Sort rows in a range by a column index (ascending or descending) with header detection.",
            json!({
                "type": "object",
                "properties": {
                    "range": { "type": "string" },
                    "sort_col_offset": { "type": "integer" },
                    "ascending": { "type": "boolean" },
                    "has_header": { "type": "boolean" }
                },
                "required": ["range", "sort_col_offset"]
            })
        ),
        tool_spec(
            "film_slip_clip",
            "Premiere Slip Trim Clip",
            "Adjust source media in/out point without shifting clip placement or duration on timeline.",
            json!({
                "type": "object",
                "properties": {
                    "clip_id": { "type": "string" },
                    "delta_seconds": { "type": "number" }
                },
                "required": ["clip_id", "delta_seconds"]
            })
        ),
        tool_spec(
            "film_roll_edit",
            "Premiere Roll Cut Trim",
            "Adjust cut boundary between two adjacent clips without changing overall sequence length.",
            json!({
                "type": "object",
                "properties": {
                    "left_clip_id": { "type": "string" },
                    "right_clip_id": { "type": "string" },
                    "delta_seconds": { "type": "number" }
                },
                "required": ["left_clip_id", "right_clip_id", "delta_seconds"]
            })
        ),
        tool_spec(
            "film_set_speed",
            "Premiere Rate Stretch / Speed",
            "Change clip playback speed multiplier (0.5x, 2.0x, -1.0x reverse) with optional ripple.",
            json!({
                "type": "object",
                "properties": {
                    "clip_id": { "type": "string" },
                    "speed": { "type": "number" },
                    "ripple": { "type": "boolean" }
                },
                "required": ["clip_id", "speed"]
            })
        ),
        tool_spec(
            "film_add_transition",
            "Add Video Transition",
            "Add a transition (CrossDissolve, DipToBlack, WipeLeft, etc.) on an edit track.",
            json!({
                "type": "object",
                "properties": {
                    "track_id": { "type": "string" },
                    "kind": { "type": "string", "enum": ["CrossDissolve", "DipToBlack", "DipToWhite", "WipeLeft", "WipeRight"] },
                    "start_seconds": { "type": "number" },
                    "duration_seconds": { "type": "number" }
                },
                "required": ["track_id", "kind", "start_seconds", "duration_seconds"]
            })
        ),
        tool_spec(
            "deck_set_theme",
            "Set Deck Theme",
            "Apply professional presentation color & font palette (Harbor, Ember, Meadow, Nocturne, Paper, Slate).",
            json!({
                "type": "object",
                "properties": {
                    "theme": { "type": "string", "enum": ["Harbor", "Ember", "Meadow", "Nocturne", "Paper", "Slate"] }
                },
                "required": ["theme"]
            })
        ),
        tool_spec(
            "deck_add_metric_slide",
            "Add Key Metric Slide",
            "Add an executive KPI / Key Metric highlight slide with large typography and labels.",
            json!({
                "type": "object",
                "properties": {
                    "title": { "type": "string" },
                    "metric_value": { "type": "string" },
                    "metric_label": { "type": "string" },
                    "description": { "type": "string" }
                },
                "required": ["title", "metric_value", "metric_label", "description"]
            })
        ),
        tool_spec(
            "light_apply_preset",
            "Apply Lightroom RAW Preset",
            "Apply professional color grading & tone curve preset to the active photo.",
            json!({
                "type": "object",
                "properties": {
                    "preset": { "type": "string", "enum": ["cinematic_warm", "moody_cold", "vibrant_landscape", "clean_portrait"] }
                },
                "required": ["preset"]
            })
        ),
        tool_spec(
            "sound_add_automation",
            "Add DAW Track Automation",
            "Add volume or pan automation breakpoint at a specific time in the DAW session.",
            json!({
                "type": "object",
                "properties": {
                    "track_index": { "type": "integer" },
                    "parameter": { "type": "string" },
                    "seconds": { "type": "number" },
                    "value": { "type": "number" }
                },
                "required": ["track_index", "parameter", "seconds", "value"]
            })
        ),
        tool_spec(
            "pdf_rotate_page",
            "Rotate PDF Page",
            "Rotate a PDF page clockwise by 90, 180, or 270 degrees.",
            json!({
                "type": "object",
                "properties": {
                    "page_index": { "type": "integer" },
                    "degrees": { "type": "integer" }
                },
                "required": ["page_index", "degrees"]
            })
        ),
        tool_spec(
            "pdf_set_watermark",
            "Set PDF Watermark",
            "Apply a semi-transparent diagonal security watermark across all PDF pages.",
            json!({
                "type": "object",
                "properties": {
                    "text": { "type": "string" }
                },
                "required": ["text"]
            })
        ),
        // --- NEXT-LEVEL MULTI-MODAL ORCHESTRATION ---
        tool_spec(
            "dream_inspect",
            "Inspect Creative Workspace Semantics",
            "High-level token-efficient semantic inspection of any creative engine ('word', 'grid', 'film', 'deck', 'sound', 'assets', or 'all').",
            json!({
                "type": "object",
                "properties": {
                    "domain": { "type": "string", "enum": ["word", "grid", "film", "deck", "sound", "assets", "all"] }
                }
            })
        ),
        tool_spec(
            "dream_register_asset",
            "Register Cross-Domain Asset",
            "Add an image, video, audio, or vector asset into the central project asset catalog to reference across apps.",
            json!({
                "type": "object",
                "properties": {
                    "name": { "type": "string" },
                    "kind": { "type": "string", "enum": ["Image", "Video", "Audio", "Vector", "Dataset", "Document", "Model3D"] },
                    "uri": { "type": "string" }
                },
                "required": ["name", "kind", "uri"]
            })
        ),
        tool_spec(
            "dream_history",
            "Get Execution History",
            "Get the audit trail of commands and operations executed in the active workspace.",
            json!({
                "type": "object"
            })
        )
    ])
}

pub async fn call_tool(session: &DreamSession, name: &str, args: &Value) -> ToolResult {
    match name {
        // --- WORD TOOLS ---
        "word_new_document" => {
            let title = args["title"].as_str().unwrap_or("Untitled Document");
            let doc = WordDocument::new(title);
            let mut lock = session.word_doc.lock().unwrap();
            *lock = Some(doc);
            session.event_bus.emit(ChangeEvent::DocumentUpdated {
                id: "active".into(),
                title: title.to_string(),
            });
            ToolResult::text(format!("Created new Word document: '{}'", title))
        }

        "word_add_heading" => {
            let level_num = args["level"].as_u64().unwrap_or(1);
            let text = args["text"].as_str().unwrap_or_default();
            let heading = match level_num {
                1 => HeadingLevel::Heading1,
                2 => HeadingLevel::Heading2,
                3 => HeadingLevel::Heading3,
                4 => HeadingLevel::Heading4,
                _ => HeadingLevel::Heading1,
            };

            let mut lock = session.word_doc.lock().unwrap();
            if let Some(doc) = lock.as_mut() {
                doc.add_heading(heading, text);
                ToolResult::text(format!("Added Heading {} ('{}')", level_num, text))
            } else {
                ToolResult::error("No active Word document.")
            }
        }

        "word_add_paragraph" => {
            let text = args["text"].as_str().unwrap_or_default();
            let bold = args["bold"].as_bool().unwrap_or(false);
            let italic = args["italic"].as_bool().unwrap_or(false);

            let mut lock = session.word_doc.lock().unwrap();
            if let Some(doc) = lock.as_mut() {
                if bold || italic {
                    let mut run = dreamcraft_primitives::tool::word::TextRun::new(text);
                    if bold { run = run.bold(); }
                    if italic { run = run.italic(); }
                    doc.add_formatted_paragraph(vec![run]);
                } else {
                    doc.add_paragraph(text);
                }
                ToolResult::text("Added paragraph to Word document.")
            } else {
                ToolResult::error("No active Word document.")
            }
        }

        "word_insert_table" => {
            let data_arr = match args["data"].as_array() {
                Some(arr) => arr,
                None => return ToolResult::error("Missing 'data' 2D array."),
            };
            let has_header = args["has_header"].as_bool().unwrap_or(true);

            let mut table_rows: Vec<Vec<String>> = Vec::new();
            for r in data_arr {
                if let Some(row) = r.as_array() {
                    let row_strs: Vec<String> = row
                        .iter()
                        .map(|v| v.as_str().unwrap_or(&v.to_string()).to_string())
                        .collect();
                    table_rows.push(row_strs);
                }
            }

            let mut lock = session.word_doc.lock().unwrap();
            if let Some(doc) = lock.as_mut() {
                let row_cnt = table_rows.len();
                let col_cnt = table_rows.first().map(|r| r.len()).unwrap_or(0);
                doc.insert_table(table_rows, has_header);
                ToolResult::text(format!("Inserted table ({} rows × {} cols)", row_cnt, col_cnt))
            } else {
                ToolResult::error("No active Word document.")
            }
        }

        "word_get_content" => {
            let fmt = args["format"].as_str().unwrap_or("markdown");
            let lock = session.word_doc.lock().unwrap();
            if let Some(doc) = lock.as_ref() {
                if fmt == "text" {
                    ToolResult::text(doc.to_plain_text())
                } else {
                    ToolResult::text(doc.to_markdown())
                }
            } else {
                ToolResult::error("No active Word document.")
            }
        }

        "word_search_replace" => {
            let find = args["find"].as_str().unwrap_or_default();
            let replace_with = args["replace_with"].as_str().unwrap_or_default();
            let mut lock = session.word_doc.lock().unwrap();
            if let Some(doc) = lock.as_mut() {
                let count = doc.search_and_replace(find, replace_with);
                ToolResult::text(format!("Replaced {} occurrences of '{}' with '{}'", count, find, replace_with))
            } else {
                ToolResult::error("No active Word document.")
            }
        }

        // --- GRID TOOLS ---
        "grid_new_workbook" => {
            let title = args["title"].as_str().unwrap_or("Untitled Workbook");
            let mut lock = session.workbook.lock().unwrap();
            *lock = Some(Workbook::new(title));
            ToolResult::text(format!("Created new Workbook: '{}'", title))
        }

        "grid_set_cell" => {
            let coord_str = args["coord"].as_str().unwrap_or_default();
            let val = args["value"].as_str().unwrap_or_default();
            let coord = match CellCoord::parse(coord_str) {
                Some(c) => c,
                None => return ToolResult::error(format!("Invalid cell coordinate '{}'", coord_str)),
            };

            let mut lock = session.workbook.lock().unwrap();
            if let Some(wb) = lock.as_mut() {
                wb.active_sheet_mut().set_cell(coord, val);
                wb.active_sheet_mut().recalculate();
                let computed = wb.active_sheet().get_cell(&coord).computed.display_string();
                ToolResult::text(format!("Set {} = '{}' (computed: '{}')", coord_str, val, computed))
            } else {
                ToolResult::error("No active Workbook.")
            }
        }

        "grid_read_range" => {
            let range = args["range"].as_str().unwrap_or("A1");
            let lock = session.workbook.lock().unwrap();
            if let Some(wb) = lock.as_ref() {
                match wb.active_sheet().read_range(range) {
                    Ok(vals) => ToolResult::json(&json!(vals)),
                    Err(e) => ToolResult::error(e.to_string()),
                }
            } else {
                ToolResult::error("No active Workbook.")
            }
        }

        "grid_write_range" => {
            let start = args["start"].as_str().unwrap_or("A1");
            let val_arr = match args["values"].as_array() {
                Some(arr) => arr,
                None => return ToolResult::error("Missing 'values' array."),
            };
            let mut rows: Vec<Vec<String>> = Vec::new();
            for r in val_arr {
                if let Some(row) = r.as_array() {
                    let r_strs: Vec<String> = row
                        .iter()
                        .map(|v| v.as_str().unwrap_or(&v.to_string()).to_string())
                        .collect();
                    rows.push(r_strs);
                }
            }

            let mut lock = session.workbook.lock().unwrap();
            if let Some(wb) = lock.as_mut() {
                match wb.active_sheet_mut().write_range(start, rows) {
                    Ok(_) => ToolResult::text(format!("Successfully wrote range starting at {}", start)),
                    Err(e) => ToolResult::error(e.to_string()),
                }
            } else {
                ToolResult::error("No active Workbook.")
            }
        }

        "grid_export_csv" => {
            let lock = session.workbook.lock().unwrap();
            if let Some(wb) = lock.as_ref() {
                ToolResult::text(wb.active_sheet().to_csv())
            } else {
                ToolResult::error("No active Workbook.")
            }
        }

        // --- FILM TOOLS ---
        "film_new_sequence" => {
            let name = args["name"].as_str().unwrap_or("Sequence 01");
            let fps = args["fps"].as_f64().unwrap_or(24.0);
            let mut seq = dreamcraft_primitives::tool::film::Sequence::new(name);
            seq.settings.fps = fps;
            let mut lock = session.film_seq.lock().unwrap();
            *lock = Some(seq);
            ToolResult::text(format!("Created sequence '{}' ({} fps)", name, fps))
        }

        "film_insert_clip" => {
            let track_kind_str = args["track_kind"].as_str().unwrap_or("video");
            let track_idx = args["track_index"].as_u64().unwrap_or(0) as usize;
            let name = args["name"].as_str().unwrap_or("Clip");
            let source_path = args["source_path"].as_str().unwrap_or_default();
            let start_s = args["start_seconds"].as_f64().unwrap_or(0.0);
            let dur_s = args["duration_seconds"].as_f64().unwrap_or(4.0);

            let mut lock = session.film_seq.lock().unwrap();
            if let Some(seq) = lock.as_mut() {
                let track_id = if track_kind_str == "audio" {
                    if track_idx >= seq.audio_tracks.len() {
                        seq.add_track(TrackKind::Audio, format!("A{}", track_idx + 1))
                    } else {
                        seq.audio_tracks[track_idx].id
                    }
                } else {
                    if track_idx >= seq.video_tracks.len() {
                        seq.add_track(TrackKind::Video, format!("V{}", track_idx + 1))
                    } else {
                        seq.video_tracks[track_idx].id
                    }
                };

                let start_tick = Tick::from_seconds(start_s);
                let dur_tick = Tick::from_seconds(dur_s);

                match seq.insert_clip(track_id, name, source_path, start_tick, dur_tick) {
                    Ok(id) => ToolResult::text(format!("Inserted clip '{}' (id: {}, {:.2}s - {:.2}s)", name, id.short_str(), start_s, start_s + dur_s)),
                    Err(e) => ToolResult::error(e.to_string()),
                }
            } else {
                ToolResult::error("No active Sequence.")
            }
        }

        "film_razor_cut" => {
            let track_kind_str = args["track_kind"].as_str().unwrap_or("video");
            let track_idx = args["track_index"].as_u64().unwrap_or(0) as usize;
            let cut_s = args["cut_time_seconds"].as_f64().unwrap_or(0.0);

            let mut lock = session.film_seq.lock().unwrap();
            if let Some(seq) = lock.as_mut() {
                let track_id = if track_kind_str == "audio" {
                    seq.audio_tracks.get(track_idx).map(|t| t.id)
                } else {
                    seq.video_tracks.get(track_idx).map(|t| t.id)
                };

                let tid = match track_id {
                    Some(id) => id,
                    None => return ToolResult::error("Track index out of bounds."),
                };

                match seq.razor_cut(tid, Tick::from_seconds(cut_s)) {
                    Ok((l, r)) => ToolResult::text(format!("Razor cut executed at {:.2}s into clips {} and {}", cut_s, l.short_str(), r.short_str())),
                    Err(e) => ToolResult::error(e.to_string()),
                }
            } else {
                ToolResult::error("No active Sequence.")
            }
        }

        "film_ripple_delete" => {
            let clip_id_str = args["clip_id"].as_str().unwrap_or_default();
            let parsed_id = match uuid::Uuid::parse_str(clip_id_str) {
                Ok(u) => Id(u),
                Err(_) => return ToolResult::error("Invalid clip UUID."),
            };

            let mut lock = session.film_seq.lock().unwrap();
            if let Some(seq) = lock.as_mut() {
                match seq.ripple_delete(parsed_id) {
                    Ok(shift) => ToolResult::text(format!("Ripple delete successful: shifted subsequent clips by {:.2}s", shift.to_seconds())),
                    Err(e) => ToolResult::error(e.to_string()),
                }
            } else {
                ToolResult::error("No active Sequence.")
            }
        }

        "film_trim_clip" => {
            let clip_id_str = args["clip_id"].as_str().unwrap_or_default();
            let delta_s = args["delta_seconds"].as_f64().unwrap_or(0.0);
            let trim_tail = args["trim_tail"].as_bool().unwrap_or(true);

            let parsed_id = match uuid::Uuid::parse_str(clip_id_str) {
                Ok(u) => Id(u),
                Err(_) => return ToolResult::error("Invalid clip UUID."),
            };

            let mut lock = session.film_seq.lock().unwrap();
            if let Some(seq) = lock.as_mut() {
                match seq.trim_clip(parsed_id, Tick::from_seconds(delta_s), trim_tail) {
                    Ok(_) => ToolResult::text(format!("Trimmed clip {} by {:.2}s", parsed_id.short_str(), delta_s)),
                    Err(e) => ToolResult::error(e.to_string()),
                }
            } else {
                ToolResult::error("No active Sequence.")
            }
        }

        "film_export_edl" => {
            let lock = session.film_seq.lock().unwrap();
            if let Some(seq) = lock.as_ref() {
                ToolResult::text(seq.export_edl())
            } else {
                ToolResult::error("No active Sequence.")
            }
        }

        // --- DECK TOOLS ---
        "deck_new_presentation" => {
            let title = args["title"].as_str().unwrap_or("Untitled Presentation");
            let mut lock = session.deck.lock().unwrap();
            *lock = Some(dreamcraft_primitives::tool::Presentation::new(title));
            ToolResult::text(format!("Created new Presentation: '{}'", title))
        }

        "deck_add_slide" => {
            let title = args["title"].as_str().unwrap_or("Slide");
            let layout_str = args["layout"].as_str().unwrap_or("TitleAndContent");
            let layout = match layout_str {
                "TitleSlide" => SlideLayout::TitleSlide,
                "TwoColumns" => SlideLayout::TwoColumns,
                "Blank" => SlideLayout::Blank,
                _ => SlideLayout::TitleAndContent,
            };

            let mut lock = session.deck.lock().unwrap();
            if let Some(d) = lock.as_mut() {
                let idx = d.add_slide(layout, title);
                if let Some(sub) = args["subtitle"].as_str() {
                    if let Some(s) = d.slides.get_mut(idx) {
                        s.subtitle = Some(sub.to_string());
                    }
                }
                ToolResult::text(format!("Added Slide {} ('{}')", idx + 1, title))
            } else {
                ToolResult::error("No active Presentation.")
            }
        }

        "deck_add_bullet" => {
            let text = args["text"].as_str().unwrap_or_default();
            let y_off = args["y_offset"].as_f64().unwrap_or(0.0);
            let s_idx = args["slide_index"].as_u64();

            let mut lock = session.deck.lock().unwrap();
            if let Some(d) = lock.as_mut() {
                let target_slide = match s_idx {
                    Some(i) => d.slides.get_mut(i as usize),
                    None => d.current_slide_mut(),
                };
                if let Some(s) = target_slide {
                    s.add_bullet(text, y_off);
                    ToolResult::text(format!("Added bullet: '{}'", text))
                } else {
                    ToolResult::error("Slide index out of bounds.")
                }
            } else {
                ToolResult::error("No active Presentation.")
            }
        }

        "deck_add_image" => {
            let src = args["source_path"].as_str().unwrap_or_default();
            let x = args["x"].as_f64().unwrap_or(100.0);
            let y = args["y"].as_f64().unwrap_or(100.0);
            let w = args["width"].as_f64().unwrap_or(400.0);
            let h = args["height"].as_f64().unwrap_or(300.0);
            let s_idx = args["slide_index"].as_u64();

            let mut lock = session.deck.lock().unwrap();
            if let Some(d) = lock.as_mut() {
                let target_slide = match s_idx {
                    Some(i) => d.slides.get_mut(i as usize),
                    None => d.current_slide_mut(),
                };
                if let Some(s) = target_slide {
                    s.add_image(src, Rect::new(x, y, w, h));
                    ToolResult::text(format!("Added image '{}' to slide", src))
                } else {
                    ToolResult::error("Slide index out of bounds.")
                }
            } else {
                ToolResult::error("No active Presentation.")
            }
        }

        // --- 5. SOUND TOOLS (SoundCraft / Pro Tools) ---
        "sound_new_project" => {
            let name = args["name"].as_str().unwrap_or("Audio Project");
            let mut lock = session.sound_project.lock().unwrap();
            *lock = Some(dreamcraft_primitives::tool::SoundProject::new(name));
            ToolResult::text(format!("Created Pro Tools audio session: '{}' with default tracks", name))
        }

        "sound_add_track" => {
            let name = args["name"].as_str().unwrap_or("Track");
            let mut lock = session.sound_project.lock().unwrap();
            if let Some(proj) = lock.as_mut() {
                let id = proj.add_track(name);
                ToolResult::text(format!("Added audio track '{}' (id: {})", name, id.short_str()))
            } else {
                ToolResult::error("No active Audio Project.")
            }
        }

        "sound_set_fader" => {
            let t_idx = args["track_index"].as_u64().unwrap_or(0) as usize;
            let vol = args["volume_db"].as_f64().unwrap_or(0.0);
            let mut lock = session.sound_project.lock().unwrap();
            if let Some(proj) = lock.as_mut() {
                if let Some(track) = proj.tracks.get_mut(t_idx) {
                    track.volume_db = vol;
                    ToolResult::text(format!("Set track '{}' fader to {:.1} dB", track.name, vol))
                } else {
                    ToolResult::error("Track index out of bounds.")
                }
            } else {
                ToolResult::error("No active Audio Project.")
            }
        }

        "sound_add_effect" => {
            let t_idx = args["track_index"].as_u64().unwrap_or(0) as usize;
            let kind = args["effect_kind"].as_str().unwrap_or("eq");
            let eff = match kind {
                "compressor" => AudioEffect::Compressor(Compressor::default()),
                "reverb" => AudioEffect::Reverb(Reverb::default()),
                "delay" => AudioEffect::Delay(Delay::default()),
                _ => AudioEffect::Equalizer(ParametricEq::default()),
            };

            let mut lock = session.sound_project.lock().unwrap();
            if let Some(proj) = lock.as_mut() {
                if let Some(track) = proj.tracks.get_mut(t_idx) {
                    track.effects.push(eff);
                    ToolResult::text(format!("Added {} effect to track '{}'", kind, track.name))
                } else {
                    ToolResult::error("Track index out of bounds.")
                }
            } else {
                ToolResult::error("No active Audio Project.")
            }
        }

        // --- 6. LIGHT TOOLS (LightCraft / Lightroom) ---
        "light_develop_photo" => {
            let mut lock = session.light_photo.lock().unwrap();
            if let Some(photo) = lock.as_mut() {
                if let Some(ev) = args["exposure_ev"].as_f64() {
                    photo.set_exposure(ev);
                }
                if let (Some(temp), Some(tint)) = (args["temperature"].as_f64(), args["tint"].as_f64()) {
                    photo.set_white_balance(temp, tint);
                }
                if let (Some(hl), Some(sh)) = (args["highlights"].as_f64(), args["shadows"].as_f64()) {
                    photo.set_highlights_shadows(hl, sh);
                }
                ToolResult::text(format!("Updated Lightroom develop parameters for '{}': Exposure={:.2} EV, Temp={:.1}, Tint={:.1}",
                    photo.path, photo.settings.tone.exposure, photo.settings.wb.temperature, photo.settings.wb.tint))
            } else {
                ToolResult::error("No active Photo to develop.")
            }
        }

        "light_inspect_photo" => {
            let lock = session.light_photo.lock().unwrap();
            if let Some(photo) = lock.as_ref() {
                ToolResult::json(&json!(photo))
            } else {
                ToolResult::error("No active Photo.")
            }
        }

        // --- 7. EFFECT TOOLS (EffectCraft / After Effects) ---
        "effect_new_comp" => {
            let name = args["name"].as_str().unwrap_or("Comp 01");
            let w = args["width"].as_u64().unwrap_or(1920) as u32;
            let h = args["height"].as_u64().unwrap_or(1080) as u32;
            let fps = args["fps"].as_f64().unwrap_or(30.0);
            let frames = args["duration_frames"].as_i64().unwrap_or(300);

            let mut lock = session.composition.lock().unwrap();
            *lock = Some(dreamcraft_primitives::tool::Composition::new(name, w, h, fps, frames));
            ToolResult::text(format!("Created After Effects composition: '{}' ({}×{} @ {} fps, {} frames)", name, w, h, fps, frames))
        }

        "effect_add_layer" => {
            let name = args["name"].as_str().unwrap_or("Layer");
            let kind_str = args["kind"].as_str().unwrap_or("solid");
            let content = args["content_or_hex"].as_str().unwrap_or("#FFFFFF");

            let mut lock = session.composition.lock().unwrap();
            if let Some(comp) = lock.as_mut() {
                if kind_str == "text" {
                    let id = comp.add_text_layer(name, content);
                    ToolResult::text(format!("Added Text layer '{}' (text: '{}', id: {})", name, content, id.short_str()))
                } else {
                    let color = Color::from_hex(content).unwrap_or(Color::WHITE);
                    let id = comp.add_solid_layer(name, color);
                    ToolResult::text(format!("Added Solid layer '{}' (color: {}, id: {})", name, content, id.short_str()))
                }
            } else {
                ToolResult::error("No active Composition.")
            }
        }

        // --- 8. DESIGN TOOLS (DesignCraft / InDesign) ---
        "design_new_document" => {
            let title = args["title"].as_str().unwrap_or("Publication 01");
            let mut lock = session.design_doc.lock().unwrap();
            *lock = Some(dreamcraft_primitives::tool::DesignDocument::new(title));
            ToolResult::text(format!("Created InDesign publication document: '{}' (Facing Pages, US Letter)", title))
        }

        "design_add_text_frame" => {
            let page_idx = args["page_index"].as_u64().unwrap_or(0) as usize;
            let x = args["x"].as_f64().unwrap_or(36.0);
            let y = args["y"].as_f64().unwrap_or(36.0);
            let w = args["width"].as_f64().unwrap_or(540.0);
            let h = args["height"].as_f64().unwrap_or(720.0);
            let text = args["text"].as_str().unwrap_or_default();

            let mut lock = session.design_doc.lock().unwrap();
            if let Some(doc) = lock.as_mut() {
                match doc.add_text_frame(page_idx, Rect::new(x, y, w, h), text) {
                    Some(id) => ToolResult::text(format!("Added threaded text frame to Page {} (id: {})", page_idx + 1, id.short_str())),
                    None => ToolResult::error("Page index out of bounds."),
                }
            } else {
                ToolResult::error("No active Design Document.")
            }
        }

        // --- 9. PHOTO TOOLS ---
        "photo_create_canvas" => {
            let name = args["name"].as_str().unwrap_or("Canvas 01");
            let w = args["width"].as_u64().unwrap_or(1920) as u32;
            let h = args["height"].as_u64().unwrap_or(1080) as u32;

            let mut lock = session.photo_canvas.lock().unwrap();
            *lock = Some(dreamcraft_primitives::tool::PhotoCanvas::new(name, w, h));
            ToolResult::text(format!("Created Photoshop raster canvas '{}' ({}×{} px)", name, w, h))
        }

        // --- 10. VECTOR TOOLS ---
        "vector_create" => {
            let name = args["name"].as_str().unwrap_or("Artwork 01");
            let w = args["width"].as_f64().unwrap_or(1920.0);
            let h = args["height"].as_f64().unwrap_or(1080.0);

            let mut lock = session.vector_doc.lock().unwrap();
            *lock = Some(dreamcraft_primitives::tool::VectorDocument::new(name, w, h));
            ToolResult::text(format!("Created Illustrator vector document '{}' ({}×{} pt)", name, w, h))
        }

        "vector_add_rect" => {
            let x = args["x"].as_f64().unwrap_or(0.0);
            let y = args["y"].as_f64().unwrap_or(0.0);
            let w = args["width"].as_f64().unwrap_or(100.0);
            let h = args["height"].as_f64().unwrap_or(100.0);
            let fill = args["fill_hex"].as_str().and_then(Color::from_hex);

            let mut lock = session.vector_doc.lock().unwrap();
            if let Some(v) = lock.as_mut() {
                let id = v.add_rect("Rect", x, y, w, h, fill, Some(Color::BLACK));
                ToolResult::text(format!("Added vector rect (id: {})", id.short_str()))
            } else {
                ToolResult::error("No active Vector Document.")
            }
        }

        "vector_export_svg" => {
            let lock = session.vector_doc.lock().unwrap();
            if let Some(v) = lock.as_ref() {
                ToolResult::text(v.to_svg())
            } else {
                ToolResult::error("No active Vector Document.")
            }
        }

        // --- 11. CAD TOOLS ---
        "cad_create" => {
            let name = args["name"].as_str().unwrap_or("Drawing 01");
            let mut lock = session.cad_drawing.lock().unwrap();
            *lock = Some(dreamcraft_primitives::tool::CadDrawing::new(name));
            ToolResult::text(format!("Created AutoCAD 2D drafting file '{}'", name))
        }

        "cad_add_line" => {
            let x1 = args["x1"].as_f64().unwrap_or(0.0);
            let y1 = args["y1"].as_f64().unwrap_or(0.0);
            let x2 = args["x2"].as_f64().unwrap_or(100.0);
            let y2 = args["y2"].as_f64().unwrap_or(100.0);

            let mut lock = session.cad_drawing.lock().unwrap();
            if let Some(cad) = lock.as_mut() {
                cad.add_line(0, x1, y1, x2, y2);
                ToolResult::text(format!("Added CAD line from ({}, {}) to ({}, {})", x1, y1, x2, y2))
            } else {
                ToolResult::error("No active CAD Drawing.")
            }
        }

        "cad_export_dxf" => {
            let lock = session.cad_drawing.lock().unwrap();
            if let Some(cad) = lock.as_ref() {
                ToolResult::text(cad.to_dxf())
            } else {
                ToolResult::error("No active CAD Drawing.")
            }
        }

        // --- 12. PDF TOOLS ---
        "pdf_create" => {
            let title = args["title"].as_str().unwrap_or("Document 01");
            let mut lock = session.pdf_doc.lock().unwrap();
            *lock = Some(dreamcraft_primitives::tool::PdfDocument::new(title));
            ToolResult::text(format!("Created Acrobat PDF document '{}'", title))
        }

        // --- 13. AI GENERATION TOOLS ---
        "ai_generate_image" => {
            let prompt = args["prompt"].as_str().unwrap_or("Abstract artwork");
            let w = args["width"].as_u64().unwrap_or(1024) as u32;
            let h = args["height"].as_u64().unwrap_or(768) as u32;

            match CreativePipeline::generate_image(prompt, w, h).await {
                Ok(uri) => ToolResult::json(&json!({ "image_uri": uri, "prompt": prompt })),
                Err(e) => ToolResult::error(e.to_string()),
            }
        }

        "ai_generate_video" => {
            let prompt = args["prompt"].as_str().unwrap_or("Cinematic scenery");
            let dur = args["duration_seconds"].as_f64().unwrap_or(4.0);

            match CreativePipeline::generate_video(prompt, dur).await {
                Ok(uri) => ToolResult::json(&json!({ "video_uri": uri, "duration": dur })),
                Err(e) => ToolResult::error(e.to_string()),
            }
        }

        // --- 14. CROSS-APP COMPOSITION TOOLS ---
        "compose_illustrated_document" => {
            let topic = args["topic"].as_str().unwrap_or("Artificial Intelligence");
            match CreativePipeline::compose_illustrated_report(topic).await {
                Ok(doc) => {
                    let md = doc.to_markdown();
                    let mut lock = session.word_doc.lock().unwrap();
                    *lock = Some(doc);
                    ToolResult::text(format!("Generated complete illustrated document for '{}':\n\n{}", topic, md))
                }
                Err(e) => ToolResult::error(e.to_string()),
            }
        }

        "compose_pitch_deck" => {
            let title = args["title"].as_str().unwrap_or("NextGen Pitch");
            let num_slides = args["num_slides"].as_u64().unwrap_or(4) as usize;
            match CreativePipeline::compose_pitch_deck(title, num_slides).await {
                Ok(deck) => {
                    let slide_count = deck.slides.len();
                    let mut lock = session.deck.lock().unwrap();
                    *lock = Some(deck);
                    ToolResult::text(format!("Generated complete pitch presentation '{}' with {} slides.", title, slide_count))
                }
                Err(e) => ToolResult::error(e.to_string()),
            }
        }

        "compose_ai_film" => {
            let title = args["title"].as_str().unwrap_or("Cinematic Short");
            let scenes_arr = match args["scenes"].as_array() {
                Some(arr) => arr,
                None => return ToolResult::error("Missing 'scenes' array."),
            };
            let scenes: Vec<&str> = scenes_arr.iter().filter_map(|v| v.as_str()).collect();

            match CreativePipeline::compose_ai_film(title, scenes).await {
                Ok(seq) => {
                    let edl = seq.export_edl();
                    let mut lock = session.film_seq.lock().unwrap();
                    *lock = Some(seq);
                    ToolResult::text(format!("Generated movie sequence '{}' with {} scenes!\n\nEDL Preview:\n{}", title, scenes_arr.len(), edl))
                }
                Err(e) => ToolResult::error(e.to_string()),
            }
        }

        "grid_sort_range" => {
            let range = args["range"].as_str().unwrap_or("A1:D10");
            let offset = args["sort_col_offset"].as_u64().unwrap_or(0) as u32;
            let asc = args["ascending"].as_bool().unwrap_or(true);
            let has_hdr = args["has_header"].as_bool().unwrap_or(false);

            let mut lock = session.workbook.lock().unwrap();
            if let Some(wb) = lock.as_mut() {
                let sheet = wb.active_sheet_mut();
                match sheet.sort_range(range, offset, asc, has_hdr) {
                    Ok(_) => ToolResult::text(format!("Sorted range {} by column offset {} (ascending: {})", range, offset, asc)),
                    Err(e) => ToolResult::error(e.to_string()),
                }
            } else {
                ToolResult::error("No active Workbook.")
            }
        }

        "film_slip_clip" => {
            let cid_str = args["clip_id"].as_str().unwrap_or("");
            let delta = args["delta_seconds"].as_f64().unwrap_or(0.0);
            let tick = Tick::from_seconds(delta);

            let mut lock = session.film_seq.lock().unwrap();
            if let Some(seq) = lock.as_mut() {
                let id = Id::parse(cid_str).unwrap_or_default();
                match seq.slip(id, tick) {
                    Ok(_) => ToolResult::text(format!("Slipped clip {} by {:.2}s", cid_str, delta)),
                    Err(e) => ToolResult::error(e.to_string()),
                }
            } else {
                ToolResult::error("No active Film Sequence.")
            }
        }

        "film_roll_edit" => {
            let left_id_str = args["left_clip_id"].as_str().unwrap_or("");
            let right_id_str = args["right_clip_id"].as_str().unwrap_or("");
            let delta = args["delta_seconds"].as_f64().unwrap_or(0.0);
            let tick = Tick::from_seconds(delta);

            let mut lock = session.film_seq.lock().unwrap();
            if let Some(seq) = lock.as_mut() {
                let left_id = Id::parse(left_id_str).unwrap_or_default();
                let right_id = Id::parse(right_id_str).unwrap_or_default();
                match seq.roll(left_id, right_id, tick) {
                    Ok(_) => ToolResult::text(format!("Rolled edit boundary between {} and {} by {:.2}s", left_id_str, right_id_str, delta)),
                    Err(e) => ToolResult::error(e.to_string()),
                }
            } else {
                ToolResult::error("No active Film Sequence.")
            }
        }

        "film_set_speed" => {
            let cid_str = args["clip_id"].as_str().unwrap_or("");
            let speed = args["speed"].as_f64().unwrap_or(1.0);
            let ripple = args["ripple"].as_bool().unwrap_or(true);

            let mut lock = session.film_seq.lock().unwrap();
            if let Some(seq) = lock.as_mut() {
                let id = Id::parse(cid_str).unwrap_or_default();
                match seq.set_speed(id, speed, ripple) {
                    Ok(_) => ToolResult::text(format!("Set clip {} speed to {:.2}x (ripple: {})", cid_str, speed, ripple)),
                    Err(e) => ToolResult::error(e.to_string()),
                }
            } else {
                ToolResult::error("No active Film Sequence.")
            }
        }

        "film_add_transition" => {
            let tid_str = args["track_id"].as_str().unwrap_or("");
            let kind_str = args["kind"].as_str().unwrap_or("CrossDissolve");
            let start = Tick::from_seconds(args["start_seconds"].as_f64().unwrap_or(0.0));
            let dur = Tick::from_seconds(args["duration_seconds"].as_f64().unwrap_or(1.0));

            let kind = match kind_str {
                "DipToBlack" => TransitionKind::DipToBlack,
                "DipToWhite" => TransitionKind::DipToWhite,
                "WipeLeft" => TransitionKind::WipeLeft,
                "WipeRight" => TransitionKind::WipeRight,
                _ => TransitionKind::CrossDissolve,
            };

            let mut lock = session.film_seq.lock().unwrap();
            if let Some(seq) = lock.as_mut() {
                let tid = Id::parse(tid_str).unwrap_or_default();
                match seq.add_transition(tid, kind, start, dur, None, None) {
                    Ok(id) => ToolResult::text(format!("Added {:?} transition (ID: {}) to track {}", kind, id, tid_str)),
                    Err(e) => ToolResult::error(e.to_string()),
                }
            } else {
                ToolResult::error("No active Film Sequence.")
            }
        }

        "deck_set_theme" => {
            let theme_str = args["theme"].as_str().unwrap_or("Harbor");
            let theme_kind = match theme_str {
                "Ember" => DeckThemeKind::Ember,
                "Meadow" => DeckThemeKind::Meadow,
                "Nocturne" => DeckThemeKind::Nocturne,
                "Paper" => DeckThemeKind::Paper,
                "Slate" => DeckThemeKind::Slate,
                _ => DeckThemeKind::Harbor,
            };

            let mut lock = session.deck.lock().unwrap();
            if let Some(deck) = lock.as_mut() {
                deck.set_theme(theme_kind);
                ToolResult::text(format!("Applied '{}' theme palette to presentation.", theme_str))
            } else {
                ToolResult::error("No active Presentation.")
            }
        }

        "deck_add_metric_slide" => {
            let title = args["title"].as_str().unwrap_or("Performance KPI");
            let metric_val = args["metric_value"].as_str().unwrap_or("$10M+");
            let metric_label = args["metric_label"].as_str().unwrap_or("ARR");
            let desc = args["description"].as_str().unwrap_or("Description");

            let mut lock = session.deck.lock().unwrap();
            if let Some(deck) = lock.as_mut() {
                let idx = deck.add_metric_slide(title, metric_val, metric_label, desc);
                ToolResult::text(format!("Added Key Metric slide (Slide #{})", idx + 1))
            } else {
                ToolResult::error("No active Presentation.")
            }
        }

        "light_apply_preset" => {
            let preset = args["preset"].as_str().unwrap_or("cinematic_warm");
            let mut lock = session.light_photo.lock().unwrap();
            if let Some(photo) = lock.as_mut() {
                photo.apply_preset(preset);
                ToolResult::text(format!("Applied develop preset '{}' to {}", preset, photo.path))
            } else {
                ToolResult::error("No active LightCraft photo.")
            }
        }

        "sound_add_automation" => {
            let t_idx = args["track_index"].as_u64().unwrap_or(0) as usize;
            let param = args["parameter"].as_str().unwrap_or("volume");
            let secs = args["seconds"].as_f64().unwrap_or(0.0);
            let val = args["value"].as_f64().unwrap_or(0.0);
            let tick = Tick::from_seconds(secs);

            let mut lock = session.sound_project.lock().unwrap();
            if let Some(proj) = lock.as_mut() {
                if let Some(track) = proj.tracks.get(t_idx) {
                    let tid = track.id;
                    let tname = track.name.clone();
                    match proj.add_automation_point(tid, param, tick, val) {
                        Ok(_) => ToolResult::text(format!("Added {} automation at {:.2}s = {:.2} on track {}", param, secs, val, tname)),
                        Err(e) => ToolResult::error(e.to_string()),
                    }
                } else {
                    ToolResult::error("Track index out of bounds.")
                }
            } else {
                ToolResult::error("No active Audio Project.")
            }
        }

        "pdf_rotate_page" => {
            let p_idx = args["page_index"].as_u64().unwrap_or(0) as usize;
            let deg = args["degrees"].as_u64().unwrap_or(90) as u16;

            let mut lock = session.pdf_doc.lock().unwrap();
            if let Some(doc) = lock.as_mut() {
                match doc.rotate_page(p_idx, deg) {
                    Ok(_) => ToolResult::text(format!("Rotated page {} by {} degrees", p_idx + 1, deg)),
                    Err(e) => ToolResult::error(e.to_string()),
                }
            } else {
                ToolResult::error("No active PDF Document.")
            }
        }

        "pdf_set_watermark" => {
            let text = args["text"].as_str().unwrap_or("CONFIDENTIAL");
            let mut lock = session.pdf_doc.lock().unwrap();
            if let Some(doc) = lock.as_mut() {
                doc.set_watermark(text);
                ToolResult::text(format!("Applied watermark '{}' across all PDF pages.", text))
            } else {
                ToolResult::error("No active PDF Document.")
            }
        }

        "dream_inspect" => {
            let domain = args["domain"].as_str().unwrap_or("all");
            let inspection = session.inspect_semantic(domain);
            ToolResult::json(&inspection)
        }

        "dream_register_asset" => {
            let name = args["name"].as_str().unwrap_or("Untitled Asset");
            let kind_str = args["kind"].as_str().unwrap_or("Image");
            let uri = args["uri"].as_str().unwrap_or("");

            let kind = match kind_str {
                "Video" => dreamcraft_core::AssetKind::Video,
                "Audio" => dreamcraft_core::AssetKind::Audio,
                "Vector" => dreamcraft_core::AssetKind::Vector,
                "Dataset" => dreamcraft_core::AssetKind::Dataset,
                "Document" => dreamcraft_core::AssetKind::Document,
                "Model3D" => dreamcraft_core::AssetKind::Model3D,
                _ => dreamcraft_core::AssetKind::Image,
            };

            let id = session.register_asset(name, kind, uri);
            ToolResult::text(format!("Registered asset '{}' (ID: {}) into project catalog.", name, id))
        }

        "dream_history" => {
            let proj = session.project.lock().unwrap();
            let history_json = serde_json::to_value(&proj.command_history.past).unwrap_or_default();
            ToolResult::json(&history_json)
        }

        _ => ToolResult::error(format!("Unknown tool: {}", name)),
    }
}
