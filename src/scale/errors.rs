use crate::interval::IntervalError;
use crate::note::NoteError;

#[derive(Debug, Clone)]
pub enum ScaleError {
    InvalidInterval,
    ModeFromRegex,
    InvalidRegex,
}

impl From<NoteError> for ScaleError {
    fn from(_: NoteError) -> Self {
        ScaleError::InvalidRegex
    }
}

impl From<IntervalError> for ScaleError {
    fn from(_: IntervalError) -> Self {
        ScaleError::InvalidInterval
    }
}
