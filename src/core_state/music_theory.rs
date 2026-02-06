#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NoteName {
    C = 0,
    CSharp = 1,
    D = 2,
    DSharp = 3,
    E = 4,
    F = 5,
    FSharp = 6,
    G = 7,
    GSharp = 8,
    A = 9,
    ASharp = 10,
    B = 11,
}

impl NoteName {
    pub const TOTAL: u8 = 12;

    pub fn from_u8(v: u8) -> Self {
        match v % 12 {
            0 => NoteName::C,
            1 => NoteName::CSharp,
            2 => NoteName::D,
            3 => NoteName::DSharp,
            4 => NoteName::E,
            5 => NoteName::F,
            6 => NoteName::FSharp,
            7 => NoteName::G,
            8 => NoteName::GSharp,
            9 => NoteName::A,
            10 => NoteName::ASharp,
            11 => NoteName::B,
            _ => unreachable!(),
        }
    }
    
    pub fn to_string(&self) -> &'static str {
        match self {
            NoteName::C => "C",
            NoteName::CSharp => "C#",
            NoteName::D => "D",
            NoteName::DSharp => "D#",
            NoteName::E => "E",
            NoteName::F => "F",
            NoteName::FSharp => "F#",
            NoteName::G => "G",
            NoteName::GSharp => "G#",
            NoteName::A => "A",
            NoteName::ASharp => "A#",
            NoteName::B => "B",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Note {
    pub name: NoteName,
    pub octave: i8,
}

impl Note {
    pub fn new(name: NoteName, octave: i8) -> Self {
        Self { name, octave }
    }

    pub fn add_semitones(&self, semitones: i8) -> Self {
        let current_val = self.name as i8;
        let total_semitones = current_val + (self.octave as i8 * 12) + semitones;
        
        let new_octave = total_semitones.div_euclid(12);
        let new_note_val = total_semitones.rem_euclid(12);
        
        Self {
            name: NoteName::from_u8(new_note_val as u8),
            octave: new_octave,
        }
    }

    // A4 = 440Hz
    pub fn frequency(&self) -> f32 {
        let a4 = Note::new(NoteName::A, 4);
        let semitones_from_a4 = self.semitones_from(&a4);
        440.0 * 2.0_f32.powf(semitones_from_a4 as f32 / 12.0)
    }

    pub fn semitones_from(&self, other: &Note) -> i32 {
        let self_val = (self.name as i32) + (self.octave as i32 * 12);
        let other_val = (other.name as i32) + (other.octave as i32 * 12);
        self_val - other_val
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScaleType {
    Major,
    Minor,
    PentatonicMinor,
    PentatonicMajor,
    Blues,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChordType {
    Major,
    Minor,
    Diminished,
    Augmented,
    Seven,
}