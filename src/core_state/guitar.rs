use std::{collections::HashSet, ops::Index};
use super::music_theory::{Note, NoteName};


#[derive(Clone, PartialEq)]
pub struct Tuning {
    pub root_notes: Vec<Note>, 
    pub name: String,
}

impl Tuning {
    pub fn new(root_notes: Vec<Note>) -> Self {
        Self { 
            root_notes,
            name: String::from("Custom Tuning"),
        }
    }

    pub fn standard_6_string() -> Self {
        Self {
            root_notes: vec![
                Note::new(NoteName::E, 2),
                Note::new(NoteName::A, 2),
                Note::new(NoteName::D, 3),
                Note::new(NoteName::G, 3),
                Note::new(NoteName::B, 3),
                Note::new(NoteName::E, 4),
            ],
            name: String::from("Standard 6"),
        }
    }

    pub fn drop_d_6_string() -> Self {
        Self {
            root_notes: vec![
                Note::new(NoteName::D, 2),
                Note::new(NoteName::A, 2),
                Note::new(NoteName::D, 3),
                Note::new(NoteName::G, 3),
                Note::new(NoteName::B, 3),
                Note::new(NoteName::E, 4),
            ],
            
            name: String::from("Drop D"),
        }
    }

    pub fn standard_7_string() -> Self {
        Self {
            root_notes: vec![
                Note::new(NoteName::B, 1),
                Note::new(NoteName::E, 2),
                Note::new(NoteName::A, 2),
                Note::new(NoteName::D, 3),
                Note::new(NoteName::G, 3),
                Note::new(NoteName::B, 3),
                Note::new(NoteName::E, 4),
            ],
            name: String::from("Standard 7"),
        }
    }

    pub fn standard_8_string() -> Self {
        Self {
            root_notes: vec![
                Note::new(NoteName::FSharp, 1),
                Note::new(NoteName::B, 1),
                Note::new(NoteName::E, 2),
                Note::new(NoteName::A, 2),
                Note::new(NoteName::D, 3),
                Note::new(NoteName::G, 3),
                Note::new(NoteName::B, 3),
                Note::new(NoteName::E, 4),
            ],
            name: String::from("Standard 8"),
        }
    }

    pub fn to_string(&self) -> String {
        let mut note_names = String::new();

        for note in &self.root_notes {
            note_names += note.name.to_string();
        }

        note_names
    }

}

impl Index<usize> for Tuning {
    type Output = Note;

    fn index(&self, index: usize) -> &Self::Output{
        &self.root_notes[index]
    }

}


pub struct GuitarConfig {
    pub num_strings: u8,
    pub current_tuning: Tuning,

}

impl GuitarConfig {
    pub fn standard_6_string() -> Self {
        Self {
            num_strings: 6,
            current_tuning: Tuning::standard_6_string()
        }
    }

    pub fn drop_d_6_string() -> Self {
        Self {
            num_strings: 6,
            current_tuning: Tuning::drop_d_6_string()
        }
    }

}


pub struct GuitarState {
    pub config: GuitarConfig,
    pub active_frets: HashSet<(u8, u8)>, // String to fret
    pub active_note: Option<Note>,
}

impl GuitarState {
    pub fn new() -> Self {
        Self {
            config: GuitarConfig::standard_6_string(),
            active_frets: HashSet::new(),
            active_note: Option::None,
        }
    }

    pub fn get_note_on_fretboard(&self, string: u8, fret: u8) -> Note {
        if (string as usize) >= self.config.current_tuning.root_notes.len() {
            panic!("String out of bounds");
        }

        let root_note = self.config.current_tuning[string as usize];
        root_note.add_semitones(fret as i8)
    }

    pub fn clear_notes(&mut self) {
        self.active_frets.clear();
    }

    pub fn toggle_note(&mut self, string: u8, fret: u8) {
        if self.active_frets.contains(&(string, fret)) {
            self.active_frets.remove(&(string, fret));
        } else {
            self.active_frets.insert((string, fret));
        }
    }

    // Clears all other notes on string
    pub fn set_strings_note(&mut self, string: u8, fret: u8) {
        self.active_frets.retain(|(s, _)| *s != string);
        self.active_frets.insert((string, fret));
    }


}


