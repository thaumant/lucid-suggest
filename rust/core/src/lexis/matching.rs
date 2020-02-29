use std::fmt;
use std::collections::HashSet;
use crate::damlev::DamerauLevenshtein;
use super::{Word, Text};


const DAMLEV_THRESHOLD: f64 = 0.21;

thread_local! {
    static DAMLEV: DamerauLevenshtein<char> = DamerauLevenshtein::new();
}


#[derive(Clone, PartialEq)]
pub struct WordMatch {
    pub query:  MatchSide,
    pub record: MatchSide,
    pub typos:  usize,
    pub fin:    bool,
}


#[derive(Clone, PartialEq)]
pub struct MatchSide {
    pub pos:    usize,
    pub len:    usize,
    pub slice:  (usize, usize),
}


impl MatchSide {
    pub fn new(pos: usize, len: usize, slice: (usize, usize)) -> Self {
        MatchSide { pos, len, slice }
    }
}


impl fmt::Debug for WordMatch {
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


pub fn text_match(rtext: &Text<&[char]>, qtext: &Text<&[char]>) -> Vec<WordMatch> {
    let mut matches = Vec::with_capacity(qtext.words.len());
    let mut taken   = HashSet::with_capacity(qtext.words.len());

    for (i, qword) in qtext.words.iter().enumerate() {
        for (j, rword) in rtext.words.iter().enumerate() {
            if taken.contains(&j) { continue; }
            if let Some(mut m) = word_match(rword, qword) {
                m.query.pos  = i;
                m.record.pos = j;
                taken.insert(j);
                matches.push(m);
                break;
            }
        }
    }

    matches
}


pub fn word_match(rword: &Word<&[char]>, qword: &Word<&[char]>) -> Option<WordMatch> {
    if qword.is_empty() { return None; }
    if rword.is_empty() { return None; }

    let mut best_match = None;

    DAMLEV.with(|damlev| {
        damlev.distance(qword.chars.as_ref(), rword.chars.as_ref());
        let dists = &*damlev.dists.borrow();
        let qlen  = qword.len();

        for &rlen in &[qlen + 1, qlen, qlen - 1] {
            if rlen > rword.len() { continue; }
            if rlen < rword.len() && qword.fin { break; }
            if rlen == 0 { break; }

            let dist     = dists.get(qlen + 1, rlen + 1);
            let rel_dist = dist as f64 / max!(qlen, rlen, 1) as f64;
            if rel_dist > DAMLEV_THRESHOLD { continue; }

            match best_match {
                None => {
                    best_match = Some(WordMatch {
                        query:  MatchSide::new(0, qlen,        (0, qlen)),
                        record: MatchSide::new(0, rword.len(), (0, rlen)),
                        typos:  dist,
                        fin:    qword.fin || rword.len() == rlen,
                    });
                },
                Some(ref mut m) => {
                    if m.typos <= dist { continue; }
                    m.record.slice = (0, rlen);
                    m.typos        = dist;
                    m.fin          = qword.fin || rword.len() == rlen;
                },
            };
        }
    });

    best_match
}


#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use crate::lexis::Chars;
    use super::{Word, Text, word_match, text_match};


    fn text(s: &str) -> Text<Vec<char>> {
        Text::from_str(s).split(&Chars::Whitespaces)
    }


    // Match word
    // ----------------------------------------------------------------

    #[test]
    fn match_word_empty_both() {
        let q = Word::from_str("").fin(false);
        let r = Word::from_str("");
        assert_eq!(word_match(&r.to_ref(), &q.to_ref()), None);
    }


    #[test]
    fn match_word_empty_record() {
        let q = Word::from_str("mailbox").fin(false);
        let r = Word::from_str("");
        assert_eq!(word_match(&r.to_ref(), &q.to_ref()), None);
    }


    #[test]
    fn match_word_empty_query() {
        let q = Word::from_str("").fin(false);
        let r = Word::from_str("mailbox");
        assert_eq!(word_match(&r.to_ref(), &q.to_ref()), None);
    }


    // Match word: prefixed by same length word
    // ----------------------------------------------------------------

    #[test]
    fn match_word_full_strict() {
        let q = Word::from_str("mailbox").fin(false);
        let r = Word::from_str("mailbox");
        assert_debug_snapshot!(word_match(&r.to_ref(), &q.to_ref()));
    }


    #[test]
    fn match_word_full_fuzzy_insertion() {
        let q = Word::from_str("mailybox").fin(false);
        let r = Word::from_str("mailbox");
        assert_debug_snapshot!(word_match(&r.to_ref(), &q.to_ref()));
    }


    #[test]
    fn match_word_full_fuzzy_deletion() {
        let q = Word::from_str("mailox").fin(false);
        let r = Word::from_str("mailbox");
        assert_debug_snapshot!(word_match(&r.to_ref(), &q.to_ref()));
    }


    #[test]
    fn match_word_full_fuzzy_transposition() {
        let q = Word::from_str("maiblox").fin(false);
        let r = Word::from_str("mailbox");
        assert_debug_snapshot!(word_match(&r.to_ref(), &q.to_ref()));
    }


    // Match word: prefixed by lesser length word
    // ----------------------------------------------------------------

    #[test]
    fn match_word_partial_strict() {
        let q = Word::from_str("mailb").fin(false);
        let r = Word::from_str("mailbox");
        assert_debug_snapshot!(word_match(&r.to_ref(), &q.to_ref()));
    }


    #[test]
    fn match_word_partial_fuzzy_insertion() {
        let q = Word::from_str("maiylb").fin(false);
        let r = Word::from_str("mailbox");
        assert_debug_snapshot!(word_match(&r.to_ref(), &q.to_ref()));
    }


    #[test]
    fn match_word_partial_fuzzy_deletion() {
        let q = Word::from_str("maib").fin(false);
        let r = Word::from_str("mailbox");
        assert_debug_snapshot!(word_match(&r.to_ref(), &q.to_ref()));
    }


    #[test]
    fn match_word_partial_fuzzy_transposition() {
        let q = Word::from_str("malib").fin(false);
        let r = Word::from_str("mailbox");
        assert_debug_snapshot!(word_match(&r.to_ref(), &q.to_ref()));
    }


    // Match text
    // ----------------------------------------------------------------

    #[test]
    fn match_text_empty_both() {
        let q = text("").fin(false);
        let r = text("");
        assert_debug_snapshot!(text_match(&r.to_ref(), &q.to_ref()));
    }


    #[test]
    fn match_text_empty_one() {
        let q = text("mailbox").fin(false);
        let r = text("");
        assert_debug_snapshot!(text_match(&r.to_ref(), &q.to_ref()));
        assert_debug_snapshot!(text_match(&q.to_ref(), &r.to_ref()));
    }


    #[test]
    fn match_text_singleton_equality() {
        let q = text("mailbox").fin(false);
        let r = text("mailbox");
        assert_debug_snapshot!(text_match(&r.to_ref(), &q.to_ref()));
    }


    #[test]
    fn match_text_singleton_typos() {
        let q = text("maiblox").fin(false);
        let r = text("mailbox");
        assert_debug_snapshot!(text_match(&r.to_ref(), &q.to_ref()));
    }


    #[test]
    fn match_text_pair_first() {
        let q = text("yelow").fin(false);
        let r = text("yellow mailbox");
        assert_debug_snapshot!(text_match(&r.to_ref(), &q.to_ref()));
    }


    #[test]
    fn match_text_pair_second() {
        let q = text("maiblox").fin(false);
        let r = text("yellow mailbox");
        assert_debug_snapshot!(text_match(&r.to_ref(), &q.to_ref()));
    }


    #[test]
    fn match_text_pair_unfinished() {
        let q = text("maiblox yel").fin(false);
        let r = text("yellow mailbox");
        assert_debug_snapshot!(text_match(&r.to_ref(), &q.to_ref()));
    }


    #[test]
    fn match_text_intersection() {
        let q = text("big malibox yelo").fin(false);
        let r = text("small yellow metal mailbox");
        assert_debug_snapshot!(text_match(&r.to_ref(), &q.to_ref()));
    }


    #[test]
    fn match_text_regression_best_match() {
        let q = text("sneak").fin(false);
        let r = text("sneaky");
        assert_debug_snapshot!(text_match(&r.to_ref(), &q.to_ref()));
    }


}