use super::music_theory::{Scale};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mode {
    Scale,
    Chord,
    ReverseScale,
    ReverseChord,
}

impl Mode {
    pub fn to_string(&self) -> &'static str {
        match self {
            Mode::Scale => "Scale",
            Mode::Chord => "Chord",
            Mode::ReverseScale => "Reverse Scale",
            Mode::ReverseChord => "Reverse Chord",
        }
    }
}

pub struct Settings {
    pub mode: Mode, // Scale, chord...
    pub scale: Scale,
    pub debug: bool,
}


impl Settings {
    pub fn new() -> Self {
        Self {
            mode: Mode::ReverseScale,
            scale: Scale::default(),
            debug: false,
        }
    }
}