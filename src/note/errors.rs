/// An error caused when parsing a note.
#[derive(Debug, Clone)]
pub enum NoteError {
    InvalidPitch,
}

impl From<regex::Error> for NoteError {
    fn from(e: regex::Error) -> Self {
        match e {
            _ => NoteError::InvalidPitch,
        }
    }
}
