use eframe::egui;
use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, OscillatorType};

pub mod core_state;
mod gui;

#[cfg(test)]
mod tests;

#[wasm_bindgen]
pub struct DawApp {
    ctx: Option<AudioContext>,
    // States
    settings: core_state::Settings,

    // UI
    

}

impl Default for DawApp {
    fn default() -> Self {
        Self { 
            ctx: None,
            settings: core_state::Settings::new(),

        }
    }
}

impl eframe::App for DawApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        gui::header::show(ctx);

        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            gui::sidebar::show(ui, &mut self.settings);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Main Content");
            ui.separator();
            if ui.button("Play Test Tone").clicked() {
                self.play_test_tone();
            }

            ui.separator();

        });
    }
}

impl DawApp {
    fn play_test_tone(&mut self) {
        // Initialize context on first click (browsers block auto-audio)
        if self.ctx.is_none() {
            self.ctx = Some(AudioContext::new().unwrap());
        }
        
        if let Some(ref audio_ctx) = self.ctx {
            let osc = audio_ctx.create_oscillator().unwrap();
            osc.set_type(OscillatorType::Sine);
            osc.frequency().set_value(440.0); // A4
            
            osc.connect_with_audio_node(&audio_ctx.destination()).unwrap();
            osc.start().unwrap();
            
            // Stop after 0.5 seconds
            let _ = osc.stop_with_when(audio_ctx.current_time() + 0.5);
        }
    }
}

// Entry point for Trunk
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let web_options = eframe::WebOptions::default();

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
                Box::new(|_cc| Ok(Box::new(DawApp::default()))),
            )
            .await
            .expect("failed to start eframe");
    });
    Ok(())
}