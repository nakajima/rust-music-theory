use alloc::vec::Vec;

use crate::note::Pitch;
use crate::note::Tuning;

/// A note.
#[derive(Debug, Clone)]
pub struct Note {
    /// The pitch of the note (A, B, C#, etc).
    pub pitch: Pitch,
    /// The octave of the note in standard notation.
    pub octave: u8,
}

impl Note {
    /// Create a new note.

    pub fn new(pitch: Pitch, octave: u8) -> Note {
        Note { pitch, octave }
    }

    pub fn from_note_nr(nr: u8) -> Note {
        let pitch_class = Pitch::from_u8(nr % 12);
        let octave = nr / 12;
        Note::new(pitch_class, octave)
    }

    pub fn from_freq(freq: f32, tuning: Tuning) -> Note {
        match tuning {
            Tuning::EqualTemperament => {
                let a440 = Note::new(Pitch::from_str("A").unwrap(), 4).to_note_nr();
                let log2 = libm::log2f(freq / 440.0);
                Note::from_note_nr(((12.0 * log2) as i16 + a440 as i16) as u8)
            }
        }
    }

    pub fn to_note_nr(self) -> u8 {
        self.pitch.into_u8() + 12 * self.octave
    }

    pub fn to_freq(self, tuning: Tuning) -> f32 {
        match tuning {
            Tuning::EqualTemperament => {
                let a440 = Note::new(Pitch::from_str("A").unwrap(), 4).to_note_nr();
                let powf = libm::powf(
                    2.0,
                    ((self.to_note_nr() as i16 - a440 as i16) as f32) / 12.0,
                );
                powf * 440.0
            }
        }
    }
}

/// A type that can produce a sequence of notes.
pub trait Notes {
    /// Get the sequence of notes.
    fn notes(&self) -> Vec<Note>;
}
