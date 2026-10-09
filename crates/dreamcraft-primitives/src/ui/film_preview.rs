#[cfg(feature = "ui")]
use egui::{Color32, Pos2, Rect, RichText, Stroke, Ui, Vec2};

use crate::tool::film::Sequence;

pub struct FilmPreview;

impl FilmPreview {
    #[cfg(feature = "ui")]
    pub fn show(ui: &mut Ui, seq: &Sequence) {
        ui.horizontal(|ui| {
            ui.label(RichText::new(&format!("Sequence: {}", seq.name)).strong().size(16.0));
            ui.separator();
            let total_dur = seq.total_duration().to_seconds();
            ui.label(format!("Duration: {:.2}s ({} fps)", total_dur, seq.settings.fps));
            ui.separator();
            ui.label(format!("Playhead: {}", seq.playhead.format_timecode(seq.settings.fps)));
        });
        ui.add_space(10.0);

        let timeline_w = ui.available_width().max(600.0);
        let track_h = 36.0;
        let total_tracks = seq.video_tracks.len() + seq.audio_tracks.len();
        let timeline_h = total_tracks as f32 * (track_h + 4.0) + 30.0;

        let (response, painter) = ui.allocate_painter(Vec2::new(timeline_w, timeline_h), egui::Sense::hover());
        let rect = response.rect;

        // Background
        painter.rect_filled(rect, 4.0, Color32::from_rgb(24, 25, 34));
        painter.rect_stroke(rect, 4.0, Stroke::new(1.0_f32, Color32::from_rgb(45, 47, 65)));

        let max_time_s = seq.total_duration().to_seconds().max(10.0);
        let pps = (timeline_w - 60.0) / max_time_s as f32; // pixels per second

        let mut curr_y = rect.min.y + 10.0;

        // Draw video tracks
        for track in &seq.video_tracks {
            let track_rect = Rect::from_min_size(Pos2::new(rect.min.x + 50.0, curr_y), Vec2::new(timeline_w - 60.0, track_h));
            painter.rect_filled(track_rect, 2.0, Color32::from_rgb(32, 34, 46));

            // Track label
            painter.text(Pos2::new(rect.min.x + 8.0, curr_y + track_h / 2.0), egui::Align2::LEFT_CENTER, &track.name, egui::FontId::proportional(12.0), Color32::from_rgb(137, 180, 250));

            // Track clips
            for item in &track.items {
                let clip_x = rect.min.x + 50.0 + (item.start.to_seconds() as f32 * pps);
                let clip_w = (item.duration.to_seconds() as f32 * pps).max(4.0);
                let clip_rect = Rect::from_min_size(Pos2::new(clip_x, curr_y + 2.0), Vec2::new(clip_w, track_h - 4.0));

                painter.rect_filled(clip_rect, 3.0, Color32::from_rgb(40, 100, 160));
                painter.rect_stroke(clip_rect, 3.0, Stroke::new(1.0_f32, Color32::from_rgb(70, 140, 210)));

                painter.text(
                    Pos2::new(clip_x + 6.0, curr_y + track_h / 2.0),
                    egui::Align2::LEFT_CENTER,
                    &item.name,
                    egui::FontId::proportional(11.0),
                    Color32::WHITE,
                );
            }
            curr_y += track_h + 4.0;
        }

        // Draw audio tracks
        for track in &seq.audio_tracks {
            let track_rect = Rect::from_min_size(Pos2::new(rect.min.x + 50.0, curr_y), Vec2::new(timeline_w - 60.0, track_h));
            painter.rect_filled(track_rect, 2.0, Color32::from_rgb(28, 38, 32));

            // Track label
            painter.text(Pos2::new(rect.min.x + 8.0, curr_y + track_h / 2.0), egui::Align2::LEFT_CENTER, &track.name, egui::FontId::proportional(12.0), Color32::from_rgb(166, 227, 161));

            // Audio clips
            for item in &track.items {
                let clip_x = rect.min.x + 50.0 + (item.start.to_seconds() as f32 * pps);
                let clip_w = (item.duration.to_seconds() as f32 * pps).max(4.0);
                let clip_rect = Rect::from_min_size(Pos2::new(clip_x, curr_y + 2.0), Vec2::new(clip_w, track_h - 4.0));

                painter.rect_filled(clip_rect, 3.0, Color32::from_rgb(45, 110, 60));
                painter.rect_stroke(clip_rect, 3.0, Stroke::new(1.0_f32, Color32::from_rgb(80, 160, 100)));

                painter.text(
                    Pos2::new(clip_x + 6.0, curr_y + track_h / 2.0),
                    egui::Align2::LEFT_CENTER,
                    &item.name,
                    egui::FontId::proportional(11.0),
                    Color32::WHITE,
                );
            }
            curr_y += track_h + 4.0;
        }
    }
}
