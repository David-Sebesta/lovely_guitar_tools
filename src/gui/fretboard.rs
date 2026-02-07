use eframe::egui;
use egui::Sense;
use egui::{Color32, Painter, Pos2, Rect, Stroke, Vec2};
use crate::core_state::GuitarState;
use crate::core_state::Settings;


struct FretboardLayout {
    rect: Rect,
    fret_width: f32,
    string_height: f32,
    string_start_y: f32,
    num_strings: u8,
    num_frets: u8,
}

impl FretboardLayout {
    fn new(rect: Rect, num_strings: u8, num_frets: u8) -> Self {
        // Buffer at top and bottom
        let vertical_padding = 10.0;
        let usable_height = rect.height() - (vertical_padding * 2.0);

        let fret_width = rect.width() / num_frets as f32;

        let string_height = usable_height / (num_strings - 1) as f32;

        Self {
            rect,
            fret_width,
            string_height,
            string_start_y: rect.min.y + vertical_padding,
            num_strings,
            num_frets,
        }
    }

    // Screen point -> (string, fret)
    fn get_hit_string_and_fret(&self, pos: Pos2) -> (u8, u8) {
        let relative_x = pos.x - self.rect.min.x;
        let relative_y = pos.y - self.rect.min.y;

        let fret = (relative_x / self.fret_width).floor() as u8;
        
        let string_float = relative_y / self.string_height;
        let string_idx = string_float.round() as u8;

        let clamped_string = string_idx.clamp(0, self.num_strings - 1);
        let clamped_fret = fret.clamp(0, self.num_frets);

        // Invert strings (0 is bottom/lowest freq)
        let string = (self.num_strings - 1) - clamped_string;

        (string, clamped_fret)
    }

    // String & Fret -> Screen Rect
    fn get_hitbox_rect(&self, string_idx: u8, fret_idx: u8) -> Rect {
        let x_start = self.rect.min.x + (fret_idx as f32 * self.fret_width);

        // Invert string
        let string = (self.num_strings - 1) - string_idx;

        let y_center = self.string_start_y + (string as f32 * self.string_height);

        // Hit box is halfway
        let half_height = self.string_height / 2.0;

        Rect::from_min_max(
            Pos2::new(x_start, y_center - half_height),
            Pos2::new(x_start + self.fret_width, y_center + half_height)
        )
    }

}


pub fn show(ui: &mut egui::Ui, guitar: &mut GuitarState, settings: &Settings) {
    // 25 since there is a 0 fret (open string)
    let num_frets: u8 = 25;
    let num_strings = guitar.config.num_strings;

    // Allocate space
    let desired_size = Vec2::new(ui.available_width(), (num_strings as f32) * 30.0);
    let (rect , response) = ui.allocate_exact_size(desired_size, Sense::click());

    let layout = FretboardLayout::new(rect, num_strings, num_frets);

    // Handle clicks
    if response.clicked() {
        if let Some(mouse_pos) = response.interact_pointer_pos() {
            let (string, fret) = layout.get_hit_string_and_fret(mouse_pos);

            let note = guitar.config.get_note_on_fretboard(string, fret);
            guitar.active_note = Some(note);
            log::info!("Clicked String: {}, Fret: {}, Note: {}", string, fret, note.to_string());
        }
    }

    let painter = ui.painter_at(rect);
    draw_fretboard(&painter, &layout);

    if settings.debug {
        draw_debug_overlay(&painter, &layout);
    }
}


fn draw_fretboard(painter: &Painter, layout: &FretboardLayout) {

    // Draw background
    painter.rect_filled(layout.rect, 2.0, Color32::from_rgb(60, 40, 30));

    // Draw 0 fret
    painter.line_segment(
        [Pos2::new(layout.rect.min.x + layout.fret_width, layout.rect.min.y), 
                Pos2::new(layout.rect.min.x + layout.fret_width, layout.rect.max.y)], 
        Stroke::new(4.0, Color32::from_rgb(0, 0, 0)));

    // Draw frets
    for i in 2..=layout.num_frets {
        let x = layout.rect.min.x + (i as f32 * layout.fret_width);
        painter.line_segment(
            [Pos2::new(x, layout.rect.min.y), Pos2::new(x, layout.rect.max.y)], 
            Stroke::new(2.0, Color32::from_rgb(180, 180, 180)));
    }

    // Draw strings 
    for i in 0..=layout.num_strings {
        let y = layout.string_start_y + (i as f32 * layout.string_height);
        // Make lower strings thicker
        let thickness = 1.0 + (i as f32 * 0.5);
        painter.line_segment(
            [Pos2::new(layout.rect.min.x, y), Pos2::new(layout.rect.max.x, y)],
            Stroke::new(thickness, Color32::from_rgb(210, 210, 210))
        );

        // Only apply texture to the bottom 3 or 4 strings
        if i > 2 {
            for x_step in (layout.rect.min.x as i32..layout.rect.max.x as i32).step_by(2) {
                let x = x_step as f32;
                painter.line_segment(
                    [Pos2::new(x, y - (thickness * 0.4)), Pos2::new(x, y + (thickness * 0.4))],
                    Stroke::new(1.0, Color32::from_rgba_unmultiplied(0, 0, 0, 80))
                );
            }
        }

        // Shadow
        painter.line_segment(
            [Pos2::new(layout.rect.min.x, y + thickness), Pos2::new(layout.rect.max.x, y + thickness)],
            Stroke::new(thickness, Color32::from_rgba_unmultiplied(0, 0, 0, 200))
        );

    }

}

fn draw_debug_overlay(painter: &Painter, layout: &FretboardLayout) {
    let debug_color = Color32::from_rgba_unmultiplied(255, 0, 0, 255);
    let stroke = Stroke::new(1.0, Color32::YELLOW);

    for s in 0..layout.num_strings {
        for f in 0..layout.num_frets {
            let hitbox = layout.get_hitbox_rect(s, f);

            painter.rect_stroke(hitbox, 0.0, stroke, egui::StrokeKind::Middle);

            painter.circle_filled(hitbox.center(), 2.0, debug_color);
        }
    }
}