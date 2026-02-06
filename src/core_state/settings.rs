use super::guitar::GuitarConfig;


pub enum Mode {
    Scale,
    Chord,
}

pub struct Settings {
    pub guitar_config: GuitarConfig,
    pub mode: Mode, // Scale, chord...
}


impl Settings {
    pub fn new() -> Self {
        Self {
            guitar_config: GuitarConfig::standard_6_string(),
            mode: Mode::Scale,
        }
    }
}