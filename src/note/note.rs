use alloc::vec::Vec;

use crate::note::Pitch;

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
    pub fn new(pitch: Pitch, octave: u8) -> Self {
        Note { pitch, octave }
    }
}

/// A type that can produce a sequence of notes.
pub trait Notes {
    /// Get the sequence of notes.
    fn notes(&self) -> Vec<Note>;
}
