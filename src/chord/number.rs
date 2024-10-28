use crate::chord::errors::ChordError;
use alloc::vec::Vec;
use lazy_static::lazy_static;
use regex::{Match, Regex};

lazy_static! {
    static ref NUMBER_REGEXES: Vec<(Regex, Number)> = {
        use Number::*;
        vec![
            (Regex::new("(?i)(triad)").unwrap(), Triad),
            (Regex::new("(?i)(seventh)").unwrap(), Seventh),
            (Regex::new(r"(?i)(major\s*seventh)").unwrap(), MajorSeventh),
            (Regex::new("(?i)(ninth)").unwrap(), Ninth),
            (Regex::new("(?i)(eleventh)").unwrap(), Eleventh),
            (Regex::new("(?i)(thirteenth)").unwrap(), Thirteenth),
        ]
    };
}

/// The superscript number after a chord.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number {
    Triad,
    Seventh,
    MajorSeventh,
    Ninth,
    Eleventh,
    Thirteenth,
}

impl Number {
    /// Parse the number using a regex.
    pub fn from_regex(string: &str) -> Result<(Self, Option<Match>), ChordError> {
        for (regex, number_enum) in &*NUMBER_REGEXES {
            let mode = regex.find(string);

            if let Some(number_match) = mode {
                return Ok((*number_enum, Some(number_match)));
            };
        }

        Err(ChordError::InvalidRegex)
    }
}
