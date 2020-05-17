use std::fmt;
use std::collections::HashSet;
use crate::damlev::DamerauLevenshtein;
use crate::jaccard::Jaccard;
use super::{Word, Text};


const LENGTH_THRESHOLD:  f64 = 0.51;
const JACCARD_THRESHOLD: f64 = 0.41;
const DAMLEV_THRESHOLD:  f64 = 0.21;

thread_local! {
    static DAMLEV:  DamerauLevenshtein<char> = DamerauLevenshtein::new();
    static JACCARD: Jaccard<char>            = Jaccard::new();
}


#[derive(Clone, PartialEq)]
pub struct WordMatch {
    pub query:  MatchSide,
    pub record: MatchSide,
    pub typos:  usize,
    pub fin:    bool,
}


#[derive(Clone, PartialEq, Debug)]
pub struct MatchSide {
    pub pos:     usize,
    pub len:     usize,
    pub slice:   (usize, usize),
    pub primary: bool,
}


impl MatchSide {
    pub fn new(pos: usize, len: usize, slice: (usize, usize), primary: bool) -> Self {
        MatchSide { pos, len, slice, primary }
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
    if qword.is_empty() || rword.is_empty() {
        return None;
    }
    if !length_check(qword, rword) {
        return None;
    }
    if !jaccard_check(qword, rword) {
        return None;
    }

    let mut best_match = None;

    DAMLEV.with(|damlev| {
        damlev.distance(qword.chars.as_ref(), rword.chars.as_ref());
        let dists  = &*damlev.dists.borrow();

        let left  = if qword.fin { max!(qword.stem, rword.stem) } else { qword.stem } - 1;
        let right = max!(qword.len(), rword.len()) + 1;

        if right <= left { return; }

        let range = (left .. right).rev();

        for rlen in range.clone() {
            for qlen in range.clone() {
                // Out of bounds.
                if qlen > qword.len() { continue; }
                if rlen > rword.len() { continue; }
                // Left margin is for insertion/deletion, not for both prefixes at the same time.
                if rlen == left  && qlen == left  { continue; }
                // Compare full words only if query is finished.
                if qword.fin && rlen < rword.stem { break; }
                if qword.fin && qlen < qword.stem { break; }
                // Words with 2+ insertions/deletions are mismatched by default.
                if (qlen as isize - rlen as isize).abs() > 1 { continue; }

                let dist = dists.get(qlen + 1, rlen + 1);
                let rel  = dist as f64 / max!(qlen, rlen, 1) as f64;

                if rel > DAMLEV_THRESHOLD { continue; }

                match best_match {
                    None => {
                        best_match = Some(WordMatch {
                            query:  MatchSide::new(0, qlen,        (0, qlen), qword.is_primary()),
                            record: MatchSide::new(0, rword.len(), (0, rlen), rword.is_primary()),
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
                }

                if dist == 0 {
                    break;
                }
            }
        }
    });

    best_match
}


pub fn length_check(qword: &Word<&[char]>, rword: &Word<&[char]>) -> bool {
    let qlen  = qword.len();
    let rlen  = if qword.fin { rword.len() } else { min!(qlen, rword.len()) };

    if qlen <= 3 || rlen <= 3 {
        return qlen == rlen;
    }

    let long  = max!(qlen, rlen);
    let short = min!(qlen, rlen);
    let dist  = 1.0 - (short as f64 / long as f64);

    dist < LENGTH_THRESHOLD
}


pub fn jaccard_check(qword: &Word<&[char]>, rword: &Word<&[char]>) -> bool {
    let Word { chars: qchars, fin, .. } = qword;
    let Word { chars: rchars, .. }      = rword;
    let rslice = if *fin { &rchars } else { &rchars[.. min!(qword.len(), rword.len())] };
    let dist   = JACCARD.with(|j| j.rel_dist(&rslice, &qchars));
    dist < JACCARD_THRESHOLD
}


#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use crate::lexis::Chars;
    use crate::lang::lang_english;
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


    #[test]
    fn match_word_full_query_too_long() {
        let q1 = Word::from_str("mailboxes").fin(true);
        let q2 = Word::from_str("mailboxes").fin(false);
        let r  = Word::from_str("mail");
        assert_debug_snapshot!(word_match(&r.to_ref(), &q1.to_ref()));
        assert_debug_snapshot!(word_match(&r.to_ref(), &q2.to_ref()));
    }

    #[test]
    fn match_word_full_stem() {
        let mut r  = Word::from_str("universe");
        let mut q1 = Word::from_str("university");
        let     q2 = Word::from_str("university");

        let lang = lang_english();
        q1.stem(&lang);
        r.stem(&lang);

        assert_debug_snapshot!(word_match(&r.to_ref(), &q1.to_ref()));
        assert_debug_snapshot!(word_match(&r.to_ref(), &q2.to_ref()));
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