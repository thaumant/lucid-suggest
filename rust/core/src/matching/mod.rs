mod damlev;
mod jaccard;
mod word;
mod text;

use std::fmt;
use crate::tokenization::{Word, WordView};
pub use word::word_match;
pub use text::text_match;


#[derive(Clone, PartialEq, Debug)]
pub struct WordMatch {
    pub query:  MatchSide,
    pub record: MatchSide,
    pub typos:  f64,
    pub fin:    bool,
}


impl WordMatch {
    pub fn new(
        qword: &WordView,
        rword: &WordView,
        qlen:  usize,
        rlen:  usize,
        typos: f64
    ) -> Self {
        Self {
            query: MatchSide {
                ix:       qword.ix,
                len:      qword.len(),
                slice:    (0, qlen),
                function: qword.is_function(),
            },
            record: MatchSide {
                ix:       rword.ix,
                len:      rword.len(),
                slice:    (0, rlen),
                function: rword.is_function(),
            },
            typos,
            fin: qword.fin || rword.len() == rlen,
        }
    }

    pub fn split_query(&self, w1: &WordView, w2: &WordView) -> (Self, Self) {
        let (query1, query2) = self.query.split(w1, w2);
        let (typos1, typos2) = Self::split_typos(self.typos, w1.len(), w2.len());
        let part1  = Self {
            query:  query1,
            record: self.record.clone(),
            typos:  typos1,
            fin:    true,
        };
        let part2 = Self {
            query:  query2,
            record: self.record.clone(),
            typos:  typos2,
            fin:    self.fin,
        };
        (part1, part2)
    }

    pub fn split_record(&self, w1: &WordView, w2: &WordView) -> (Self, Self) {
        let (record1, record2) = self.record.split(w1, w2);
        let (typos1, typos2)   = Self::split_typos(self.typos, w1.len(), w2.len());
        let part1  = Self {
            query:  self.query.clone(),
            record: record1,
            typos:  typos1,
            fin:    true,
        };
        let part2 = Self {
            query:  self.query.clone(),
            record: record2,
            typos:  typos2,
            fin:    self.fin,
        };
        (part1, part2)
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


#[derive(Clone, PartialEq, Debug)]
pub struct MatchSide {
    pub ix:      usize,
    pub len:     usize,
    pub slice:   (usize, usize),
    pub function: bool,
}

impl MatchSide {
    pub fn split(&self, w1: &WordView, w2: &WordView) -> (Self, Self) {
        let part1  = Self {
            ix:      w1.ix,
            len:     w1.len(),
            slice:   (0, w1.len()),
            function: w1.is_function(),
        };
        let part2 = Self {
            ix:      w2.ix,
            len:     w2.len(),
            slice:   (0, self.slice.1 - (w2.place.0 - w1.place.0)),
            function: w2.is_function(),
        };
        (part1, part2)
    }
}


impl fmt::Display for WordMatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WordMatch {{ ")?;

        for i in 0 .. self.record.len {
            if i == self.record.slice.0 { write!(f, "[")?; }
            write!(f, "r")?;
            if i + 1 == self.record.slice.1 { write!(f, "]")?; }
        }

        write!(f, " /{}/ ", self.typos)?;

        for i in 0 .. self.query.len {
            if i == self.query.slice.0 { write!(f, "[")?; }
            write!(f, "q")?;
            if i + 1 == self.query.slice.1 { write!(f, "]")?; }
        }
        if !self.fin {
            write!(f, "..")?;
        }

        write!(f, " }}")?;
        Ok(())
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