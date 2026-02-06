use eframe::egui;
use egui::{Rangef, Sense};
use egui::{Color32, Painter, Pos2, Rect, Stroke, Vec2};
use crate::core_state::GuitarState;
use crate::core_state::Settings;

pub fn show(ui: &mut egui::Ui, guitar: &mut GuitarState, settings: &Settings) {

    // TODO: Make this not in both functions
    let num_frets: u8 = 25;
    let num_strings = guitar.config.num_strings;

    

    // Determine size
    let desired_size = Vec2::new(ui.available_width(), (num_strings as f32) * 30.0);
    let (rect , response) = ui.allocate_exact_size(desired_size, Sense::click());

    // Handle clicks
    if response.clicked() {
        if let Some(mouse_pos) = response.interact_pointer_pos() {
            // Get relative pos
            let relative_pos = mouse_pos - rect.min;
            log::info!("Clicked X: {}, Y: {}", relative_pos.x, relative_pos.y);
            // TODO: Make this not in both functions
            let string_range_y = Rangef::new(rect.min.y + 10.0, rect.max.y - 10.0); // a little buffer
    
            let fret_width = rect.width() / num_frets as f32;
            let string_height = string_range_y.span() / (num_strings - 1) as f32;

            let clicked_fret = (relative_pos.x / fret_width).floor() as u8;
            let clicked_string = num_strings - (relative_pos.y / string_height).round() as u8;

            let note = guitar.config.get_note_on_fretboard(clicked_string, clicked_fret);
            guitar.active_note = Some(note);
            log::info!("Clicked String: {}, Fret: {}, Note: {}", clicked_string, clicked_fret, note.name.to_string());
        }
    }

    let painter = ui.painter_at(rect);
    draw_fretboard(&painter, rect, guitar);
}


fn draw_fretboard(painter: &Painter, rect: Rect, guitar: &GuitarState) {

    // Draw background
    painter.rect_filled(rect, 2.0, Color32::from_rgb(60, 40, 30));

    // TODO: Make this not in both functions
    let num_frets: u8 = 25;
    let num_strings = guitar.config.num_strings;
    let string_range_y = Rangef::new(rect.min.y + 10.0, rect.max.y - 10.0); // a little buffer
    
    let fret_width = rect.width() / num_frets as f32;
    let string_height = string_range_y.span() / (num_strings - 1) as f32;

    // Draw 0 fret
    painter.line_segment(
        [Pos2::new(rect.min.x + fret_width, rect.min.y), Pos2::new(rect.min.x + fret_width, rect.max.y)], 
        Stroke::new(4.0, Color32::from_rgb(0, 0, 0)));

    // Draw frets
    for i in 2..=num_frets {
        let x = rect.min.x + (i as f32 * fret_width);
        painter.line_segment(
            [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)], 
            Stroke::new(2.0, Color32::from_rgb(180, 180, 180)));
    }

    // Draw strings 
    for i in 0..=num_strings {
        let y = string_range_y.min + (i as f32 * string_height);
        // Make lower strings thicker
        let thickness = 1.0 + (i as f32 * 0.5);
        painter.line_segment(
            [Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)],
            Stroke::new(thickness, Color32::from_rgb(210, 210, 210))
        );

        // Only apply texture to the bottom 3 or 4 strings
        if i > 2 { 
            for x_step in (rect.min.x as i32..rect.max.x as i32).step_by(2) {
                let x = x_step as f32;
                painter.line_segment(
                    [Pos2::new(x, y - (thickness * 0.4)), Pos2::new(x, y + (thickness * 0.4))],
                    Stroke::new(1.0, Color32::from_rgba_unmultiplied(0, 0, 0, 80))
                );
            }
        }

        // Shadow
        painter.line_segment(
            [Pos2::new(rect.min.x, y + thickness), Pos2::new(rect.max.x, y + thickness)],
            Stroke::new(thickness, Color32::from_rgba_unmultiplied(0, 0, 0, 200))
        );

    }

}