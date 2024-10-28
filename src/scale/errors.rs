use crate::interval::IntervalError;
use crate::note::NoteError;

#[derive(Debug, Clone)]
pub enum ScaleError {
    InvalidInterval,
    ModeFromRegex,
    InvalidRegex,
}

impl From<regex::Error> for ScaleError {
    fn from(e: regex::Error) -> Self {
        match e {
            _ => ScaleError::ModeFromRegex,
        }
    }
}

impl From<NoteError> for ScaleError {
    fn from(e: NoteError) -> Self {
        match e {
            _ => ScaleError::InvalidRegex,
        }
    }
}

impl From<IntervalError> for ScaleError {
    fn from(e: IntervalError) -> Self {
        match e {
            _ => ScaleError::InvalidInterval,
        }
    }
}
