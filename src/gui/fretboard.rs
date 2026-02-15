use eframe::egui;
use egui::{Align2, FontFamily, FontId, Sense};
use egui::{Color32, Painter, Pos2, Rect, Stroke, Vec2};
use crate::core_state::GuitarState;
use crate::core_state::{Settings, Mode};


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

        // Since there is a zero fret add 1
        let fret_width = rect.width() / (num_frets + 1) as f32;

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
        // Fix: Use string_start_y to align with the actual drawn string positions
        let relative_y = pos.y - self.string_start_y;

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
    let num_frets: u8 = guitar.config.num_frets;
    let num_strings = guitar.config.num_strings;

    // Allocate space
    let desired_size = Vec2::new(ui.available_width(), (num_strings as f32) * 30.0);
    let (rect , response) = ui.allocate_exact_size(desired_size, Sense::click());

    let layout = FretboardLayout::new(rect, num_strings, num_frets);

    // Handle clicks
    if response.clicked() {
        if let Some(mouse_pos) = response.interact_pointer_pos() {
            let (string, fret) = layout.get_hit_string_and_fret(mouse_pos);

            let note = guitar.get_note_on_fretboard(string, fret);
            match settings.mode {
                Mode::ReverseScale => {
                    guitar.toggle_note(string, fret);
                },
                Mode::ReverseChord => {
                    guitar.set_strings_note(string, fret);
                },
                _ => {}
            }

            guitar.active_note = Some(note);
            log::info!("Clicked String: {}, Fret: {}, Note: {}", string, fret, note.to_string());
        }
    }

    let painter = ui.painter_at(rect);
    draw_fretboard(&painter, &layout);
    draw_active_notes(&painter, &layout, guitar);

    if settings.debug {
        draw_debug_overlay(&painter, &layout);
    }
}


fn draw_fretboard(painter: &Painter, layout: &FretboardLayout) {

    // Draw background
    painter.rect_filled(layout.rect, 2.0, Color32::from_rgb(60, 40, 30));
    
    draw_frets(painter, layout);
    draw_inlays(painter, layout);
    draw_strings(painter, layout);

    

}

fn draw_frets(painter: &Painter, layout: &FretboardLayout) {
    // Draw 0 fret
    painter.line_segment(
        [Pos2::new(layout.rect.min.x + layout.fret_width, layout.rect.min.y), 
                Pos2::new(layout.rect.min.x + layout.fret_width, layout.rect.max.y)], 
        Stroke::new(6.0, Color32::from_rgb(0, 0, 0)));

    // Draw frets
    for i in 2..=layout.num_frets {
        let x = layout.rect.min.x + (i as f32 * layout.fret_width);
        painter.line_segment(
            [Pos2::new(x, layout.rect.min.y), Pos2::new(x, layout.rect.max.y)], 
            Stroke::new(2.0, Color32::from_rgb(180, 180, 180)));
    }
}

fn draw_inlays(painter: &Painter, layout: &FretboardLayout) {

    let y_center = layout.rect.center().y;
    let y_quarter = layout.rect.y_range().span() * 0.25;

    // Single dot 3, 5, 7, 9, 15, 17, 19, 21
    let single_dot_frets: [u8; 8] = [3, 5, 7, 9, 15, 17, 19, 21].map(|x: u8| x + 1);
    for fret in single_dot_frets {
        let x = layout.rect.min.x + (fret as f32 * layout.fret_width) - layout.fret_width * 0.5;
        painter.circle_filled(Pos2::new(x, y_center), 10.0, Color32::WHITE);
    }

    // Double dot 12, 24
    let double_dot_frets: [u8; 2] = [12, 24].map(|x: u8| x + 1);
    for fret in double_dot_frets {
        let x = layout.rect.min.x + (fret as f32 * layout.fret_width) - layout.fret_width * 0.5;
        painter.circle_filled(Pos2::new(x, layout.rect.min.y + y_quarter), 10.0, Color32::WHITE);
        painter.circle_filled(Pos2::new(x, layout.rect.min.y + y_quarter * 3.0), 10.0, Color32::WHITE);
    }



}

fn draw_strings(painter: &Painter, layout: &FretboardLayout) {
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

fn draw_active_notes(painter: &Painter, layout: &FretboardLayout, guitar: &GuitarState) {

    const RADIUS: f32 = 14.0;
    let circle_color = Color32::LIGHT_YELLOW;
    let text_color = Color32::BLACK;

    for (string, fret) in guitar.active_frets.iter() {
        let hitbox = layout.get_hitbox_rect(*string, *fret);
        let note = guitar.get_note_on_fretboard(*string, *fret);

        painter.circle_filled(hitbox.center(), RADIUS, circle_color);
        painter.text(hitbox.center(), Align2::CENTER_CENTER, note.to_string(), 
                    FontId::new(16.0, FontFamily::default()), text_color);

    }
}

fn draw_debug_overlay(painter: &Painter, layout: &FretboardLayout) {
    let debug_color = Color32::from_rgba_unmultiplied(255, 0, 0, 255);
    let stroke = Stroke::new(1.0, Color32::YELLOW);

    for s in 0..layout.num_strings {
        for f in 0..=layout.num_frets {
            let hitbox = layout.get_hitbox_rect(s, f);

            painter.rect_stroke(hitbox, 0.0, stroke, egui::StrokeKind::Middle);

            painter.circle_filled(hitbox.center(), 2.0, debug_color);
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use egui::{Rect, Pos2, Vec2};

    fn create_test_layout() -> FretboardLayout {
        let rect = Rect::from_min_size(Pos2::new(0.0, 0.0), Vec2::new(1000.0, 500.0));
        // 6 strings, 25 frets
        FretboardLayout::new(rect, 6, 25)
    }

    #[test]
    fn test_round_trip_consistency() {
        let layout = create_test_layout();

        for s in 0..6 {
            for f in 0..=25 {
                let hitbox = layout.get_hitbox_rect(s, f);
                let click_pos = hitbox.center();
                let (detected_s, detected_f) = layout.get_hit_string_and_fret(click_pos);

                assert_eq!(
                    (s, f), 
                    (detected_s, detected_f), 
                    "Mismatch at String {}, Fret {}. Detected String {}, Fret {} instead. Hitbox center: {:?}", 
                    s, f, detected_s, detected_f, click_pos
                );
            }
        }
    }

    #[test]
    fn test_click_clamping() {
        let layout = create_test_layout();
        
        // Click way top-left
        let (s, f) = layout.get_hit_string_and_fret(Pos2::new(-50.0, -50.0));
        
        // Fret logic: -50 x -> floor -> negative -> clamp(0) -> 0.
        // String logic:
        // relative_y = -50.
        // string_float = -50 / height. Negative.
        // Round -> Negative integer.
        // Clamp(0, 5) -> 0.
        // Invert -> 5 - 0 = 5.
        // So clicking WAY ABOVE the fretboard returns String 5 (High E).
        assert_eq!(f, 0);
        assert_eq!(s, 5); 

        // Click way bottom-right
        let (s, f) = layout.get_hit_string_and_fret(Pos2::new(9999.0, 9999.0));
        
        // Fret -> Max (25)
        // String -> Large positive -> Clamped to 5 -> Inverted -> 5-5 = 0.
        assert_eq!(f, 25);
        assert_eq!(s, 0);
    }

    #[test]
    fn test_click_bias() {
        // Small layout to exaggerate the error
        let rect = Rect::from_min_size(Pos2::new(0.0, 0.0), Vec2::new(100.0, 100.0));
        let layout = FretboardLayout::new(rect, 4, 10);
        
        // Usable height = 100 - 20 = 80.
        // Strings = 4. Gaps = 3.
        // String Height = 80 / 3 = 26.666...
        // String 0 Y = 10.0.
        // String 1 Y = 36.666...
        
        // Click at Y=20.0.
        // Distance to String 0 (10.0) = 10.0.
        // Distance to String 1 (36.66) = 16.66.
        // Should be String 0.
        
        // Invert check: 
        // We want to check the RAW string index first to avoid confusion with inversion.
        // The helper 'get_hit_string_and_fret' returns inverted strings.
        // String 0 (visually top) is index 3 (visually bottom) in the return value?
        // Wait, 'get_hitbox_rect' index 0 is TOP.
        // 'get_hit_string_and_fret' returns INVERTED index.
        // If I pass inverted index 3 (High E / Top) to get_hitbox_rect?
        // Let's trace get_hitbox_rect: "Invert string: let string = (num-1) - string_idx".
        // If I pass 3 -> string = 0. y_center = 10.0.
        // So 'String 3' (High E) is at Y=10.0.
        
        // So we expect 'get_hit_string_and_fret' to return 3.
        
        let (s, _) = layout.get_hit_string_and_fret(Pos2::new(50.0, 20.0));
        assert_eq!(s, 3, "Clicked Y=20 (closer to Top String Y=10), but got String index {}", s);
    }
}