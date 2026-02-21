use eframe::egui;
use egui::RichText;

pub fn show(ctx: &egui::Context) {
    egui::TopBottomPanel::top("my_header").show(ctx, |ui| {
        ui.centered_and_justified(|ui| {
            ui.label(RichText::new("Lovely Guitar Tools").size(32.0).strong());
        });
    });
}