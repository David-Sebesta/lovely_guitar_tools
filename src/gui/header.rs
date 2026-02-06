use eframe::egui;

pub fn show(ctx: &egui::Context) {
    egui::TopBottomPanel::top("my_header").show(ctx, |ui| {
        ui.label("I am a header!");
    });
}