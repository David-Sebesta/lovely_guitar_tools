use super::guitar::GuitarConfig;


pub struct Settings {
    pub guitar_config: GuitarConfig,
    //pub mode, // Scale, chords...
}


impl Settings {
    pub fn new() -> Self {
        Self {
            guitar_config: GuitarConfig::standard_6_string(),
        }
    }
}