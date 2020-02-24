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


impl<T: AsRef<[char]>> Text<T> {
    pub fn matches<U: AsRef<[char]>>(&self, query: &Text<U>) -> Vec<WordMatch> {
        let mut matches = Vec::with_capacity(query.words.len());
        let mut taken   = HashSet::with_capacity(query.words.len());

        for (i, qword) in query.words.iter().enumerate() {
            for (j, rword) in self.words.iter().enumerate() {
                if taken.contains(&j) { continue; }
                if let Some(mut m) = rword.matches(qword) {
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
}


impl<T: AsRef<[char]>> Word<T> {
    pub fn matches<U: AsRef<[char]>>(&self, qword: &Word<U>) -> Option<WordMatch> {
        if qword.is_empty() { return None; }
        if self.is_empty() { return None; }
    
        let mut result = None;
    
        for &len in &[qword.len() + 1, qword.len(), qword.len() - 1] {
            if len > self.len() { continue; }
            if len < self.len() && qword.fin { break; }
            if len == 0 { break; }
            let rslice   = &self.chars.as_ref()[0..len];
            let dist     = DAMLEV.with(|dl| dl.distance(qword.chars.as_ref(), &rslice));
            let rel_dist = dist as f64 / max!(qword.len(), len, 1) as f64;
            if rel_dist > DAMLEV_THRESHOLD { continue; }
            match result {
                None => {
                    result = Some(WordMatch {
                        query: MatchSide {
                            pos:   0,
                            len:   qword.len(),
                            slice: (0, qword.len()),
                        },
                        record: MatchSide {
                            pos:   0,
                            len:   self.len(),
                            slice: (0, len),
                        },
                        typos: dist,
                        fin:   qword.fin || self.len() == len,
                    });
                },
                Some(ref mut m) => {
                    if m.typos <= dist { continue; }
                    m.record.slice = (0, len);
                    m.typos        = dist;
                    m.fin          = qword.fin || self.len() == len;
                },
            };
        }
    
        result
    }
}


#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use crate::lexis::Chars;
    use super::{Word, Text};
    use std::borrow::Cow;


    fn chars(s: &str) -> Vec<char> {
        s.chars().collect()
    }
    
    fn record(chars: &[char]) -> Text<Cow<[char]>> {
        Text::new_cow(Cow::Borrowed(chars)).split(&Chars::Whitespaces)
    }

    fn query(s: &[char]) -> Text<Cow<[char]>> {
        record(s).fin(false)
    }


    // Match word
    // ----------------------------------------------------------------

    #[test]
    fn match_word_empty_both() {
        let q = Word::new_owned(chars("")).fin(false);
        let r = Word::new_owned(chars(""));
        assert_eq!(r.matches(&q), None);
    }


    #[test]
    fn match_word_empty_record() {
        let q  = Word::new_owned(chars("mailbox")).fin(false);
        let r  = Word::new_owned(chars(""));
        assert_eq!(r.matches(&q), None);
    }


    #[test]
    fn match_word_empty_query() {
        let q  = Word::new_owned(chars("")).fin(false);
        let r  = Word::new_owned(chars("mailbox"));
        assert_eq!(r.matches(&q), None);
    }


    // Match word: prefixed by same length word
    // ----------------------------------------------------------------

    #[test]
    fn match_word_full_strict() {
        let q  = Word::new_owned(chars("mailbox")).fin(false);
        let r  = Word::new_owned(chars("mailbox"));
        assert_debug_snapshot!(r.matches(&q));
    }


    #[test]
    fn match_word_full_fuzzy_insertion() {
        let q  = Word::new_owned(chars("mailybox")).fin(false);
        let r  = Word::new_owned(chars("mailbox"));
        assert_debug_snapshot!(r.matches(&q));
    }


    #[test]
    fn match_word_full_fuzzy_deletion() {
        let q  = Word::new_owned(chars("mailox")).fin(false);
        let r  = Word::new_owned(chars("mailbox"));
        assert_debug_snapshot!(r.matches(&q));
    }


    #[test]
    fn match_word_full_fuzzy_transposition() {
        let q  = Word::new_owned(chars("maiblox")).fin(false);
        let r  = Word::new_owned(chars("mailbox"));
        assert_debug_snapshot!(r.matches(&q));
    }


    // Match word: prefixed by lesser length word
    // ----------------------------------------------------------------

    #[test]
    fn match_word_partial_strict() {
        let q  = Word::new_owned(chars("mailb")).fin(false);
        let r  = Word::new_owned(chars("mailbox"));
        assert_debug_snapshot!(r.matches(&q));
    }


    #[test]
    fn match_word_partial_fuzzy_insertion() {
        let q  = Word::new_owned(chars("maiylb")).fin(false);
        let r  = Word::new_owned(chars("mailbox"));
        assert_debug_snapshot!(r.matches(&q));
    }


    #[test]
    fn match_word_partial_fuzzy_deletion() {
        let q  = Word::new_owned(chars("maib")).fin(false);
        let r  = Word::new_owned(chars("mailbox"));
        assert_debug_snapshot!(r.matches(&q));
    }


    #[test]
    fn match_word_partial_fuzzy_transposition() {
        let q  = Word::new_owned(chars("malib")).fin(false);
        let r  = Word::new_owned(chars("mailbox"));
        assert_debug_snapshot!(r.matches(&q));
    }


    // Match text
    // ----------------------------------------------------------------

    #[test]
    fn match_text_empty_both() {
        let c1 = chars("");
        let c2 = chars("");
        let q  = query(&c1);
        let r  = record(&c2);
        assert_debug_snapshot!(r.matches(&q));
    }


    #[test]
    fn match_text_empty_one() {
        let c1 = chars("mailbox");
        let c2 = chars("");
        let q  = query(&c1);
        let r  = record(&c2);
        assert_debug_snapshot!(r.matches(&q));
        assert_debug_snapshot!(q.matches(&r));
    }


    #[test]
    fn match_text_singleton_equality() {
        let c1 = chars("mailbox");
        let c2 = chars("mailbox");
        let q  = query(&c1);
        let r  = record(&c2);
        assert_debug_snapshot!(r.matches(&q));
    }


    #[test]
    fn match_text_singleton_typos() {
        let c1 = chars("maiblox");
        let c2 = chars("mailbox");
        let q  = query(&c1);
        let r  = record(&c2);
        assert_debug_snapshot!(r.matches(&q));
    }


    #[test]
    fn match_text_pair_first() {
        let c1 = chars("yelow");
        let c2 = chars("yellow mailbox");
        let q  = query(&c1);
        let r  = record(&c2);
        assert_debug_snapshot!(r.matches(&q));
    }


    #[test]
    fn match_text_pair_second() {
        let c1 = chars("maiblox");
        let c2 = chars("yellow mailbox");
        let q  = query(&c1);
        let r  = record(&c2);
        assert_debug_snapshot!(r.matches(&q));
    }


    #[test]
    fn match_text_pair_unfinished() {
        let c1 = chars("maiblox yel");
        let c2 = chars("yellow mailbox");
        let q  = query(&c1);
        let r  = record(&c2);
        assert_debug_snapshot!(r.matches(&q));
    }


    #[test]
    fn match_text_intersection() {
        let c1 = chars("big malibox yelo");
        let c2 = chars("small yellow metal mailbox");
        let q  = query(&c1);
        let r  = record(&c2);
        assert_debug_snapshot!(r.matches(&q));
    }


    #[test]
    fn match_text_regression_best_match() {
        let c1 = chars("sneak");
        let c2 = chars("sneaky");
        let q  = query(&c1);
        let r  = record(&c2);
        assert_debug_snapshot!(r.matches(&q));
    }
}