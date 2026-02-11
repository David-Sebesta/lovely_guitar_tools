use eframe::egui;
use strum::IntoEnumIterator;
use web_sys::js_sys::Set;
use crate::core_state::GuitarState;
use crate::core_state::Tuning;
use crate::core_state::NoteName;
use crate::core_state::MusicalStructure;
use crate::core_state::find_matching_chords;
use crate::core_state::find_matching_scales;
use crate::core_state::{Settings, Mode};

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
        match settings.mode {
            Mode::Scale => {
                guitar.update_notes(&settings.scale.notes());
            },
            Mode::Chord => {
                guitar.update_notes(&settings.chord.notes());
            },
            Mode::ReverseScale => {
                guitar.clear_notes();
            },
            Mode::ReverseChord => {
                guitar.clear_notes();
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
        match settings.mode {
            Mode::Scale => {
                guitar.update_notes(&settings.scale.notes());
            },
            Mode::Chord => {
                guitar.update_notes(&settings.chord.notes());
            },
            Mode::ReverseScale | Mode::ReverseChord => {
                guitar.shift_notes(guitar.config.num_strings as i8 - before_strings as i8);
            },
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

    // Mode specific
    ui.add_space(10.0);
    match settings.mode {
        Mode::Scale => {
            show_scale_settings(ui, guitar, settings);
        },
        Mode::Chord => {
            show_chord_settings(ui, guitar, settings);
        },
        Mode::ReverseScale => {
            show_reverse_scales(ui, guitar, settings);
        },
        Mode::ReverseChord => {
            show_reverse_chords(ui, guitar, settings);
        }
    }


}

fn show_scale_or_chord_selector<T> ( 
    ui: &mut egui::Ui,
    label_root: &str,
    label_type: &str,
    root: &mut NoteName, 
    structure_type: &mut T,
    get_name: impl Fn(&T) -> &str,
) -> bool
where T: IntoEnumIterator + PartialEq + Copy {
    
    let before_root = *root;
    let before_type = *structure_type;
    
    // Root Note
    egui::ComboBox::from_label(label_root)
    .selected_text(before_root.to_string())
    .show_ui(ui, |ui| {
        for note in NoteName::iter() {
            ui.selectable_value(root, note, note.to_string());
        }
    });

    // Type
    egui::ComboBox::from_label(label_type)
    .selected_text(get_name(&before_type))
    .show_ui(ui, |ui| {
        for t in T::iter() {
            let name = get_name(&t).to_string();
            ui.selectable_value(structure_type, t, name);
        }
    });

    // If something changed
    before_root != *root || before_type != *structure_type
}


fn show_scale_settings(ui: &mut egui::Ui, guitar: &mut GuitarState, settings: &mut Settings) {
    if show_scale_or_chord_selector(ui, "Root Note", "Scale Type", 
        &mut settings.scale.root, &mut settings.scale.scale_type,
        |t| t.to_string()) {
            guitar.update_notes(&settings.scale.notes());
        }
}


fn show_chord_settings(ui: &mut egui::Ui, guitar: &mut GuitarState, settings: &mut Settings) {
    if show_scale_or_chord_selector(ui, "Root Note", "Chord Type", 
        &mut settings.chord.root, &mut settings.chord.chord_type,
        |t| t.to_string()) {
            guitar.update_notes(&settings.chord.notes());
        }
}

fn show_reverse_scales(ui: &mut egui::Ui, guitar: &mut GuitarState, settings: &mut Settings) {
    let matching_scales = find_matching_scales(&guitar.get_active_note_names());
    for scale in matching_scales {
        ui.label(scale.to_string());
    }
}

fn show_reverse_chords(ui: &mut egui::Ui, guitar: &mut GuitarState, settings: &mut Settings) {
    let matching_chords = find_matching_chords(&guitar.get_active_note_names());
    for chord in matching_chords {
        ui.label(chord.to_string());
    }
}