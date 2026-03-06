use eframe::egui;
use wasm_bindgen::prelude::*;

use crate::{audio_engine::AudioEngine, core_state::MusicalStructure};

pub mod core_state;
mod gui;
mod audio_engine;


#[cfg(test)]
mod tests;

#[wasm_bindgen]
pub struct LovelyGuitarToolsApp {
    audio_engine: AudioEngine,
    // States
    guitar_state: core_state::GuitarState,
    settings: core_state::Settings,

}

impl Default for LovelyGuitarToolsApp {
    fn default() -> Self {
        Self { 
            audio_engine: AudioEngine::new(),
            guitar_state: core_state::GuitarState::new(),
            settings: core_state::Settings::new(),

        }
    }
}

impl eframe::App for LovelyGuitarToolsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        gui::header::show(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                gui::settings_panel::show(ui, &mut self.guitar_state, &mut self.settings, &mut self.audio_engine);

                ui.separator();

                gui::fretboard::show(ui, &mut self.guitar_state, &self.settings, &mut self.audio_engine);

                ui.separator();

                gui::details::show(ui, &mut self.guitar_state, &mut self.settings, &mut self.audio_engine);
            });
        });
    }
}


// Entry point for Trunk
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let web_options = eframe::WebOptions::default();

    // Initialize the logger backend
    // This connects log::info! to console.log()
    console_log::init_with_level(log::Level::Info).expect("error initializing logger");

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("Failed to cast to HtmlCanvasElement");

        eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|_cc| Ok(Box::new(LovelyGuitarToolsApp::default()))),
            )
            .await
            .expect("failed to start eframe");
    });
    Ok(())
}