use crate::note::NoteError;

/// An error while parsing a chord.
#[derive(Debug, Clone)]
pub enum ChordError {
    InvalidRegex,
}

impl From<NoteError> for ChordError {
    fn from(_: NoteError) -> Self {
        ChordError::InvalidRegex
    }
}
