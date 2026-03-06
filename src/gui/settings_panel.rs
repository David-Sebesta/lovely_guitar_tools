use eframe::egui;
use egui::RichText;
use strum::IntoEnumIterator;
use crate::audio_engine;
use crate::audio_engine::AudioEngine;
use crate::core_state::GuitarState;
use crate::core_state::Tuning;
use crate::core_state::NoteName;
use crate::core_state::MusicalStructure;
use crate::core_state::{Settings, Mode};
use crate::gui::widget::toggle_switch;

pub fn show(ui: &mut egui::Ui, guitar: &mut GuitarState, settings: &mut Settings, audio_engine: &mut AudioEngine) {

    egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(20, 10))
        .show(ui, |ui| {
                ui.horizontal(|ui| {

                    show_guitar_settings(ui, guitar, settings);
                    show_mode_settings(ui, guitar, settings);
                    show_root_notes(ui, guitar, settings);
                    show_play_button(ui, guitar, settings, audio_engine);
            
                });

        });
}


fn show_guitar_settings(ui: &mut egui::Ui, guitar: &mut GuitarState, settings: &mut Settings) {
    egui::Frame::new()
    .stroke(ui.visuals().widgets.noninteractive.bg_stroke) 
    .corner_radius(4.0) 
    .inner_margin(8.0) 
    .show(ui, |ui| {
    
        let mut updated_tuning = false;
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
                            guitar.update_notes(&settings.scale.notes(), true);
                        },
                        Mode::Chord => {
                            guitar.update_notes(&settings.chord.notes(), true);
                        },
                        Mode::ReverseScale | Mode::ReverseChord => {
                            guitar.shift_notes(guitar.config.num_strings as i8 - before_num_strings as i8);
                        },
                    }
                    updated_tuning = true;
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
                    
                    if before_tuning != guitar.config.current_tuning {
                        updated_tuning = true;
                    }
            });


            ui.add_space(10.0);

            // Per String Tuning
            ui.horizontal(|ui| {
                for note in guitar.config.current_tuning.root_notes.iter_mut() {
                    ui.vertical(|ui| {
                        if ui.button("⬆").clicked() {
                            *note = note.add_semitones(1);
                            guitar.config.current_tuning.name = "Custom Tuning".to_string();
                            updated_tuning = true;
                        }

                        ui.label(format!("{}{}", note.name.to_string(), note.octave.to_string()));

                        if ui.button("⬇").clicked() {
                            *note = note.add_semitones(-1);
                            guitar.config.current_tuning.name = "Custom Tuning".to_string();
                            updated_tuning = true;
                        }

                    });
                }
            });
        });

        if updated_tuning {
            guitar.update_matching_structures();
        }
    
    });
    

}


fn show_play_button(ui: &mut egui::Ui, guitar: &mut GuitarState, settings: &mut Settings, audio_engine: &mut AudioEngine) {
    egui::Frame::new()
    .stroke(ui.visuals().widgets.noninteractive.bg_stroke) 
    .corner_radius(4.0) 
    .inner_margin(8.0) 
    .show(ui, |ui| {

        ui.style_mut().spacing.button_padding = egui::vec2(40.0, 5.0);

        let button = ui.button(RichText::new("Play").size(48.0).strong());
        if button.clicked() {
            match settings.mode {
                Mode::Chord => {
                    audio_engine.play_chord(&settings.chord.get_notes(3));
                },
                Mode::ReverseChord => {
                    audio_engine.play_chord(&guitar.get_active_notes());
                },
                Mode::Scale => {
                    audio_engine.play_scale(&mut settings.scale.get_notes(3), true, true);
                },
                Mode::ReverseScale => {
                    audio_engine.play_scale(&mut guitar.get_active_notes(), true, false);
                },
            }
        }

    });
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
        // Calculate the exact width of your two columns plus the gap between them
        let total_width = 80.0 + 60.0 + ui.spacing().item_spacing.x;
        
        // Force the entire block to exactly this width, centering everything inside it
        ui.allocate_ui_with_layout(
            egui::vec2(total_width, ui.available_height()), 
            egui::Layout::top_down(egui::Align::Center),
            |ui| {
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
                    match settings.mode {
                        Mode::Chord => {
                            guitar.update_notes(&settings.chord.notes(), true);
                            guitar.clear_greyed_notes();
                        },
                        Mode::Scale => {
                            guitar.update_notes(&settings.scale.notes(), true);
                            guitar.clear_greyed_notes();
                        },
                        _ => { guitar.clear_all_notes(); }
                    }

                }
                
                // Wrap the label in a centered layout
                ui.label(RichText::new(settings.mode.to_string()).size(16.0).strong());
            
        });
    });


}

fn show_root_notes(ui: &mut egui::Ui, guitar: &mut GuitarState, settings: &mut Settings) {
    
    let mut current_root_value = if settings.mode == Mode::Chord || settings.mode == Mode::ReverseChord {
        settings.chord.root 
    } else { 
        settings.scale.root 
    };

    let before_root = current_root_value;

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

                        let is_selected = current_root_value == root_note;

                        if ui.add_sized(
                            egui::vec2(button_width, 24.0),
                            egui::Button::selectable(is_selected, label)
                        ).clicked() {
                            current_root_value = root_note;
                            // TODO: Maybe make a sound later
                        }

                    }
                });

                show_mode_specific(ui, guitar, settings);
                
            }
        );
    });

    if before_root != current_root_value {
        // Changed root
        match settings.mode {
            Mode::Chord => {
                settings.chord.root = current_root_value;
                guitar.update_notes(&settings.chord.notes(), true);
            },
            Mode::Scale => {
                settings.scale.root = current_root_value;
                guitar.update_notes(&settings.scale.notes(), true);
            },
            _ => {}
        }

    }
}

fn show_mode_specific(ui: &mut egui::Ui, guitar: &mut GuitarState, settings: &mut Settings) {
    match settings.mode {
        Mode::Chord => {
            if show_scale_or_chord_type_selector(ui,  "Chord Type", 
                &mut settings.chord.chord_type, |t| t.to_string()) {
                    guitar.update_notes(&settings.chord.notes(), true);
            }
        },
        Mode::Scale => {
            if show_scale_or_chord_type_selector(ui, "Scale Type", 
                &mut settings.scale.scale_type, |t| t.to_string()) {
                    guitar.update_notes(&settings.scale.notes(), true);
            }
        },
        Mode::ReverseChord | Mode::ReverseScale => {
            if ui.button("Clear Notes").clicked() {
                guitar.clear_notes();
            }
        }
    }
}

fn show_scale_or_chord_type_selector<T> ( 
    ui: &mut egui::Ui,
    label_type: &str, 
    structure_type: &mut T,
    get_name: impl Fn(&T) -> &str,
) -> bool
where T: IntoEnumIterator + PartialEq + Copy {

    let before_type = *structure_type;
    
    ui.vertical_centered_justified(|ui| {
        ui.label(RichText::new(label_type).size(16.0).strong());
        egui::ComboBox::from_id_salt("cs_type_box")
        .selected_text(get_name(&before_type))
        .show_ui(ui, |ui| {
            for t in T::iter() {
                let name = get_name(&t).to_string();
                ui.vertical_centered_justified(|ui| {
                    ui.selectable_value(structure_type, t, name);
                });
            }
        });
    });

    before_type != *structure_type
}
