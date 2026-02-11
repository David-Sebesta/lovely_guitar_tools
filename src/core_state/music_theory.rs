use strum::{EnumIter, IntoEnumIterator};

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

// 
pub trait HasIntervals {
    fn intervals(&self) -> Vec<i8>;
}

// Musical Structure trait has roots and intervals
// It is basically a collection of notes based on the root and interval between the root and notes
pub trait MusicalStructure {
    fn root(&self) -> NoteName;
    fn intervals(&self) -> Vec<i8>;

    fn notes(&self) -> Vec<NoteName> {
        let root_val = self.root() as i8;

        self.intervals().iter().map(|&interval| {
            let note_val = (root_val + interval).rem_euclid(NoteName::TOTAL as i8);
            NoteName::from_u8(note_val as u8)
        }).collect()
    }
    
    fn contains(&self, note: NoteName) -> bool {
        self.notes().contains(&note)
    }
    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum ScaleType {
    Major,
    Minor,
    PentatonicMajor,
    PentatonicMinor,
    Blues,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Locrian,
    HarmonicMinor,
    MelodicMinor,
    WholeTone,
    DoubleHarmonic,
    HungarianMinor,
    SuperLocrian,
    PhrygianDominant,
    LydianAugmented,
    LydianDominant,
    MixolydianFlatSix,
    LocrianSharpTwo,
    DiminishedWholeHalf,
    DiminishedHalfWhole,
    NeapolitanMajor,
    NeapolitanMinor,
    Enigmatic,
    Hirajoshi,
    InSen,
    Yo,
    Persian,
}

impl ScaleType {
    pub fn to_string(&self) -> &'static str {
        match self {
            ScaleType::Major => "Major",
            ScaleType::Minor => "Minor",
            ScaleType::PentatonicMajor => "Pentatonic Major",
            ScaleType::PentatonicMinor => "Pentatonic Minor",
            ScaleType::Blues => "Blues",
            ScaleType::Dorian => "Dorian",
            ScaleType::Phrygian => "Phrygian",
            ScaleType::Lydian => "Lydian",
            ScaleType::Mixolydian => "Mixolydian",
            ScaleType::Locrian => "Locrian",
            ScaleType::HarmonicMinor => "Harmonic Minor",
            ScaleType::MelodicMinor => "Melodic Minor",
            ScaleType::WholeTone => "Whole Tone",
            ScaleType::DoubleHarmonic => "Double Harmonic",
            ScaleType::HungarianMinor => "Hungarian Minor",
            ScaleType::SuperLocrian => "Super Locrian (Altered)",
            ScaleType::PhrygianDominant => "Phrygian Dominant",
            ScaleType::LydianAugmented => "Lydian Augmented",
            ScaleType::LydianDominant => "Lydian Dominant",
            ScaleType::MixolydianFlatSix => "Mixolydian b6 (Hindu)",
            ScaleType::LocrianSharpTwo => "Locrian #2",
            ScaleType::DiminishedWholeHalf => "Diminished (Whole-Half)",
            ScaleType::DiminishedHalfWhole => "Diminished (Half-Whole)",
            ScaleType::NeapolitanMajor => "Neapolitan Major",
            ScaleType::NeapolitanMinor => "Neapolitan Minor",
            ScaleType::Enigmatic => "Enigmatic",
            ScaleType::Hirajoshi => "Hirajoshi",
            ScaleType::InSen => "In Sen",
            ScaleType::Yo => "Yo",
            ScaleType::Persian => "Persian",
        }
    }
}

impl HasIntervals for ScaleType {
    fn intervals(&self) -> Vec<i8> {
        match self {
            ScaleType::Major => vec![0, 2, 4, 5, 7, 9, 11],
            ScaleType::Minor => vec![0, 2, 3, 5, 7, 8, 10],
            ScaleType::PentatonicMajor => vec![0, 2, 4, 7, 9],
            ScaleType::PentatonicMinor => vec![0, 3, 5, 7, 10],
            ScaleType::Blues => vec![0, 3, 5, 6, 7, 10],
            ScaleType::Dorian => vec![0, 2, 3, 5, 7, 9, 10],
            ScaleType::Phrygian => vec![0, 1, 3, 5, 7, 8, 10],
            ScaleType::Lydian => vec![0, 2, 4, 6, 7, 9, 11],
            ScaleType::Mixolydian => vec![0, 2, 4, 5, 7, 9, 10],
            ScaleType::Locrian => vec![0, 1, 3, 5, 6, 8, 10],
            ScaleType::HarmonicMinor => vec![0, 2, 3, 5, 7, 8, 11],
            ScaleType::MelodicMinor => vec![0, 2, 3, 5, 7, 9, 11],
            ScaleType::WholeTone => vec![0, 2, 4, 6, 8, 10],
            ScaleType::DoubleHarmonic => vec![0, 1, 4, 5, 7, 8, 11],
            ScaleType::HungarianMinor => vec![0, 2, 3, 6, 7, 8, 11],
            ScaleType::SuperLocrian => vec![0, 1, 3, 4, 6, 8, 10],
            ScaleType::PhrygianDominant => vec![0, 1, 4, 5, 7, 8, 10],
            ScaleType::LydianAugmented => vec![0, 2, 4, 6, 8, 9, 11],
            ScaleType::LydianDominant => vec![0, 2, 4, 6, 7, 9, 10],
            ScaleType::MixolydianFlatSix => vec![0, 2, 4, 5, 7, 8, 10],
            ScaleType::LocrianSharpTwo => vec![0, 2, 3, 5, 6, 8, 10],
            ScaleType::DiminishedWholeHalf => vec![0, 2, 3, 5, 6, 8, 9, 11],
            ScaleType::DiminishedHalfWhole => vec![0, 1, 3, 4, 6, 7, 9, 10],
            ScaleType::NeapolitanMajor => vec![0, 1, 3, 5, 7, 9, 11],
            ScaleType::NeapolitanMinor => vec![0, 1, 3, 5, 7, 8, 11],
            ScaleType::Enigmatic => vec![0, 1, 5, 6, 8, 9, 11],
            ScaleType::Hirajoshi => vec![0, 2, 3, 7, 8],
            ScaleType::InSen => vec![0, 1, 5, 7, 10],
            ScaleType::Yo => vec![0, 2, 5, 7, 9],
            ScaleType::Persian => vec![0, 1, 4, 5, 6, 8, 11],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Scale {
    pub root: NoteName,
    pub scale_type: ScaleType,
}

impl Scale {
    pub fn new(root: NoteName, scale_type: ScaleType) -> Self {
        Self {root, scale_type}
    }

    pub fn to_string(&self) -> String {
        format!{"{} {}", self.root.to_string(), self.scale_type.to_string()}
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

impl MusicalStructure for Scale {
    fn root(&self) -> NoteName {
        self.root
    }

    fn intervals(&self) -> Vec<i8> {
        self.scale_type.intervals()
    }
}

// Chords
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum ChordType {
    Major,
    Minor,
    Diminished,
    Augmented,
    MajorSeven,
    DominantSeven,
    MinorSeven,
    SuspendedTwo,
    SuspendedFour,
    MinorSevenFlatFive,
    DiminishedSeven,
    MajorNine,
    MinorNine,
}

impl ChordType {
    pub fn to_string(&self) -> &'static str {
        match self {
            ChordType::Major => "Major",
            ChordType::Minor => "Minor",
            ChordType::Diminished => "Diminished",
            ChordType::Augmented => "Augmented",
            ChordType::MajorSeven => "Major 7",
            ChordType::DominantSeven => "Dominant 7",
            ChordType::MinorSeven => "Minor 7",
            ChordType::SuspendedTwo => "Sus 2",
            ChordType::SuspendedFour => "Sus 4",
            ChordType::MinorSevenFlatFive => "m7b5",
            ChordType::DiminishedSeven => "Diminished 7",
            ChordType::MajorNine => "Major 9",
            ChordType::MinorNine => "Minor 9",
        }
    }
}

impl HasIntervals for ChordType {
    fn intervals(&self) -> Vec<i8> {
        match self {
            ChordType::Major => vec![0, 4, 7],
            ChordType::Minor => vec![0, 3, 7],
            ChordType::Diminished => vec![0, 3, 6],
            ChordType::Augmented => vec![0, 4, 8],
            ChordType::MajorSeven => vec![0, 4, 7, 11],
            ChordType::DominantSeven => vec![0, 4, 7, 10],
            ChordType::MinorSeven => vec![0, 3, 7, 10],
            ChordType::SuspendedTwo => vec![0, 2, 7],
            ChordType::SuspendedFour => vec![0, 5, 7],
            ChordType::MinorSevenFlatFive => vec![0, 3, 6, 10],
            ChordType::DiminishedSeven => vec![0, 3, 6, 9],
            ChordType::MajorNine => vec![0, 4, 7, 11, 2],
            ChordType::MinorNine => vec![0, 3, 7, 10, 2],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Chord {
    pub root: NoteName,
    pub chord_type: ChordType,
    pub position: u8,
}

impl Chord {
    pub fn new(root: NoteName, chord_type: ChordType) -> Self {
        Self { root, chord_type, position: 0}
    }

    pub fn to_string(&self) -> String { 
        format!{"{} {}", self.root.to_string(), self.chord_type.to_string()}
    }
}

impl Default for Chord {
    fn default() -> Self {
        Self { 
            root: NoteName::C, 
            chord_type: ChordType::Major,
            position: 1,
        }
    }
}

impl MusicalStructure for Chord {
    fn root(&self) -> NoteName {
        self.root
    }

    fn intervals(&self) -> Vec<i8> {
        self.chord_type.intervals()
    }
}


// Reverse Scales and Chords
pub fn find_matching_scales(notes: &[NoteName]) -> Vec<Scale> {
    let mut matching_scales: Vec<Scale> = Vec::new();

    for root in notes {
        for scale_type in ScaleType::iter() {
            let scale = Scale::new(*root, scale_type);
            if notes.iter().all(|note| scale.contains(*note)) {
                matching_scales.push(scale);
            }
        }
    }

    matching_scales
}

pub fn find_matching_chords(notes: &[NoteName]) -> Vec<Chord> {
    let mut matching_chords: Vec<Chord> = Vec::new();
    
    for root in notes {
        for chord_type in ChordType::iter() {
            let chord = Chord::new(*root, chord_type);
            if notes.iter().all(|note| chord.contains(*note)) {
                matching_chords.push(chord);
            }
        }
    }

    matching_chords
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

    #[test]
    fn test_matching_scales() {
        let c_major_scale = Scale::new(NoteName::C, ScaleType::Major);
        let matching_scales = find_matching_scales(&c_major_scale.notes());
        
        // The first one should be C Major always
        // Then it should also have a A Minor after
        assert_eq!(Scale::new(NoteName::C, ScaleType::Major), matching_scales[0]);
        assert_eq!(true, matching_scales.contains(&Scale::new(NoteName::A, ScaleType::Minor)));
    }

    #[test]
    fn test_matching_chords() {
        let c_major_chord = Chord::new(NoteName::C, ChordType::Major);
        let matching_chords = find_matching_chords(&c_major_chord.notes());
        
        // The first one should be C Major always
        // Then it should also have a A Minor after
        assert_eq!(Chord::new(NoteName::C, ChordType::Major), matching_chords[0]);
        assert_eq!(true, matching_chords.contains(&Chord::new(NoteName::C, ChordType::MajorSeven)));
    }


}