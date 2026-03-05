use eframe::egui;
use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, OscillatorType};

use crate::{audio_engine::AudioEngine, core_state::{Chord, MusicalStructure, Scale}};

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

    // UI
    

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

        // egui::SidePanel::left("left_panel")
        //     .min_width(280.0)
        //     .show(ctx, |ui| {
        //     gui::sidebar::show(ui, &mut self.guitar_state, &mut self.settings);
        // });

        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.label("Main Content");
            // ui.separator();
            // if ui.button("Play").clicked() {
            //     self.play_mode_tone();
            // }

            gui::settings_panel::show(ui, &mut self.guitar_state, &mut self.settings);

            ui.separator();

            gui::fretboard::show(ui, &mut self.guitar_state, &self.settings);

            ui.separator();

            gui::details::show(ui, &mut self.guitar_state, &mut self.settings);

        });
    }
}

impl LovelyGuitarToolsApp {
    fn play_mode_tone(&mut self) {
        // // Initialize context on first click (browsers block auto-audio)
        // if self.ctx.is_none() {
        //     self.ctx = Some(AudioContext::new().unwrap());
        // }
        
        // if let Some(ref audio_ctx) = self.ctx && let Some(active_note) = self.guitar_state.active_note {
        //     let osc = audio_ctx.create_oscillator().unwrap();
        //     osc.set_type(OscillatorType::Sine);
        //     osc.frequency().set_value(active_note.frequency()); // A4
            
        //     osc.connect_with_audio_node(&audio_ctx.destination()).unwrap();
        //     osc.start().unwrap();
            
        //     // Stop after 0.5 seconds
        //     let _ = osc.stop_with_when(audio_ctx.current_time() + 0.5);
        // }

        match self.settings.mode {
            core_state::Mode::Scale => {
                let mut notes = self.settings.scale.get_notes(4);
                if let Some(n) = notes.first().map(|n| n.add_semitones(12)) {
                    notes.push(n);
                }
                self.audio_engine.play_scale(&notes, true);
            },
            core_state::Mode::Chord => {
                let notes = self.settings.chord.get_notes(4);
                self.audio_engine.play_chord(&notes);

            },
            core_state::Mode::ReverseScale => {

            },
            core_state::Mode::ReverseChord => {
                let notes = self.guitar_state.get_active_notes();
                self.audio_engine.play_chord(&notes);
            }
        }

        // if let Some(active_note) = self.guitar_state.active_note {
        //     self.audio_engine.play_note(&active_note);
        // }

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