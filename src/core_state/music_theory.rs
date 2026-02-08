use strum::{IntoEnumIterator, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
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

    pub fn to_string(&self) -> String { 
        format!("{}{}", self.name.to_string(), self.octave.to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum ScaleType {
    Major,
    Minor,
    PentatonicMinor,
    PentatonicMajor,
    Blues,
}

impl ScaleType {
    pub fn intervals(&self) -> Vec<i8> {
        match self {
            ScaleType::Major => vec![0, 2, 4, 5, 7, 9, 11],
            ScaleType::Minor => vec![0, 2, 3, 5, 7, 8, 10],
            ScaleType::PentatonicMajor => vec![0, 3, 5, 7, 10],
            ScaleType::PentatonicMinor => vec![0, 2, 4, 7, 9],
            ScaleType::Blues => vec![0, 4, 5, 6, 7, 10],
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            ScaleType::Major => "Major",
            ScaleType::Minor => "Minor",
            ScaleType::PentatonicMajor => "Pentatonic Major",
            ScaleType::PentatonicMinor => "Pentatonic Minor",
            ScaleType::Blues => "Blues",
        }
    }
}

pub struct Scale {
    pub root: NoteName,
    pub scale_type: ScaleType,
}

impl Scale {
    pub fn new(root: NoteName, scale_type: ScaleType) -> Self {
        Self {root, scale_type}
    }

    pub fn notes(&self) -> Vec<NoteName> {
        let intervals = self.scale_type.intervals();
        intervals.iter().map(|&interval| {
                let root_val = self.root as i8;
                let note_val = (root_val + interval).rem_euclid(NoteName::TOTAL as i8);
                NoteName::from_u8(note_val as u8)
        }).collect()
    }

    pub fn contains(&self, note: NoteName) -> bool {
        self.notes().contains(&note)
    }
}

impl Default for Scale {
    fn default() -> Self {
        Self {
            root: NoteName::C,
            scale_type: ScaleType::Major,
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum ChordType {
    Major,
    Minor,
    Diminished,
    Augmented,
    Seven,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_round_trip() {
        let scale = Scale::new(NoteName::C, ScaleType::Major);
        assert_eq!(true,  scale.contains(NoteName::C));
        assert_eq!(false, scale.contains(NoteName::CSharp));
        assert_eq!(true,  scale.contains(NoteName::D));
        assert_eq!(false, scale.contains(NoteName::DSharp));
        assert_eq!(true,  scale.contains(NoteName::E));
        assert_eq!(true,  scale.contains(NoteName::F));
        assert_eq!(false, scale.contains(NoteName::FSharp));
        assert_eq!(true,  scale.contains(NoteName::G));
        assert_eq!(false, scale.contains(NoteName::GSharp));
        assert_eq!(true,  scale.contains(NoteName::A));
        assert_eq!(false, scale.contains(NoteName::ASharp));
        assert_eq!(true,  scale.contains(NoteName::B));
    }


}