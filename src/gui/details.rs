use crate::AudioEngine;
use crate::core_state::GuitarState;
use crate::core_state::MusicalStructure;
use crate::core_state::NoteName;
use crate::core_state::Tuning;
use crate::core_state::{Mode, Settings};
use eframe::egui;
use egui::RichText;
use egui::scroll_area;
use strum::IntoEnumIterator;
use std::fmt::Display;

pub fn show(ui: &mut egui::Ui, guitar: &mut GuitarState, settings: &mut Settings, audio_engine: &mut AudioEngine) {
    egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(20, 10))
        .show(ui, |ui| {
            ui.vertical(|ui| {
                ui.heading(RichText::new("Details").size(24.0).strong());
                ui.add_space(8.0);

                match settings.mode {
                    Mode::Chord => show_chord_details(ui, guitar, settings),
                    Mode::Scale => show_scale_details(ui, guitar, settings),
                    Mode::ReverseChord => show_reverse_chord_details(ui, guitar, settings, audio_engine),
                    Mode::ReverseScale => show_reverse_scale_details(ui, guitar, settings, audio_engine),
                }
            });
        });
}

fn show_chord_details(ui: &mut egui::Ui, guitar: &mut GuitarState, settings: &mut Settings) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("Notes:").strong());

        let notes_str = settings
            .chord
            .notes()
            .iter()
            .map(|note| note.to_string())
            .collect::<Vec<_>>()
            .join(" - ");

        ui.label(notes_str);
    });

    ui.horizontal(|ui| {
        ui.label(RichText::new("Intervals:").strong());
        let interval_str = settings
            .chord
            .intervals()
            .iter()
            .map(|interval| interval.to_string())
            .collect::<Vec<_>>()
            .join(" - ");

        ui.label(interval_str);
    });
}

fn show_scale_details(ui: &mut egui::Ui, guitar: &mut GuitarState, settings: &mut Settings) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("Notes:").strong());

        let notes_str = settings
            .scale
            .notes()
            .iter()
            .map(|note| note.to_string())
            .collect::<Vec<_>>()
            .join(" - ");

        ui.label(notes_str);
    });

    ui.horizontal(|ui| {
        ui.label(RichText::new("Intervals:").strong());
        let interval_str = settings
            .scale
            .intervals()
            .iter()
            .map(|interval| interval.to_string())
            .collect::<Vec<_>>()
            .join(" - ");

        ui.label(interval_str);
    });
}

fn show_reverse_chord_details(ui: &mut egui::Ui, guitar: &mut GuitarState, settings: &mut Settings, audio_engine: &mut AudioEngine) {
    let possible_chords = guitar.matching_chords.clone();

    if let Some(chord) = show_selectable_list(ui, "Possible Chords", &possible_chords, guitar, audio_engine) {
        settings.mode = Mode::Chord;
        settings.chord = chord;
        guitar.update_notes(&settings.chord.notes(), true);
    }
}

fn show_reverse_scale_details(ui: &mut egui::Ui, guitar: &mut GuitarState, settings: &mut Settings, audio_engine: &mut AudioEngine) {
    let possible_scales = guitar.matching_scales.clone();

    if let Some(scale) = show_selectable_list(ui, "Possible Scales", &possible_scales, guitar, audio_engine) {
        settings.mode = Mode::Scale;
        settings.scale = scale;
        guitar.update_notes(&settings.scale.notes(), true);
    }
}

fn show_selectable_list<T>(
    ui: &mut egui::Ui,
    title: &str,
    items: &Option<Vec<T>>,
    guitar: &mut GuitarState,
    audio_engine: &mut AudioEngine,
) -> Option<T>
where
    T: Display + Clone + MusicalStructure + 'static, // Tells Rust "T must have .to_string() and .clone()"
{
    ui.label(RichText::new(title).size(18.0));
    let mut selected_item = None;

    guitar.clear_greyed_notes();

    egui::ScrollArea::horizontal().show(ui, |ui| {
        if let Some(list) = items {
            for item in list {
                let button = ui.button(RichText::new(item.to_string()).size(16.0));
                if button.clicked() {
                    selected_item = Some(item.clone());
                } else if button.secondary_clicked() {
                    audio_engine.play_musical_structure(item);
                }
                
                // Show greyed out notes on fretboard
                if button.hovered() {
                    guitar.update_notes(&item.notes(), false);
                }
            }
        }
    });

    selected_item
}