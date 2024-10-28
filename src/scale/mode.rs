/// The mode of a scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Also known as a major scale.
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    /// Also known as a natural minor scale.
    Aeolian,
    Locrian,
    HarmonicMinor,
    MelodicMinor,
}

impl Mode {
    /// Get whether the mode is diatonic (not harmonic or melodic minor).
    pub fn is_diatonic(self) -> bool {
        !matches!(self, Self::HarmonicMinor | Self::MelodicMinor)
    }
}
