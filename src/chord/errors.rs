use crate::note::NoteError;

/// An error while parsing a chord.
#[derive(Debug, Clone)]
pub enum ChordError {
    InvalidRegex,
}

impl From<NoteError> for ChordError {
    fn from(e: NoteError) -> Self {
        match e {
            _ => ChordError::InvalidRegex,
        }
    }
}

impl From<regex::Error> for ChordError {
    fn from(e: regex::Error) -> Self {
        match e {
            _ => ChordError::InvalidRegex,
        }
    }
}
