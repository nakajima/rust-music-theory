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
