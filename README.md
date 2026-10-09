# ✦ DreamCraft

**DreamCraft** is an all-in-one, AI-native creative and office suite built in pure Rust.

It distills the core primitives from [storytold's](https://github.com/storytold) *craft series (`wordcraft`, `gridcraft`, `deckcraft`, `filmcraft`, `photocraft`, `vectorcraft`, `cadcraft`, `pdfcraft`, `artcraft`, etc.) into a unified single repository.

Instead of installing 12+ separate desktop applications with complex manual ribbons, toolbars, and menus, **DreamCraft exposes pure execution primitives to AI agents over the Model Context Protocol (MCP)**, complemented by a zero-editing presentation preview layer for human inspection.

---

## 🏛 Architecture: The Three Pillars

DreamCraft splits creative and productivity software into three distinct categories of primitives:

```
                                  AI Agent
                                     │ (MCP over JSON-RPC)
                                     ▼
┌────────────────────────────────────────────────────────────────────────┐
│                              DreamCraft                                │
│                                                                        │
│   1. Tool Primitives (Deterministic Execution Cores)                   │
│      ├── WordCraft: Documents, headings, runs, formatting, tables     │
│      ├── GridCraft: Spreadsheets, cells, formula AST (SUM, AVG), CSV   │
│      ├── FilmCraft: Timeline, tracks, clips, razor cuts, ripple delete │
│      ├── DeckCraft: Slides, shapes, bullets, themes, media            │
│      ├── SoundCraft: Pro Tools DAW mixer, faders, pan, EQ, compressor  │
│      ├── LightCraft: Lightroom RAW develop, exposure, temp, tone curve │
│      ├── EffectCraft: After Effects comp, layers, keyframe transforms  │
│      ├── DesignCraft: InDesign multi-column spreads & text frames      │
│      ├── PhotoCraft: Multi-layer raster canvas, blend modes, adjust   │
│      ├── VectorCraft: Shapes, beziers, SVG generation                 │
│      ├── CADCraft: 2D drafting, lines, circles, DXF export            │
│      └── PdfCraft: Page merging, splitting, text extraction           │
│                                                                        │
│   2. AI Primitives (Non-Deterministic Generative Clients)              │
│      ├── Fal.ai (Flux Schnell/Dev, SDXL fast diffusion)               │
│      ├── OpenAI Sora (Video generation)                               │
│      ├── Midjourney (Image generation)                                │
│      ├── WorldLabs (3D Gaussian Splatting / world generation)         │
│      ├── Grok (Multimodal creative reasoning)                         │
│      ├── GMICloud (High throughput GPU model inference)               │
│      └── Kinovi (Generative motion video camera control)              │
│                                                                        │
│   3. UI Primitives (Presentation & Preview Layer)                      │
│      ├── DocPreview: Paginated typography and document layout         │
│      ├── GridPreview: Virtualized spreadsheet grid & formula bar      │
│      ├── DeckPreview: 16:9 slide stage & deck carousel                │
│      ├── FilmPreview: Video & audio tracks, clips, scrubber           │
│      └── CanvasPreview: Pan/zoom viewport for Photo/Vector/CAD        │
└────────────────────────────────────────────────────────────────────────┘
                                     │
                                     ▼
                            Human User (Viewer)
                      (Zero manual edits; inspect only)
```

---

## 📦 Workspace Crates

| Crate | Purpose |
|---|---|
| [`dreamcraft-core`](crates/dreamcraft-core) | Common types: UUID `Id`, `Color`, `Rect`, 254G ticks/sec `Tick`, `TimeRange`, event bus, error types. |
| [`dreamcraft-primitives`](crates/dreamcraft-primitives) | The 3 core pillars: deterministic `tool` engines, non-deterministic `ai` clients, and `egui` presentation `ui` widgets. |
| [`dreamcraft-mcp`](crates/dreamcraft-mcp) | Model Context Protocol server implementing JSON-RPC stdio protocol with 30+ tools. |
| [`dreamcraft-studio`](crates/dreamcraft-studio) | Native `egui` desktop viewer showing live document, spreadsheet, slide, and timeline previews. |
| [`dreamcraft-cli`](crates/dreamcraft-cli) | CLI runner for launching MCP, running demos, or inspecting tools. |

---

## 🚀 Quick Start

### 1. Run the End-to-End Demo
Executes document creation, spreadsheet formula calculation, video timeline razor cutting/ripple deleting, slide deck generation, and AI composition:
```bash
cargo run -p dreamcraft-cli -- demo
```

### 2. List Available MCP Tools
```bash
cargo run -p dreamcraft-cli -- tools
```

### 3. Launch DreamCraft as an MCP Server
To connect DreamCraft to an AI agent (Claude Desktop, Antigravity, Cursor, etc.):
```bash
cargo run -p dreamcraft-cli -- mcp
```

Add to your MCP configuration (e.g. `claude_desktop_config.json`):
```json
{
  "mcpServers": {
    "dreamcraft": {
      "command": "/path/to/dreamcraft/target/debug/dreamcraft",
      "args": ["mcp"]
    }
  }
}
```

### 4. Launch the Presentation Studio Viewer
```bash
cargo run -p dreamcraft-studio
```

---

## 🛠 Available MCP Tool Primitives

### 📄 Word Processing (`word_*`)
- `word_new_document`: Create new Word document with title.
- `word_add_heading`: Add heading (level 1-4).
- `word_add_paragraph`: Add formatted body paragraph (bold, italic).
- `word_insert_table`: Insert 2D structured table.
- `word_search_replace`: Search and replace text occurrences.
- `word_get_content`: Return document as formatted Markdown or plain text.

### 📊 Spreadsheets (`grid_*`)
- `grid_new_workbook`: Create new Excel-style workbook.
- `grid_set_cell`: Enter value or formula (e.g. `coord="B5"`, `value="=SUM(B1:B4)"`).
- `grid_read_range`: Read 2D range of cells (e.g. `"A1:C10"`).
- `grid_write_range`: Write 2D array of rows starting at cell.
- `grid_export_csv`: Export active sheet to CSV.

### 🎬 Video Editing (`film_*`)
- `film_new_sequence`: Create timeline sequence with custom fps.
- `film_insert_clip`: Insert video or audio clip onto track.
- `film_razor_cut`: Split/cut clip at exact timestamp into two clips.
- `film_ripple_delete`: Delete clip and ripple shift all following clips backwards.
- `film_trim_clip`: Trim head or tail duration.
- `film_export_edl`: Export industry-standard EDL (Edit Decision List) for Premiere/DaVinci.

### 📽 Slide Presentations (`deck_*`)
- `deck_new_presentation`: Create slide deck with title.
- `deck_add_slide`: Add slide (TitleSlide, TitleAndContent, TwoColumns, Blank).
- `deck_add_bullet`: Add formatted bullet point text.
- `deck_add_image`: Insert image onto slide at `(x, y, w, h)`.

### 🎨 Creative Artwork (`photo_*`, `vector_*`, `cad_*`, `pdf_*`)
- `photo_create_canvas`: Multi-layer raster graphics canvas.
- `vector_create` & `vector_export_svg`: Illustrator-style vector shapes with SVG output.
- `cad_create` & `cad_export_dxf`: AutoCAD 2D drafting with DXF format export.
- `pdf_create`: Multi-page PDF assembly and text extraction.

### 🤖 Generative AI (`ai_*`) & Composition (`compose_*`)
- `ai_generate_image`: Fal.ai / Flux / SDXL image generation (with deterministic mock fallback).
- `ai_generate_video`: Sora / Kling / Fal video generation.
- `compose_illustrated_document`: AI generates illustration and writes full Word document with tables.
- `compose_pitch_deck`: AI generates multi-slide pitch deck with visuals.
- `compose_ai_film`: AI generates video scenes and cuts them onto the Film timeline with EDL export.

---

## 📄 License
MIT OR Apache-2.0
