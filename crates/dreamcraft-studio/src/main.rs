use dreamcraft_primitives::tool::{PhotoCanvas, Presentation, Sequence, VectorDocument, WordDocument, Workbook};
use dreamcraft_primitives::ui::{CanvasPreview, DeckPreview, DocPreview, FilmPreview, GridPreview};
use eframe::egui::{self, Color32, RichText, Vec2};
use std::sync::{Arc, Mutex};

#[derive(PartialEq, Eq)]
enum ActiveTab {
    Document,
    Spreadsheet,
    Presentation,
    Timeline,
    Canvas,
}

pub struct DreamCraftStudio {
    active_tab: ActiveTab,
    word_doc: Arc<Mutex<Option<WordDocument>>>,
    workbook: Arc<Mutex<Option<Workbook>>>,
    deck: Arc<Mutex<Option<Presentation>>>,
    film_seq: Arc<Mutex<Option<Sequence>>>,
    photo_canvas: Arc<Mutex<Option<PhotoCanvas>>>,
    vector_doc: Arc<Mutex<Option<VectorDocument>>>,
    status_log: Vec<String>,
}

impl DreamCraftStudio {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Sample starter demo state
        let mut sample_doc = WordDocument::new("DreamCraft Autonomous Strategy");
        sample_doc.add_heading(
            dreamcraft_primitives::tool::HeadingLevel::Heading1,
            "1. Core Tool Primitives",
        );
        sample_doc.add_paragraph(
            "DreamCraft synthesizes Word, Excel, PowerPoint, Premiere, Photoshop, Illustrator, CAD, and PDF into a unified Rust core driven exclusively by AI agents.",
        );
        sample_doc.insert_table(
            vec![
                vec!["App Core".into(), "Capability".into(), "AI Agent Interface".into()],
                vec!["WordCraft".into(), "Document layout & formatting".into(), "MCP / word_*".into()],
                vec!["GridCraft".into(), "Excel formulas & calculation".into(), "MCP / grid_*".into()],
                vec!["DeckCraft".into(), "Presentations & slide shapes".into(), "MCP / deck_*".into()],
                vec!["FilmCraft".into(), "Video timeline cuts & trims".into(), "MCP / film_*".into()],
                vec!["ArtCraft".into(), "Fal/Sora/WorldLabs AI Gen".into(), "MCP / ai_*".into()],
            ],
            true,
        );

        let mut sample_wb = Workbook::new("Financial Projection");
        sample_wb.active_sheet_mut().write_range(
            "A1",
            vec![
                vec!["Quarter".into(), "Users".into(), "Revenue ($)".into()],
                vec!["Q1".into(), "12000".into(), "144000".into()],
                vec!["Q2".into(), "28000".into(), "336000".into()],
                vec!["Q3".into(), "65000".into(), "780000".into()],
                vec!["Total".into(), "=SUM(B2:B4)".into(), "=SUM(C2:C4)".into()],
            ],
        ).unwrap();

        let mut sample_deck = Presentation::new("DreamCraft AI Studio");
        let s2 = sample_deck.add_slide(
            dreamcraft_primitives::tool::SlideLayout::TitleAndContent,
            "Autonomous Creative Suite",
        );
        if let Some(s) = sample_deck.slides.get_mut(s2) {
            s.add_bullet("100% Pure Rust high-speed core", 0.0);
            s.add_bullet("Zero manual UI bloat: Driven by coding / AI tools", 50.0);
            s.add_bullet("Live Presentation Viewer for human inspection", 100.0);
        }

        let mut sample_seq = Sequence::new("Teaser Edit");
        let v1 = sample_seq.video_tracks[0].id;
        let a1 = sample_seq.audio_tracks[0].id;
        let _ = sample_seq.insert_clip(v1, "Intro Shot", "assets/intro.mp4", dreamcraft_core::Tick::ZERO, dreamcraft_core::Tick::from_seconds(4.0));
        let _ = sample_seq.insert_clip(v1, "Core Reveal", "assets/reveal.mp4", dreamcraft_core::Tick::from_seconds(4.0), dreamcraft_core::Tick::from_seconds(6.0));
        let _ = sample_seq.insert_clip(a1, "Epic Synth Bed", "assets/bgm.wav", dreamcraft_core::Tick::ZERO, dreamcraft_core::Tick::from_seconds(10.0));

        let mut sample_photo = PhotoCanvas::new("Key Visual", 1280, 720);
        sample_photo.add_color_layer("Sky Gradient", dreamcraft_core::Color::rgb(20, 30, 60), dreamcraft_core::Rect::new(0.0, 0.0, 1280.0, 720.0));
        sample_photo.add_color_layer("Foreground Accent", dreamcraft_core::Color::rgba(255, 120, 50, 180), dreamcraft_core::Rect::new(100.0, 400.0, 1080.0, 200.0));

        let mut sample_vector = VectorDocument::new("Logo Vector", 800.0, 600.0);
        sample_vector.add_rect("Frame", 50.0, 50.0, 700.0, 500.0, Some(dreamcraft_core::Color::rgb(240, 245, 255)), Some(dreamcraft_core::Color::rgb(40, 60, 120)));
        sample_vector.add_circle("Orb", 400.0, 300.0, 120.0, Some(dreamcraft_core::Color::rgb(80, 140, 255)), None);

        Self {
            active_tab: ActiveTab::Document,
            word_doc: Arc::new(Mutex::new(Some(sample_doc))),
            workbook: Arc::new(Mutex::new(Some(sample_wb))),
            deck: Arc::new(Mutex::new(Some(sample_deck))),
            film_seq: Arc::new(Mutex::new(Some(sample_seq))),
            photo_canvas: Arc::new(Mutex::new(Some(sample_photo))),
            vector_doc: Arc::new(Mutex::new(Some(sample_vector))),
            status_log: vec![
                "[System]: DreamCraft Presentation Studio initialized.".into(),
                "[MCP]: Agent listening on stdio (24 tool primitives active).".into(),
                "[Agent]: Loaded document, spreadsheet, pitch deck, and movie timeline.".into(),
            ],
        }
    }
}

impl eframe::App for DreamCraftStudio {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_header").show(ctx, |ui| {
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("✦ DreamCraft Studio").strong().size(18.0).color(Color32::from_rgb(137, 180, 250)));
                ui.label(RichText::new("AI-Native Creative Presentation Layer").weak().size(13.0));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new("● AI Agent Active (MCP)").color(Color32::from_rgb(166, 227, 161)).strong());
                });
            });

            ui.add_space(4.0);
            ui.separator();
            ui.add_space(4.0);

            // Tab navigation
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.active_tab, ActiveTab::Document, "📄 Document (Word)");
                ui.selectable_value(&mut self.active_tab, ActiveTab::Spreadsheet, "📊 Spreadsheet (Excel)");
                ui.selectable_value(&mut self.active_tab, ActiveTab::Presentation, "📽 Presentation (Deck)");
                ui.selectable_value(&mut self.active_tab, ActiveTab::Timeline, "🎬 Movie Timeline (Film)");
                ui.selectable_value(&mut self.active_tab, ActiveTab::Canvas, "🎨 Visual Canvas (Photo/Vector)");
            });
            ui.add_space(4.0);
        });

        egui::TopBottomPanel::bottom("bottom_status").min_height(60.0).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Agent Operation Feed:").strong().size(11.0).color(Color32::from_rgb(150, 160, 180)));
            });
            egui::ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                for log in &self.status_log {
                    ui.label(RichText::new(log).monospace().size(11.0).color(Color32::from_rgb(180, 190, 200)));
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.active_tab {
                ActiveTab::Document => {
                    let lock = self.word_doc.lock().unwrap();
                    if let Some(doc) = lock.as_ref() {
                        DocPreview::show(ui, doc);
                    }
                }
                ActiveTab::Spreadsheet => {
                    let lock = self.workbook.lock().unwrap();
                    if let Some(wb) = lock.as_ref() {
                        GridPreview::show(ui, wb);
                    }
                }
                ActiveTab::Presentation => {
                    let lock = self.deck.lock().unwrap();
                    if let Some(deck) = lock.as_ref() {
                        DeckPreview::show(ui, deck);
                    }
                }
                ActiveTab::Timeline => {
                    let lock = self.film_seq.lock().unwrap();
                    if let Some(seq) = lock.as_ref() {
                        FilmPreview::show(ui, seq);
                    }
                }
                ActiveTab::Canvas => {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            let lock = self.photo_canvas.lock().unwrap();
                            if let Some(canvas) = lock.as_ref() {
                                CanvasPreview::show_photo(ui, canvas);
                            }
                        });
                        ui.separator();
                        ui.vertical(|ui| {
                            let lock = self.vector_doc.lock().unwrap();
                            if let Some(doc) = lock.as_ref() {
                                CanvasPreview::show_vector(ui, doc);
                            }
                        });
                    });
                }
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(Vec2::new(1280.0, 840.0))
            .with_title("DreamCraft Studio"),
        ..Default::default()
    };
    eframe::run_native(
        "DreamCraft Studio",
        native_options,
        Box::new(|cc| Ok(Box::new(DreamCraftStudio::new(cc)))),
    )
}
