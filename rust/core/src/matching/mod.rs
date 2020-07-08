mod damlev;
mod jaccard;
mod word;
mod text;

use crate::tokenization::{Word, WordView};
pub use word::word_match;
pub use text::text_match;


#[derive(Clone, PartialEq, Debug)]
pub struct WordMatch {
    pub offset:   usize,
    pub slice:    (usize, usize),
    pub subslice: (usize, usize),
    pub typos:    f64,
    pub func:     bool,
    pub fin:      bool,
}


impl WordMatch {
    pub fn new_pair(
        rword:  &WordView,
        qword:  &WordView,
        rslice: usize,
        qslice: usize,
        typos: f64
    ) -> (Self, Self) {
        debug_assert!(rword.slice.0 + rslice <= rword.slice.1, "Record match subslice is too long");
        debug_assert!(qword.slice.0 + qslice <= qword.slice.1, "Query match subslice is too long");
        let fin = qword.fin || rword.len() == rslice;
        let rmatch = WordMatch {
            offset:   rword.offset,
            slice:    rword.slice,
            subslice: (0, rslice),
            func:     rword.is_function(),
            typos,
            fin,
        };
        let qmatch = WordMatch {
            offset:   qword.offset,
            slice:    qword.slice,
            subslice: (0, qslice),
            func:     qword.is_function(),
            typos,
            fin,
        };
        (rmatch, qmatch)
    }

    pub fn word_len(&self) -> usize {
        let (left, right) = self.slice;
        return right - left;
    }

    pub fn match_len(&self) -> usize {
        let (left, right) = self.subslice;
        return right - left;
    }

    pub fn split(&self, w1: &WordView, w2: &WordView) -> Option<(Self, Self)> {
        debug_assert!(w2.slice.0 > w1.slice.0,              "Invalid word order in match split");
        debug_assert!(w1.offset == self.offset || w2.offset == self.offset, "Invalid word offsets in match split");
        if w1.slice.0 + self.subslice.1 <= w2.slice.0 {
            return None;
        }
        let (typos1, typos2) = Self::split_typos(self.typos, w1.len(), w2.len());
        let part1 = Self {
            offset:   w1.offset,
            slice:    w1.slice,
            subslice: (0, w1.len()),
            func:     w1.is_function(),
            typos:    typos1,
            fin:      true,
        };
        let part2 = Self {
            offset:   w2.offset,
            slice:    w2.slice,
            subslice: (0, self.subslice.1 - (w2.slice.0 - w1.slice.0)),
            func:     w2.is_function(),
            typos:    typos2,
            fin:      self.fin,
        };
        Some((part1, part2))
    }

    fn split_typos(typos: f64, len1: usize, len2: usize) -> (f64, f64) {
        if len1 == 0 { return (0.0, typos); }
        if len2 == 0 { return (typos, 0.0); }
        let len1   = len1 as f64;
        let len2   = len2 as f64;
        let split1 = (typos * len1 * 10.0 / (len1 + len2)).ceil() / 10.0;
        let split2 = ((typos - split1) * 10.0).round() / 10.0;
        (split1, split2)
    }
}


#[cfg(test)]
mod tests {
    use super::{WordMatch};

    fn test_split_typos(
        len1:   usize,
        len2:   usize,
        sample: &[(f64, f64, f64)],
    ) {
        for &(typos, expected1, expected2) in sample {
            let (received1, received2) = WordMatch::split_typos(typos, len1, len2);
            assert_eq!(
                (received1, received2),
                (expected1, expected2),
                "Expected {} typos for lengths {}/{} to split {}/{}, got {}/{}",
                typos, len1, len2, expected1, expected2, received1, received2
            );
        }
    }

    #[test]
    fn split_typos_in_half() {
        test_split_typos(5, 5, &[
            (0.0, 0.0, 0.0),
            (1.0, 0.5, 0.5),
            (2.0, 1.0, 1.0),
            (3.0, 1.5, 1.5),
            (4.0, 2.0, 2.0),
            (5.0, 2.5, 2.5),
        ]);
    }

    #[test]
    fn split_typos_1_to_0() {
        test_split_typos(5, 0, &[
            (1.0, 1.0, 0.0),
            (2.0, 2.0, 0.0),
            (3.0, 3.0, 0.0),
            (4.0, 4.0, 0.0),
            (5.0, 5.0, 0.0),
        ]);
    }

    #[test]
    fn split_typos_0_to_1() {
        test_split_typos(0, 5, &[
            (1.0, 0.0, 1.0),
            (2.0, 0.0, 2.0),
            (3.0, 0.0, 3.0),
            (4.0, 0.0, 4.0),
            (5.0, 0.0, 5.0),
        ]);
    }

    #[test]
    fn split_typos_2_to_1() {
        test_split_typos(10, 5, &[
            (0.0, 0.0, 0.0),
            (1.0, 0.7, 0.3),
            (2.0, 1.4, 0.6),
            (3.0, 2.0, 1.0),
            (4.0, 2.7, 1.3),
            (5.0, 3.4, 1.6),
        ]);
    }

    #[test]
    fn split_typos_1_to_2() {
        test_split_typos(5, 10, &[
            (0.0, 0.0, 0.0),
            (1.0, 0.4, 0.6),
            (2.0, 0.7, 1.3),
            (3.0, 1.0, 2.0),
            (4.0, 1.4, 2.6),
            (5.0, 1.7, 3.3),
        ]);
    }

    #[test]
    fn split_typos_3_to_2() {
        test_split_typos(9, 6, &[
            (0.0, 0.0, 0.0),
            (1.0, 0.6, 0.4),
            (2.0, 1.2, 0.8),
            (3.0, 1.8, 1.2),
            (4.0, 2.4, 1.6),
            (5.0, 3.0, 2.0),
        ]);
    }

    #[test]
    fn split_typos_2_to_3() {
        test_split_typos(6, 9, &[
            (0.0, 0.0, 0.0),
            (1.0, 0.4, 0.6),
            (2.0, 0.8, 1.2),
            (3.0, 1.2, 1.8),
            (4.0, 1.6, 2.4),
            (5.0, 2.0, 3.0),
        ]);
    }
}