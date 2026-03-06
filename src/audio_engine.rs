use egui::util::id_type_map::TypeId;
use web_sys::{AudioContext, OscillatorType, BiquadFilterType};
use wasm_bindgen::JsValue;
use crate::core_state::{MusicalStructure, music_theory::Note, Scale, Chord};

pub struct AudioEngine {
    ctx: Option<AudioContext>,
    _metronome_active: bool,
    _bpm: u32,
    _next_click_time: f64,
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self {
            ctx: None,
            _metronome_active: false,
            _bpm: 120,
            _next_click_time: 0.0,
        }
    }
}

impl AudioEngine {
    pub fn new() -> Self {
        Self::default()
    }

    // Initalize the AudioContext
    pub fn init(&mut self) -> Result<(), JsValue> {
        if self.ctx.is_none() {
            let ctx = AudioContext::new()?;
            self.ctx = Some(ctx);
        }

        // Resume if suspended
        if let Some(ctx) = &self.ctx {
            if ctx.state() == web_sys::AudioContextState::Suspended {
                let _ = ctx.resume();
            }
        }
        Ok(())
    }

    fn play_frequency(&self, freq: f32, start_time: f64, duration: f64) -> Result<(), JsValue> {
        let ctx = match &self.ctx {
            Some(c) => c,
            None => return Ok(()) // Silent fail
        };

        // Sawtooth
        let osc = ctx.create_oscillator()?;
        osc.set_type(OscillatorType::Sawtooth);
        osc.frequency().set_value(freq);
        
        // Filter
        let filter = ctx.create_biquad_filter()?;
        filter.set_type(BiquadFilterType::Lowpass);
        filter.q().set_value(1.0);

        // Filter Envelope
        let t = start_time;
        let filter_freq = filter.frequency();
        filter_freq.set_value_at_time(3000.0, t)?;
        filter_freq.exponential_ramp_to_value_at_time(500.0, t + 0.1)?;

        // Gain
        let gain = ctx.create_gain()?;
        let gain_val = gain.gain();

        // Attack
        gain_val.set_value_at_time(0.0, t)?;
        gain_val.linear_ramp_to_value_at_time(0.4, t + 0.02)?;

        // Decay
        gain_val.exponential_ramp_to_value_at_time(0.001, t + duration)?;

        // Connect graph
        osc.connect_with_audio_node(&filter)?;
        filter.connect_with_audio_node(&gain)?;
        gain.connect_with_audio_node(&ctx.destination())?;

        // Schedule start and stop
        osc.start_with_when(t)?;
        osc.stop_with_when(t + duration)?;

        Ok(())

    }

    pub fn play_note(&mut self, note: &Note) {
        if let Err(_) = self.init() { return; }

        if let Some(ctx) = &self.ctx {
            let now = ctx.current_time();
            let _ = self.play_frequency(note.frequency(), now, 1.5);
        }

    }

    pub fn play_chord(&mut self, notes: &[Note]) {
        if let Err(_) = self.init() { return; }

        if let Some(ctx) = &self.ctx {
            let now = ctx.current_time();
            let strum_speed = 0.01; // 30ms

            for (i, note) in notes.iter().enumerate() {
                let start_time = now + (i as f64 * strum_speed);
                let _ = self.play_frequency(note.frequency(), start_time, 2.0);
            }

        }
    }

    pub fn play_scale(&mut self, notes: &mut Vec<Note>, ascending: bool, add_octave: bool) {
        if let Err(_) = self.init() { return; }

        if let Some(ctx) = &self.ctx {
            if add_octave {
                if let Some(n) = notes.first().map(|n| n.add_semitones(12)) {
                    notes.push(n);
                }
            }

            let now = ctx.current_time();
            let note_duration = 0.2;

            let ordered_notes: Vec<&Note> = if ascending {
                notes.iter().collect()
            } else {
                notes.iter().rev().collect()
            };


            for (i, note) in ordered_notes.iter().enumerate() {
                let start_time = now + (i as f64 * note_duration);
                let _ = self.play_frequency(note.frequency(), start_time, 0.6);
            }
        }
    }

    pub fn play_musical_structure<T>(&mut self, musical_structure: &T) 
        where T: MusicalStructure + 'static, {
        let mut notes = musical_structure.get_notes(3);

        let type_id = TypeId::of::<T>();
        if type_id == TypeId::of::<Scale>() {
            self.play_scale(&mut notes, true, true);
        } else if type_id == TypeId::of::<Chord>() {
            self.play_chord(&notes);
        }
    }

    // Metronome


}