/// An error caused when parsing a note.
#[derive(Debug, Clone)]
pub enum NoteError {
    InvalidPitch,
}

impl From<regex::Error> for NoteError {
    fn from(_: regex::Error) -> Self {
        NoteError::InvalidPitch
    }
}
