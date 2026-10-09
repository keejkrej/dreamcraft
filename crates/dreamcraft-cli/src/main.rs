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

            // 4. Slide Deck Primitives (PowerPoint clone)
            println!("[4/5] Executing Slide Deck Primitives (DeckCraft)...");
            let mut deck = dreamcraft_primitives::tool::Presentation::new("AI Pitch Deck");
            let s_idx = deck.add_slide(SlideLayout::TitleAndContent, "Autonomous Architecture");
            if let Some(slide) = deck.slides.get_mut(s_idx) {
                slide.add_bullet("Pure Rust deterministic execution engines", 0.0);
                slide.add_bullet("Fully scriptable via Model Context Protocol (MCP)", 40.0);
                slide.add_bullet("Zero human editing chrome needed", 80.0);
            }
            println!("✓ Created Presentation with {} slides.", deck.slides.len());

            // 5. Cross-App AI Composition Workflow
            println!("[5/5] Executing Cross-App AI Composition Workflow (ArtCraft)...");
            let generated_doc = CreativePipeline::compose_illustrated_report("Quantum Computing in 2026").await?;
            println!("✓ Composed illustrated document with {} blocks.", generated_doc.blocks.len());

            println!("\n✦ All DreamCraft primitives executed successfully! ✦");
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

        _ => {
            println!("DreamCraft CLI - All-in-one AI-Native Creative Suite");
            println!("\nUsage:");
            println!("  dreamcraft mcp     Start the Model Context Protocol (MCP) server on stdio");
            println!("  dreamcraft demo    Run end-to-end demo of all tool, AI, and composition primitives");
            println!("  dreamcraft tools   List all available MCP tools");
        }
    }

    Ok(())
}
