use eframe::egui;
use egui::ComboBox;
use egui::SelectableLabel;
use strum::IntoEnumIterator;
use crate::core_state::GuitarState;
use crate::core_state::Tuning;
use crate::core_state::NoteName;
use crate::core_state::ScaleType;
use crate::core_state::{Settings, Mode};

macro_rules! selectable_enum {
    ($ui:expr, $current_val:expr, $enum_type:ident, [ $($variant:ident),* ]) => {
        $( $ui.selectable_value($current_val, $enum_type::$variant, $enum_type::$variant.to_string()); )*
    };
}   


pub fn show(ui: &mut egui::Ui, guitar: &mut GuitarState, settings: &mut Settings) {
    ui.horizontal(|ui| {
        ui.heading("Settings");
        ui.checkbox(&mut settings.debug, "Debug");
    });

    // Mode
    let before_mode = settings.mode;
    egui::ComboBox::from_label("Mode")
        .selected_text(format!("{:?}", before_mode.to_string()))
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut settings.mode, Mode::Scale, Mode::Scale.to_string());
            ui.selectable_value(&mut settings.mode, Mode::Chord, Mode::Chord.to_string());
            ui.selectable_value(&mut settings.mode, Mode::ReverseScale, Mode::ReverseScale.to_string());
            ui.selectable_value(&mut settings.mode, Mode::ReverseChord, Mode::ReverseChord.to_string());
        });

    // Clear notes when switching mode
    if before_mode != settings.mode {
        guitar.clear_notes();
        match settings.mode {
            Mode::Scale => {
                guitar.update_scale_notes(&settings.scale);
            },
            Mode::Chord => {

            },
            Mode::ReverseScale => {

            },
            Mode::ReverseChord => {

            },
        }
    }

    // Strings combo box
    let before_strings = guitar.config.num_strings; 
    egui::ComboBox::from_label("Num of strings")
        .selected_text(format!("{:?}", before_strings))
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut guitar.config.num_strings, 6, "6");
            ui.selectable_value(&mut guitar.config.num_strings, 7, "7");
            ui.selectable_value(&mut guitar.config.num_strings, 8, "8");
        });
    
    if before_strings != guitar.config.num_strings {
        match guitar.config.num_strings {
            6 => guitar.config.current_tuning = Tuning::standard_6_string(),
            7 => guitar.config.current_tuning = Tuning::standard_7_string(),
            8 => guitar.config.current_tuning = Tuning::standard_8_string(),
            _ => panic!("Bad number of strings"),
        }
        guitar.clear_notes();
    }

    // Tuning combo box
    let before_tuning = guitar.config.current_tuning.clone();
    egui::ComboBox::from_label("Current Tuning")
        .selected_text(format!("{:?}", before_tuning.name))
        .show_ui(ui, |ui| {
            match guitar.config.num_strings {
                6 => {
                    ui.selectable_value(&mut guitar.config.current_tuning, Tuning::standard_6_string(), "Standard 6");
                    ui.selectable_value(&mut guitar.config.current_tuning, Tuning::drop_d_6_string(), "Drop D");
                },
                7 => {
                    ui.selectable_value(&mut guitar.config.current_tuning, Tuning::standard_7_string(), "Standard 7");
                }
                8 => {
                    ui.selectable_value(&mut guitar.config.current_tuning, Tuning::standard_8_string(), "Standard 8");
                },
                _ => {
                    panic!("Bad number of strings");
                }
            }
        });

    // Actual String tunings
    ui.add_space(10.0);
    ui.label("String Tuning:");

    ui.horizontal(|ui| {
        for note in guitar.config.current_tuning.root_notes.iter_mut() {
            ui.vertical(|ui| {
                if ui.button("⬆").clicked() {
                    *note = note.add_semitones(1);
                    guitar.config.current_tuning.name = "Custom Tuning".to_string();
                }

                ui.label(format!("{}{}", note.name.to_string(), note.octave.to_string()));

                if ui.button("⬇").clicked() {
                    *note = note.add_semitones(-1);
                    guitar.config.current_tuning.name = "Custom Tuning".to_string();
                }

            });
        }
    });

    // Scale or chord
    ui.add_space(10.0);
    match settings.mode {
        Mode::Scale => {
            show_scale_settings(ui, guitar, settings);
        },
        Mode::Chord => {

        },
        _ => {}
    }


}


fn show_scale_settings(ui: &mut egui::Ui, guitar: &mut GuitarState, settings: &mut Settings) {
    // Root note
    let before_root = settings.scale.root;
    egui::ComboBox::from_label("Root Note")
    .selected_text(before_root.to_string())
    .show_ui(ui, |ui| {
        for note in NoteName::iter() {
            ui.selectable_value(&mut settings.scale.root, note, note.to_string());
        }
    });
    
    let before_scale_type = settings.scale.scale_type;
    egui::ComboBox::from_label("Scale Type")
    .selected_text(before_scale_type.to_string())
    .show_ui(ui, |ui| {
        for scale_type in ScaleType::iter() {
            ui.selectable_value(&mut settings.scale.scale_type, scale_type, scale_type.to_string());
        }
    });
    
    // New scale
    if before_root != settings.scale.root || before_scale_type != settings.scale.scale_type {
        guitar.update_scale_notes(&settings.scale);
    }
}