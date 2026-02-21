use eframe::egui;
use strum::IntoEnumIterator;
use crate::core_state::GuitarState;
use crate::core_state::Tuning;
use crate::core_state::NoteName;
use crate::core_state::MusicalStructure;
use crate::core_state::{Settings, Mode};
use crate::gui::widget::toggle_switch;

pub fn show(ui: &mut egui::Ui, guitar: &mut GuitarState, settings: &mut Settings) {

    egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(20, 10))
        .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let total_width = ui.available_width();
            
                    // Left
                    ui.allocate_ui_with_layout(egui::vec2(total_width / 3.0, ui.available_height()), 
                    egui::Layout::left_to_right(egui::Align::Center), 
                    |ui| {
                        show_guitar_settings(ui, guitar, settings);
                    });
            
                    // Center
                    ui.allocate_ui_with_layout(egui::vec2(total_width / 3.0, ui.available_height()), 
                    egui::Layout::left_to_right(egui::Align::Center), 
                    |ui| {
                        show_mode_settings(ui, guitar, settings);
                        show_root_notes(ui, guitar, settings);
                    });
            
                    // Right
                    ui.allocate_ui_with_layout(egui::vec2(total_width / 3.0, ui.available_height()), 
                    egui::Layout::right_to_left(egui::Align::Center), 
                    |ui| {
                        show_play_button(ui, guitar, settings);
                    });
            
            
                });

        });
}


fn show_guitar_settings(ui: &mut egui::Ui, guitar: &mut GuitarState, settings: &mut Settings) {
    
    egui::Frame::new()
    .stroke(ui.visuals().widgets.noninteractive.bg_stroke) 
    .corner_radius(4.0) 
    .inner_margin(8.0) 
    .show(ui, |ui| {
    
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                // Number of strings
                let before_num_strings = guitar.config.num_strings;
                ui.vertical(|ui| {
                    ui.label("Number of Strings");
                    ui.add(
                        egui::Slider::new(&mut guitar.config.num_strings, 6..=8)
                            .show_value(true)
                            .step_by(1.0)
                    );
                });

                if before_num_strings != guitar.config.num_strings {
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
                            guitar.shift_notes(guitar.config.num_strings as i8 - before_num_strings as i8);
                        },
                    }
                }

                ui.add_space(20.0);

                // Tuning
                let before_tuning = guitar.config.current_tuning.clone();
                ui.vertical(|ui| {
                    ui.label("Current Tuning");
                    egui::ComboBox::from_id_salt("tuning_dropdown")
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
                    });
            });

            ui.add_space(10.0);

            // Per String Tuning
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
        });
    
    });
    

}


fn show_play_button(ui: &mut egui::Ui, guitar: &mut GuitarState, settings: &mut Settings) {

}


fn show_mode_settings(ui: &mut egui::Ui, guitar: &mut GuitarState, settings: &mut Settings) {

    let is_chord_mode = settings.mode == Mode::Chord || settings.mode == Mode::ReverseChord; 
    let is_reverse_mode = settings.mode == Mode::ReverseScale || settings.mode == Mode::ReverseChord; 
    
    let mut mode_switch = is_chord_mode;
    let mut reverse_switch = is_reverse_mode;
    
    egui::Frame::new()
    .stroke(ui.visuals().widgets.noninteractive.bg_stroke) 
    .corner_radius(4.0) 
    .inner_margin(8.0) 
    .show(ui, |ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            // Chord/Scale
            ui.allocate_ui_with_layout(
                egui::vec2(80.0, ui.available_height()), 
                egui::Layout::top_down(egui::Align::Center),
            |ui| {
                    ui.label(egui::RichText::new("CHORD").strong());
                    toggle_switch(ui, &mut mode_switch, true);
                    ui.label(egui::RichText::new("SCALE").strong());
            });

            // Reverse
            ui.allocate_ui_with_layout(
                egui::vec2(60.0, ui.available_height()), 
                egui::Layout::top_down(egui::Align::Center),
            |ui| {
                    ui.label(egui::RichText::new("REVERSE").strong());
                    toggle_switch(ui, &mut reverse_switch, true);
                    ui.add_space(12.0);
            });
        });
    });

    // Update logic. I already made it so Mode has all four modes and I'm not going to refactor everything
    // to split it up into Chord/Scale and Reverse, so I'm going to just use this match statement
    let new_mode = match (mode_switch, reverse_switch) {
        (true, true)   => Mode::ReverseChord,
        (true, false)  => Mode::Chord,
        (false, true)  => Mode::ReverseScale,
        (false, false) => Mode::Scale,
    };

    if new_mode != settings.mode {
        // New mode
        settings.mode = new_mode;
    }

}

fn show_root_notes(ui: &mut egui::Ui, guitar: &mut GuitarState, settings: &mut Settings) {
    
    let mut current_root = if settings.mode == Mode::Chord || settings.mode == Mode::ReverseChord {
        &mut settings.chord.root 
    } else { 
        &mut settings.scale.root 
    };

    let before_root = *current_root;

    let button_width = 30.0;
    
    egui::Frame::new()
    .stroke(ui.visuals().widgets.noninteractive.bg_stroke) 
    .corner_radius(4.0) 
    .inner_margin(8.0) 
    .show(ui, |ui| {
        ui.allocate_ui_with_layout(
            egui::vec2(60.0 + button_width * NoteName::TOTAL as f32, ui.available_height()),
            egui::Layout::top_down(egui::Align::Center),
            |ui| {
                ui.label(egui::RichText::new("ROOT NOTE").size(16.0).strong());
                
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 6.0;
                    
                    for root_note in NoteName::iter() {
                        let label = root_note.to_string();

                        let is_selected = *current_root == root_note;

                        if ui.add_sized(
                            egui::vec2(button_width, 24.0),
                            egui::Button::selectable(is_selected, label)
                        ).clicked() {
                            *current_root = root_note;
                            // TODO: Maybe make a sound later
                        }

                    }
                });
            }
        );
    });

    if before_root != *current_root {
        // Changed root
        match settings.mode {
            Mode::Chord => {

            },
            Mode::Scale => {

            },
            _ => {}
        }

    }
}