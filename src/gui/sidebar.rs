use eframe::egui;
use crate::core_state::GuitarState;
use crate::core_state::Tuning;
use crate::core_state::Settings;

pub fn show(ui: &mut egui::Ui, guitar: &mut GuitarState, settings: &mut Settings) {
    ui.horizontal(|ui| {
        ui.heading("Settings");
        ui.checkbox(&mut settings.debug, "Debug");
    });

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


}
