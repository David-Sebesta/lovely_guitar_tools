use eframe::egui;

// Data
pub struct TrackPanel {
    instrument_name: String,
}

impl TrackPanel {
    pub fn new() -> Self {
        Self {
            instrument_name: String::new(),
        }
    }

    // Behavior
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Tracks");

        ui.text_edit_singleline(&mut self.instrument_name);
    }


}