use crate::scale::{Mode, Mode::*};
use strum_macros::{Display, EnumIter};

/// The type of a scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScaleType {
    Diatonic,
    MelodicMinor,
    HarmonicMinor,
}

impl ScaleType {
    /// Get the scale type from the mode.
    pub fn from_mode(mode: Mode) -> Self {
        use ScaleType::*;
        match mode {
            Ionian => Diatonic,
            Aeolian => Diatonic,
            Mode::HarmonicMinor => ScaleType::HarmonicMinor,
            Mode::MelodicMinor => ScaleType::MelodicMinor,
            _ => Diatonic,
        }
    }
}

impl From<Mode> for ScaleType {
    fn from(mode: Mode) -> Self {
        Self::from_mode(mode)
    }
}
