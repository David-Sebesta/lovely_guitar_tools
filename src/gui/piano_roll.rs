use eframe::egui;
use eframe::egui::Vec2;
use eframe::egui::vec2;
use eframe::egui::Color32;
use eframe::egui::Sense;
use eframe::egui::Stroke;
use std::f32::consts::TAU;


pub struct PianoRoll {
    // View settings
    pub zoom_level: f32,
    pub scroll_offset: f32,

    // Constants for drawing
    pub key_height: f32,
    pub tick_width: f32,
}

impl PianoRoll {
    pub fn new() -> Self {
        Self {
            zoom_level: 1.0,
            scroll_offset: 0.0,
            key_height: 20.0,
            tick_width: 10.0,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {

        // Allocate big canvas
        let size = ui.available_size();
        let (response, painter) = ui.allocate_painter(size, Sense::click_and_drag());
        let rect = response.rect;

        // Start from top (high pitch) to bottom (low pitch)
        for midi_note in 0..128 {
            // Invert pitch
            let normalized_y = (127 - midi_note) as f32 * self.key_height;

            // TODO: Apply scroll offset here later
            let y = rect.min.y + normalized_y;

            if y >= rect.min.y && y <= rect.max.x {
                painter.line_segment([
                    eframe::egui::pos2(rect.min.x, y),
                    eframe::egui::pos2(rect.max.x, y)
                ], 
                Stroke::new(1.0, Color32::from_gray(60)),
            );
            }
        }

        // Draw vertical grid (time/tick)
        // draw a line every 4 tick (1 beat) for now
        let total_ticks = 128 * 4;
        for tick in (0..total_ticks).step_by(4) {
            let x = rect.min.x + (tick as f32 * self.tick_width * self.zoom_level) - self.scroll_offset;

            if x >= rect.min.x && x <= rect.max.x {
                painter.line_segment(
                    [
                        eframe::egui::pos2(x, rect.min.y),
                        eframe::egui::pos2(x, rect.max.y)
                    ],
                    Stroke::new(1.0, Color32::from_gray(80)),
                );
            }
        }

    }

    fn tick_to_x(_tick: f32) -> f32 {
        return 1.0;
    }

    fn pitch_to_y(_pitch: f32) -> f32 {
        return 1.0;
    }


}