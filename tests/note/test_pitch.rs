extern crate rust_music_theory as theory;
use theory::note::{NoteLetter::*, Pitch};

#[cfg(test)]
mod test_note {
    use super::*;

    #[test]
    fn test_pitch_from_str_err() {
        for string in ["Ca", "Q", "Cb#", "B♯b#"] {
            assert!(Pitch::from_str(string).is_none());
        }
    }

    #[test]
    fn test_pitch_into_u8() {
        let table = vec![
            (Pitch::new(C, 0), 0),
            (Pitch::new(C, 1), 1),
            (Pitch::new(D, 0), 2),
            (Pitch::new(D, 1), 3),
            (Pitch::new(E, 0), 4),
            (Pitch::new(F, 0), 5),
            (Pitch::new(F, 1), 6),
            (Pitch::new(G, 0), 7),
            (Pitch::new(G, 1), 8),
            (Pitch::new(A, 0), 9),
            (Pitch::new(A, 1), 10),
            (Pitch::new(B, 0), 11),
        ];

        for (pitch, number) in table {
            let n = pitch.into_u8();
            assert_eq!(n, number);
        }
    }
}
