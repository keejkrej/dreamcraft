use crate::state::DreamSession;
use dreamcraft_core::{ChangeEvent, Color, Id, Rect, Tick};
use dreamcraft_primitives::ai::CreativePipeline;
use dreamcraft_primitives::tool::{
    CellCoord, HeadingLevel, SlideLayout, TrackKind, WordDocument, Workbook,
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
        // --- WORD PRIMITIVES ---
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
                    "bold": { "type": "boolean", "description": "Whether the text is bold" },
                    "italic": { "type": "boolean", "description": "Whether the text is italic" }
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
                    "has_header": { "type": "boolean", "description": "Whether the first row is a header" }
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
                    "format": { "type": "string", "enum": ["markdown", "text"], "description": "Output format" }
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
                    "find": { "type": "string", "description": "Text to find" },
                    "replace_with": { "type": "string", "description": "Replacement text" }
                },
                "required": ["find", "replace_with"]
            })
        ),

        // --- GRID / SPREADSHEET PRIMITIVES ---
        tool_spec(
            "grid_new_workbook",
            "New Excel Workbook",
            "Create a new blank Excel-style workbook.",
            json!({
                "type": "object",
                "properties": { "title": { "type": "string", "description": "Workbook title" } },
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
                "properties": {
                    "range": { "type": "string", "description": "Range expression, e.g. 'A1:D10'" }
                },
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
                    "start": { "type": "string", "description": "Top-left cell coordinate, e.g. 'A1'" },
                    "values": {
                        "type": "array",
                        "items": { "type": "array", "items": { "type": "string" } },
                        "description": "2D array of values"
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

        // --- FILM / VIDEO EDITING PRIMITIVES ---
        tool_spec(
            "film_new_sequence",
            "New Video Sequence",
            "Create a new video editing sequence / timeline.",
            json!({
                "type": "object",
                "properties": {
                    "name": { "type": "string", "description": "Sequence name" },
                    "fps": { "type": "number", "description": "Frame rate (e.g. 24.0, 30.0, 60.0)" }
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
                    "track_kind": { "type": "string", "enum": ["video", "audio"], "description": "Track kind" },
                    "track_index": { "type": "integer", "description": "Track index (0 for V1/A1, 1 for V2/A2)" },
                    "name": { "type": "string", "description": "Clip name" },
                    "source_path": { "type": "string", "description": "File path or URL of media" },
                    "start_seconds": { "type": "number", "description": "Timeline start position in seconds" },
                    "duration_seconds": { "type": "number", "description": "Clip duration in seconds" }
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
                    "cut_time_seconds": { "type": "number", "description": "Cut point in seconds" }
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
                "properties": {
                    "clip_id": { "type": "string", "description": "Clip UUID" }
                },
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
                    "clip_id": { "type": "string", "description": "Clip UUID" },
                    "delta_seconds": { "type": "number", "description": "Seconds to extend (+) or shorten (-)" },
                    "trim_tail": { "type": "boolean", "description": "True for tail trim, false for head trim" }
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

        // --- DECK / SLIDE PRIMITIVES ---
        tool_spec(
            "deck_new_presentation",
            "New Slide Presentation",
            "Create a new PowerPoint-style slide presentation.",
            json!({
                "type": "object",
                "properties": { "title": { "type": "string", "description": "Presentation title" } },
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
                    "title": { "type": "string", "description": "Slide title" },
                    "subtitle": { "type": "string", "description": "Optional subtitle" }
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
                    "slide_index": { "type": "integer", "description": "Slide index (0-based)" },
                    "text": { "type": "string", "description": "Bullet text" },
                    "y_offset": { "type": "number", "description": "Vertical offset in points" }
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
                    "source_path": { "type": "string", "description": "Image file path or URL" },
                    "x": { "type": "number" },
                    "y": { "type": "number" },
                    "width": { "type": "number" },
                    "height": { "type": "number" }
                },
                "required": ["source_path", "x", "y", "width", "height"]
            })
        ),

        // --- CREATIVE PRIMITIVES (Photo, Vector, CAD, PDF) ---
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
                    "fill_hex": { "type": "string", "description": "Hex color e.g. #FF5500" }
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

        // --- AI PRIMITIVES ---
        tool_spec(
            "ai_generate_image",
            "AI Image Generation",
            "Generate an image using Fal.ai / Flux / SDXL or deterministic mock provider.",
            json!({
                "type": "object",
                "properties": {
                    "prompt": { "type": "string", "description": "Creative visual prompt" },
                    "width": { "type": "integer", "description": "Width in px (default 1024)" },
                    "height": { "type": "integer", "description": "Height in px (default 768)" }
                },
                "required": ["prompt"]
            })
        ),
        tool_spec(
            "ai_generate_video",
            "AI Video Generation",
            "Generate a video using Sora / Fal / Kling or mock provider.",
            json!({
                "type": "object",
                "properties": {
                    "prompt": { "type": "string", "description": "Video motion prompt" },
                    "duration_seconds": { "type": "number", "description": "Duration in seconds" }
                },
                "required": ["prompt"]
            })
        ),

        // --- CROSS-APP COMPOSITION PRIMITIVES ---
        tool_spec(
            "compose_illustrated_document",
            "Compose Illustrated Word Document",
            "Autonomous composition: Generates AI illustration, outlines sections, builds Word document with tables.",
            json!({
                "type": "object",
                "properties": { "topic": { "type": "string", "description": "Report topic" } },
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
                    "title": { "type": "string", "description": "Deck title" },
                    "num_slides": { "type": "integer", "description": "Number of slides to generate" }
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
                    "title": { "type": "string", "description": "Film title" },
                    "scenes": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "List of scene descriptions"
                    }
                },
                "required": ["title", "scenes"]
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

        // --- GRID / SPREADSHEET TOOLS ---
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

        // --- CREATIVE TOOLS ---
        "vector_export_svg" => {
            let lock = session.vector_doc.lock().unwrap();
            if let Some(v) = lock.as_ref() {
                ToolResult::text(v.to_svg())
            } else {
                ToolResult::error("No active Vector Document.")
            }
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

        "cad_export_dxf" => {
            let lock = session.cad_drawing.lock().unwrap();
            if let Some(cad) = lock.as_ref() {
                ToolResult::text(cad.to_dxf())
            } else {
                ToolResult::error("No active CAD Drawing.")
            }
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

        // --- AI TOOLS ---
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

        // --- COMPOSITION TOOLS ---
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

        _ => ToolResult::error(format!("Unknown tool: {}", name)),
    }
}
