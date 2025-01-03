use crate::util::time_stamp_to_string;
use egui::{pos2, Color32, Id, Rangef, Rect, Rounding, Sense, Stroke, Ui, Vec2};

pub struct ScrubBar {
    end_time: u64,
}

impl ScrubBar {
    pub fn new(end_time: u64) -> Self {
        Self { end_time }
    }

    pub fn add(
        &mut self,
        ui: &mut Ui,
        current_time: &mut u64,
        size: Vec2,
    ) {
        let (scrub_response, scrub_painter) =
            ui.allocate_painter(size, Sense::union(Sense::click_and_drag(), Sense::hover()));
        scrub_painter.rect_filled(Rect::EVERYTHING, Rounding::ZERO, Color32::DARK_GRAY);

        let mut hover_time = None;

        if let Some(hover_pos) = scrub_response.hover_pos() {
            if scrub_painter.clip_rect().contains(hover_pos) {
                let total = scrub_painter.clip_rect().max.x - scrub_painter.clip_rect().min.x;
                let distance = hover_pos.x - scrub_painter.clip_rect().min.x;
                let progress = (distance / total) as f64;
                hover_time = Some((self.end_time as f64 * progress) as u64);
                if !scrub_response.is_pointer_button_down_on() {
                    scrub_painter.vline(
                        hover_pos.x,
                        Rangef::new(
                            scrub_painter.clip_rect().min.y,
                            scrub_painter.clip_rect().max.y,
                        ),
                        Stroke::new(2.0, Color32::WHITE),
                    );
                }
            }
        }

        let elapsed_time = *current_time;
        let progress = (elapsed_time as f64 / self.end_time as f64) as f32;
        let current_cursor_x_movement = scrub_painter.clip_rect().width() * progress;
        let current_cursor_x = scrub_painter.clip_rect().min.x + current_cursor_x_movement;

        if scrub_response.is_pointer_button_down_on() {
            let current_pos = scrub_response
                .interact_pointer_pos()
                .unwrap_or_else(|| pos2(0.0, 0.0));
            let total = scrub_painter.clip_rect().max.x - scrub_painter.clip_rect().min.x;
            let distance = current_pos.x - scrub_painter.clip_rect().min.x;
            let progress = (distance / total) as f64;
            *current_time = (self.end_time as f64 * progress) as u64;
        }

        if scrub_response.is_pointer_button_down_on() || scrub_response.hovered() {
            let text_time = if let Some(hover_time) = hover_time {
                time_stamp_to_string(hover_time)
            } else {
                time_stamp_to_string(*current_time)
            };
            egui::show_tooltip_at_pointer(ui.ctx(), ui.layer_id(), Id::new("Scrub tooltip"), |ui| {
                ui.label(text_time);
            });
        }

        scrub_painter.vline(
            current_cursor_x,
            Rangef::new(
                scrub_painter.clip_rect().min.y,
                scrub_painter.clip_rect().max.y,
            ),
            Stroke::new(2.0, Color32::WHITE),
        );
    }
}