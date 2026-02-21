use eframe::egui;
use egui::response;


pub fn toggle_switch(ui: &mut egui::Ui, on: &mut bool, vertical: bool) -> egui::Response {

    let desired_size = if vertical {
        ui.spacing().interact_size.y * egui::vec2(1.0, 2.0)
    } else {
        ui.spacing().interact_size.y * egui::vec2(2.0, 1.0)
    };
    
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());


    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }

    // Animate
    let how_on = ui.ctx().animate_bool(response.id, *on);

    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact_selectable(&response, *on);
        let radius = if vertical {
            0.5 * rect.width()
        } else {
            0.5 * rect.height()
        };

        // Track
        let track_color = if *on {
            ui.visuals().selection.bg_fill
        } else {
            ui.visuals().widgets.inactive.bg_fill
        };
        ui.painter().add(egui::Shape::rect_filled(rect, radius, track_color));



        // Knob
        let center = if vertical {
            egui::pos2(
                rect.center().x,
                egui::lerp((rect.bottom() - radius)..=(rect.top() + radius), how_on),
            )
        } else {
            egui::pos2(
                egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on),
                rect.center().y
            )
        };

        ui.painter().add(egui::Shape::circle_filled(center, radius * 0.75, egui::Color32::WHITE));
    }

    response
}