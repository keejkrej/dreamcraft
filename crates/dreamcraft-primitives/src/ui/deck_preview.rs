#[cfg(feature = "ui")]
use egui::{Color32, Pos2, Rect, RichText, Stroke, Ui, Vec2};

use crate::tool::deck::Presentation;

pub struct DeckPreview;

impl DeckPreview {
    #[cfg(feature = "ui")]
    pub fn show(ui: &mut Ui, deck: &Presentation) {
        ui.horizontal(|ui| {
            ui.label(RichText::new(&format!("Presentation: {}", deck.title)).strong().size(16.0));
            ui.separator();
            ui.label(format!("Slide {} of {}", deck.current_slide + 1, deck.slides.len()));
        });
        ui.add_space(8.0);

        if let Some(slide) = deck.slides.get(deck.current_slide) {
            // Render 16:9 slide canvas preview
            let available_width = ui.available_width().min(800.0);
            let slide_height = available_width * (9.0 / 16.0);

            let (response, painter) = ui.allocate_painter(Vec2::new(available_width, slide_height), egui::Sense::hover());
            let rect = response.rect;

            // Slide background
            let bg_color = Color32::from_rgba_unmultiplied(
                slide.background.r,
                slide.background.g,
                slide.background.b,
                slide.background.a,
            );
            painter.rect_filled(rect, 4.0, bg_color);
            painter.rect_stroke(rect, 4.0, Stroke::new(1.0_f32, Color32::from_rgb(60, 60, 80)));

            // Scale factor relative to 1920x1080 canvas
            let scale_x = available_width / deck.width as f32;
            let scale_y = slide_height / deck.height as f32;

            // Title
            let title_pos = Pos2::new(rect.min.x + 40.0 * scale_x, rect.min.y + 50.0 * scale_y);
            painter.text(
                title_pos,
                egui::Align2::LEFT_TOP,
                &slide.title,
                egui::FontId::proportional(28.0 * scale_y),
                Color32::from_rgb(30, 30, 40),
            );

            // Subtitle
            if let Some(sub) = &slide.subtitle {
                let sub_pos = Pos2::new(rect.min.x + 40.0 * scale_x, rect.min.y + 110.0 * scale_y);
                painter.text(
                    sub_pos,
                    egui::Align2::LEFT_TOP,
                    sub,
                    egui::FontId::proportional(16.0 * scale_y),
                    Color32::from_rgb(90, 90, 110),
                );
            }

            // Bullet points
            for tb in &slide.text_boxes {
                let pos = Pos2::new(
                    rect.min.x + tb.bounds.x as f32 * scale_x,
                    rect.min.y + tb.bounds.y as f32 * scale_y,
                );
                let bullet_prefix = if tb.is_bullet { "• " } else { "" };
                painter.text(
                    pos,
                    egui::Align2::LEFT_TOP,
                    format!("{}{}", bullet_prefix, tb.text),
                    egui::FontId::proportional(tb.font_size * scale_y * 1.2),
                    Color32::from_rgb(40, 40, 50),
                );
            }

            // Shapes
            for shape in &slide.shapes {
                let shape_rect = Rect::from_min_size(
                    Pos2::new(
                        rect.min.x + shape.bounds.x as f32 * scale_x,
                        rect.min.y + shape.bounds.y as f32 * scale_y,
                    ),
                    Vec2::new(
                        shape.bounds.width as f32 * scale_x,
                        shape.bounds.height as f32 * scale_y,
                    ),
                );
                let col = Color32::from_rgba_unmultiplied(
                    shape.fill_color.r,
                    shape.fill_color.g,
                    shape.fill_color.b,
                    shape.fill_color.a,
                );
                painter.rect_filled(shape_rect, 2.0, col);
            }
        }
    }
}
