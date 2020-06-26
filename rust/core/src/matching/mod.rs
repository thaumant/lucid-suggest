mod damlev;
mod jaccard;
mod word;
mod text;

use std::fmt;
use crate::tokenization::{Word};
pub use word::word_match;
pub use text::text_match;


#[derive(Clone, PartialEq, Debug)]
pub struct WordMatch {
    pub query:  MatchSide,
    pub record: MatchSide,
    pub typos:  usize,
    pub fin:    bool,
}


impl WordMatch {
    pub fn new(
        qword: &Word,
        rword: &Word,
        qlen:  usize,
        rlen:  usize,
        typos: usize
    ) -> Self {
        Self {
            query: MatchSide {
                ix:      qword.ix,
                len:     qword.len(),
                slice:   (0, qlen),
                primary: qword.is_primary(),
            },
            record: MatchSide {
                ix:      rword.ix,
                len:     rword.len(),
                slice:   (0, rlen),
                primary: rword.is_primary(),
            },
            typos,
            fin: qword.fin || rword.len() == rlen,
        }
    }

    pub fn split_query(&self, w1: &Word, w2: &Word) -> (Self, Self) {
        let (query1, query2) = self.query.split(w1, w2);
        let typos2 = self.typos / 2;
        let typos1 = self.typos - typos2;
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

    pub fn split_record(&self, w1: &Word, w2: &Word) -> (Self, Self) {
        let (record1, record2) = self.record.split(w1, w2);
        let typos2 = self.typos / 2;
        let typos1 = self.typos - typos2;
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
}


#[derive(Clone, PartialEq, Debug)]
pub struct MatchSide {
    pub ix:      usize,
    pub len:     usize,
    pub slice:   (usize, usize),
    pub primary: bool,
}

impl MatchSide {
    pub fn split(&self, w1: &Word, w2: &Word) -> (Self, Self) {
        let part1  = Self {
            ix:      w1.ix,
            len:     w1.len(),
            slice:   (0, w1.len()),
            primary: w1.is_primary(),
        };
        let part2 = Self {
            ix:      w2.ix,
            len:     w2.len(),
            slice:   (0, self.slice.1 - (w2.place.0 - w1.place.0)),
            primary: w2.is_primary(),
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
