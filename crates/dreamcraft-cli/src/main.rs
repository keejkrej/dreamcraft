use dreamcraft_core::Tick;
use dreamcraft_mcp::{DreamSession, McpServer};
use dreamcraft_primitives::ai::CreativePipeline;
use dreamcraft_primitives::tool::{
    CellCoord, HeadingLevel, SlideLayout, WordDocument, Workbook,
};
use std::io::{self, BufReader};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("help");

    match command {
        "mcp" => {
            eprintln!("✦ Starting DreamCraft MCP server on stdio...");
            let session = DreamSession::new();
            let mut server = McpServer::new(session);
            let stdin = io::stdin();
            let reader = BufReader::new(stdin);
            let stdout = io::stdout();
            server.run_stdio(reader, stdout).await;
        }

        "demo" => {
            println!("============================================================");
            println!("✦ DreamCraft: Unified AI-Native Creative & Office Suite ✦");
            println!("============================================================\n");

            // 1. Word Document Primitives
            println!("[1/5] Executing Word Document Tool Primitives...");
            let mut doc = WordDocument::new("DreamCraft Autonomous Strategy");
            doc.add_heading(HeadingLevel::Heading1, "Executive Summary");
            doc.add_paragraph("DreamCraft consolidates the *craft suite into pure Rust deterministic primitives.");
            doc.insert_table(
                vec![
                    vec!["Engine".into(), "Domain".into(), "Primitives".into()],
                    vec!["WordCraft".into(), "Documents & Typography".into(), "docx, formatting, tables".into()],
                    vec!["GridCraft".into(), "Spreadsheets".into(), "formulas, calculation, xlsx".into()],
                    vec!["FilmCraft".into(), "Video Timeline".into(), "cuts, trims, ripple delete".into()],
                    vec!["DeckCraft".into(), "Presentations".into(), "slides, shapes, layouts".into()],
                    vec!["ArtCraft".into(), "Generative AI".into(), "Fal, Sora, Midjourney, WorldLabs".into()],
                ],
                true,
            );
            println!("✓ Word Document Markdown Preview:\n{}", doc.to_markdown());

            // 2. Spreadsheet Grid Primitives + Formula Evaluator
            println!("[2/5] Executing Spreadsheet Grid Tool Primitives...");
            let mut wb = Workbook::new("Financial Projection");
            wb.active_sheet_mut().write_range(
                "A1",
                vec![
                    vec!["Category".into(), "Q1".into(), "Q2".into(), "Q3".into()],
                    vec!["Revenue".into(), "100".into(), "200".into(), "350".into()],
                    vec!["Expenses".into(), "40".into(), "70".into(), "110".into()],
                    vec!["Net".into(), "60".into(), "130".into(), "240".into()],
                    vec!["Total Net".into(), "=SUM(B4:D4)".into(), "".into(), "".into()],
                ],
            )?;
            let total_net = wb.active_sheet().get_cell(&CellCoord::parse("B5").unwrap()).computed.display_string();
            println!("✓ Evaluated Formula '=SUM(B4:D4)': {}", total_net);
            println!("✓ Spreadsheet CSV Preview:\n{}", wb.active_sheet().to_csv());

            // 3. Video Editing Timeline Primitives (Premiere clone)
            println!("[3/5] Executing Movie Cutting Tool Primitives (FilmCraft)...");
            let mut seq = dreamcraft_primitives::tool::film::Sequence::new("Feature Film Sequence");
            let v1 = seq.video_tracks[0].id;
            let clip1 = seq.insert_clip(v1, "A-Roll Interview", "media/interview.mp4", Tick::ZERO, Tick::from_seconds(10.0))?;
            println!("✓ Inserted clip '{}' from 0.0s to 10.0s", clip1.short_str());

            // Razor cut at 4.5s
            let (part_a, part_b) = seq.razor_cut(v1, Tick::from_seconds(4.5))?;
            println!("✓ Executed razor cut at 4.50s into clips [{}] and [{}]", part_a.short_str(), part_b.short_str());

            // Ripple delete part B
            let shift = seq.ripple_delete(part_b)?;
            println!("✓ Executed ripple delete on clip [{}], closing gap by {:.2}s", part_b.short_str(), shift.to_seconds());
            println!("✓ EDL Output:\n{}", seq.export_edl());

            // 4. Slide Deck Primitives (DeckCraft)
            println!("[4/9] Executing Slide Deck Primitives (DeckCraft)...");
            let mut deck = dreamcraft_primitives::tool::Presentation::new("AI Pitch Deck");
            let s_idx = deck.add_slide(SlideLayout::TitleAndContent, "Autonomous Architecture");
            if let Some(slide) = deck.slides.get_mut(s_idx) {
                slide.add_bullet("Pure Rust deterministic execution engines", 0.0);
                slide.add_bullet("Fully scriptable via Model Context Protocol (MCP)", 40.0);
                slide.add_bullet("Zero human editing chrome needed", 80.0);
            }
            println!("✓ Created Presentation with {} slides.", deck.slides.len());

            // 5. SoundCraft / DAW Mixing Primitives (Pro Tools clone)
            println!("[5/9] Executing DAW Audio Mixing Primitives (SoundCraft)...");
            let mut sound = dreamcraft_primitives::tool::SoundProject::new("Cinematic Score Session");
            let vocal_id = sound.tracks[0].id;
            sound.set_fader(vocal_id, -3.5)?;
            sound.set_pan(vocal_id, 0.0)?;
            sound.add_effect(
                vocal_id,
                dreamcraft_primitives::tool::AudioEffect::Compressor(
                    dreamcraft_primitives::tool::Compressor::default(),
                ),
            )?;
            println!("✓ Audio Session configured: {} tracks with compressor & fader control.", sound.tracks.len());

            // 6. LightCraft / Photo RAW Develop Primitives (Lightroom clone)
            println!("[6/9] Executing RAW Photo Develop Primitives (LightCraft)...");
            let mut photo = dreamcraft_primitives::tool::LightPhoto::new("landscape_sunset.dng");
            photo.set_exposure(0.65);
            photo.set_white_balance(12.0, -4.0);
            photo.set_highlights_shadows(-30.0, 45.0);
            println!("✓ Developed RAW photo: Exp={:.2} EV, Temp={:.1}, Highlights={:.1}",
                photo.settings.tone.exposure, photo.settings.wb.temperature, photo.settings.tone.highlights);

            // 7. EffectCraft / Motion Graphics & VFX Primitives (After Effects clone)
            println!("[7/9] Executing Motion Graphics Keyframing Primitives (EffectCraft)...");
            let mut comp = dreamcraft_primitives::tool::Composition::new("Title Sequence Comp", 1920, 1080, 24.0, 120);
            comp.add_solid_layer("Background Solid", dreamcraft_core::Color::rgb(10, 15, 30));
            let text_layer_id = comp.add_text_layer("Title Text", "DREAMCRAFT");
            println!("✓ Created Composition (1920x1080 @ 24fps) with layers [Text: {}]", text_layer_id.short_str());

            // 8. DesignCraft / Multi-column Publishing Primitives (InDesign clone)
            println!("[8/9] Executing Desktop Publishing Primitives (DesignCraft)...");
            let mut pub_doc = dreamcraft_primitives::tool::DesignDocument::new("Quarterly Magazine");
            pub_doc.add_text_frame(0, dreamcraft_core::Rect::new(36.0, 36.0, 540.0, 720.0), "Editorial article text body flowing through pages.");
            println!("✓ Publication created: {} pages with threaded text frames.", pub_doc.pages.len());

            // 9. Cross-App AI Composition Workflow
            println!("[9/9] Executing Cross-App AI Composition Workflow (ArtCraft)...");
            let generated_doc = CreativePipeline::compose_illustrated_report("Quantum Computing in 2026").await?;
            println!("✓ Composed illustrated document with {} blocks.", generated_doc.blocks.len());

            println!("\n✦ All 12 Craft primitives + AI Studio executed successfully! ✦");
        }

        "tools" => {
            println!("Available DreamCraft MCP Tools:\n");
            let tools = dreamcraft_mcp::tool_definitions();
            if let Some(arr) = tools.as_array() {
                for t in arr {
                    let name = t["name"].as_str().unwrap_or_default();
                    let desc = t["description"].as_str().unwrap_or_default();
                    println!("  • {:<30} {}", name, desc);
                }
            }
        }

        "realize" => {
            let bundle = args.get(2).map(|s| s.as_str()).unwrap_or("startup");
            let mut catalog = dreamcraft_core::AssetCatalog::new();
            let mut history = dreamcraft_core::command::CommandHistory::new(50);

            match bundle {
                "startup" => {
                    println!("============================================================");
                    println!("✦ Realizing Autonomous Startup Venture Dream ✦");
                    println!("============================================================\n");
                    let blueprint = dreamcraft_primitives::compose::StartupLaunchBlueprint::default();
                    let artifacts = dreamcraft_primitives::compose::DreamRealizer::realize_startup(&blueprint, &mut catalog, &mut history)?;
                    println!("✓ Brand Logo Vector SVG registered (ID: {})", artifacts.logo_asset_id);
                    println!("✓ Financial Model (GridCraft): 3-Year ARR projection with formula recalculation");
                    println!("✓ Pitch Deck (DeckCraft): {} slides formatted with Harbor theme", artifacts.pitch_deck.slides.len());
                    println!("✓ Executive Memo (WordCraft): {} blocks with embedded financial table", artifacts.executive_memo.blocks.len());
                    println!("✓ Investor Dossier (PdfCraft): {} pages with watermark and signature field", artifacts.investor_dossier.pages.len());
                    println!("✓ Live Reactive Bindings: Synchronized GridCraft financial cell to DeckCraft KPI metric card\n");
                    println!("Artifacts successfully compiled into workspace!");
                }
                "film" => {
                    println!("============================================================");
                    println!("✦ Realizing Autonomous Cinematic Production Dream ✦");
                    println!("============================================================\n");
                    let blueprint = dreamcraft_primitives::compose::CinematicProductionBlueprint::default();
                    let artifacts = dreamcraft_primitives::compose::DreamRealizer::realize_film(&blueprint, &mut catalog, &mut history)?;
                    println!("✓ Visual Storyboard (DeckCraft): {} scene prompt cards", artifacts.storyboard.slides.len());
                    println!("✓ NLE Timeline (FilmCraft): {} video tracks, transitions, dialogue & music stems", artifacts.timeline.video_tracks.len());
                    println!("✓ DAW Audio Mastering (SoundCraft): {} stems with parametric EQ & mastering limiter", artifacts.sound_mix.tracks.len());
                    println!("✓ Motion Intro (EffectCraft): 3D typography intro with motion blur");
                    println!("✓ SMPTE EDL Export:\n{}", artifacts.edl_export);
                }
                "editorial" => {
                    println!("============================================================");
                    println!("✦ Realizing Autonomous Editorial Publication Dream ✦");
                    println!("============================================================\n");
                    let blueprint = dreamcraft_primitives::compose::EditorialPublicationBlueprint::default();
                    let artifacts = dreamcraft_primitives::compose::DreamRealizer::realize_editorial(&blueprint, &mut catalog, &mut history)?;
                    println!("✓ Manuscript (WordCraft): {} blocks with full typography hierarchy", artifacts.manuscript.blocks.len());
                    println!("✓ Readership Dataset (GridCraft): Readership metrics sheet with growth formulas");
                    println!("✓ Multi-page Layout (DesignCraft): {} pages with threaded text frames", artifacts.layout.pages.len());
                    println!("✓ Prepress Dossier (PdfCraft): {} pages ready for distribution\n", artifacts.print_pdf.pages.len());
                }
                "engineering" => {
                    println!("============================================================");
                    println!("✦ Realizing Autonomous Engineering Specification Dream ✦");
                    println!("============================================================\n");
                    let blueprint = dreamcraft_primitives::compose::EngineeringSpecBlueprint::default();
                    let artifacts = dreamcraft_primitives::compose::DreamRealizer::realize_engineering(&blueprint, &mut catalog, &mut history)?;
                    println!("✓ CAD Blueprint (CadCraft): 2D technical drawing with dimensions & DXF export (ID: {})", artifacts.dxf_asset_id);
                    println!("✓ Architecture Diagram (VectorCraft): System bus topology");
                    println!("✓ Bill of Materials (GridCraft): Live cost rollup spreadsheet with SUM formulas");
                    println!("✓ Technical Specification (WordCraft): Engineering spec documentation\n");
                }
                _ => {
                    println!("Unknown dream bundle: {}. Available: startup, film, editorial, engineering", bundle);
                }
            }
        }

        _ => {
            println!("DreamCraft CLI - All-in-one AI-Native Creative Suite");
            println!("\nUsage:");
            println!("  dreamcraft mcp                      Start the Model Context Protocol (MCP) server on stdio");
            println!("  dreamcraft realize [startup|film|editorial|engineering]  Autonomously realize high-level multi-modal dreams");
            println!("  dreamcraft demo                     Run end-to-end demo of all tool, AI, and composition primitives");
            println!("  dreamcraft tools                    List all available MCP tools");
        }
    }

    Ok(())
}
