#[cfg(feature = "ui")]
use egui::{Color32, Pos2, Rect, RichText, Stroke, Ui, Vec2};

use crate::tool::photo::PhotoCanvas;
use crate::tool::vector::VectorDocument;

pub struct CanvasPreview;

impl CanvasPreview {
    #[cfg(feature = "ui")]
    pub fn show_photo(ui: &mut Ui, canvas: &PhotoCanvas) {
        ui.horizontal(|ui| {
            ui.label(RichText::new(&format!("Canvas: {}", canvas.name)).strong().size(16.0));
            ui.separator();
            ui.label(format!("Size: {}×{} px | Layers: {}", canvas.width, canvas.height, canvas.layers.len()));
        });
        ui.add_space(8.0);

        let max_w = ui.available_width().min(600.0);
        let aspect = canvas.height as f32 / canvas.width as f32;
        let max_h = max_w * aspect;

        let (resp, painter) = ui.allocate_painter(Vec2::new(max_w, max_h), egui::Sense::hover());
        let rect = resp.rect;

        // Checkerboard / base
        painter.rect_filled(rect, 4.0, Color32::from_rgb(30, 30, 40));

        let sx = max_w / canvas.width as f32;
        let sy = max_h / canvas.height as f32;

        for layer in &canvas.layers {
            if !layer.visible {
                continue;
            }
            if let Some(fill) = layer.fill_color {
                let layer_rect = Rect::from_min_size(
                    Pos2::new(rect.min.x + layer.bounds.x as f32 * sx, rect.min.y + layer.bounds.y as f32 * sy),
                    Vec2::new(layer.bounds.width as f32 * sx, layer.bounds.height as f32 * sy),
                );
                let col = Color32::from_rgba_unmultiplied(
                    fill.r,
                    fill.g,
                    fill.b,
                    (fill.a as f32 * layer.opacity).round() as u8,
                );
                painter.rect_filled(layer_rect, 0.0, col);
            }
        }
    }

    #[cfg(feature = "ui")]
    pub fn show_vector(ui: &mut Ui, doc: &VectorDocument) {
        ui.horizontal(|ui| {
            ui.label(RichText::new(&format!("Vector Doc: {}", doc.name)).strong().size(16.0));
            ui.separator();
            ui.label(format!("Canvas: {}×{} pt | Elements: {}", doc.width, doc.height, doc.elements.len()));
        });
        ui.add_space(8.0);

        let max_w = ui.available_width().min(600.0);
        let aspect = (doc.height / doc.width) as f32;
        let max_h = max_w * aspect;

        let (resp, painter) = ui.allocate_painter(Vec2::new(max_w, max_h), egui::Sense::hover());
        let rect = resp.rect;

        painter.rect_filled(rect, 4.0, Color32::WHITE);
        painter.rect_stroke(rect, 4.0, Stroke::new(1.0_f32, Color32::from_rgb(200, 200, 210)));

        let sx = max_w / doc.width as f32;
        let sy = max_h / doc.height as f32;

        for el in &doc.elements {
            match &el.kind {
                crate::tool::vector::VectorShapeKind::Rect { x, y, w, h, rx: _ } => {
                    let r = Rect::from_min_size(
                        Pos2::new(rect.min.x + *x as f32 * sx, rect.min.y + *y as f32 * sy),
                        Vec2::new(*w as f32 * sx, *h as f32 * sy),
                    );
                    let fill_col = el.fill.map(|c| Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a)).unwrap_or(Color32::TRANSPARENT);
                    let stroke = el.stroke.map(|c| Stroke::new(el.stroke_width as f32 * sx, Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a))).unwrap_or(Stroke::NONE);
                    painter.rect(r, 0.0, fill_col, stroke);
                }
                crate::tool::vector::VectorShapeKind::Circle { cx, cy, r } => {
                    let center = Pos2::new(rect.min.x + *cx as f32 * sx, rect.min.y + *cy as f32 * sy);
                    let radius = *r as f32 * sx;
                    let fill_col = el.fill.map(|c| Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a)).unwrap_or(Color32::TRANSPARENT);
                    let stroke = el.stroke.map(|c| Stroke::new(el.stroke_width as f32 * sx, Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a))).unwrap_or(Stroke::NONE);
                    painter.circle(center, radius, fill_col, stroke);
                }
                _ => {}
            }
        }
    }
}
