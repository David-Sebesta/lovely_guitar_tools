use crate::core_state::guitar::{Tuning, GuitarConfig};
use crate::core_state::music_theory::{Note, NoteName};

#[test]
fn standard_6_get_all_notes() {
    let guitar = GuitarConfig::standard_6_string();
    assert_eq!(guitar.get_note_on_fretboard(0, 0), Note::new(NoteName::E, 2));
    assert_eq!(guitar.get_note_on_fretboard(1, 0), Note::new(NoteName::A, 2));
    assert_eq!(guitar.get_note_on_fretboard(2, 0), Note::new(NoteName::D, 3));
    assert_eq!(guitar.get_note_on_fretboard(3, 0), Note::new(NoteName::G, 3));
    assert_eq!(guitar.get_note_on_fretboard(4, 0), Note::new(NoteName::B, 3));
    assert_eq!(guitar.get_note_on_fretboard(5, 0), Note::new(NoteName::E, 4));
}

#[test]
fn standard_6_with_frets() {
    let guitar = GuitarConfig::standard_6_string();
    assert_eq!(guitar.get_note_on_fretboard(0, 1), Note::new(NoteName::F, 2));
    assert_eq!(guitar.get_note_on_fretboard(1, 1), Note::new(NoteName::ASharp, 2));
    assert_eq!(guitar.get_note_on_fretboard(2, 1), Note::new(NoteName::DSharp, 3));
    assert_eq!(guitar.get_note_on_fretboard(3, 1), Note::new(NoteName::GSharp, 3));
    assert_eq!(guitar.get_note_on_fretboard(4, 1), Note::new(NoteName::C, 4));
    assert_eq!(guitar.get_note_on_fretboard(5, 1), Note::new(NoteName::F, 4));
}


#[test]
fn standard_6_tuning_name() {
    let tuning = Tuning::standard_6_string();
    assert_eq!(tuning.to_string(), "EADGBE");
}