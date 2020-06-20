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
    pub ix:      usize,
    pub len:     usize,
    pub slice:   (usize, usize),
    pub primary: bool,
}


impl MatchSide {
    pub fn new(len: usize, slice: (usize, usize), primary: bool) -> Self {
        MatchSide { ix: 0, len, slice, primary }
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
    let capacity = min!(rtext.words.len(), qtext.words.len());
    let mut taken: HashSet<usize> = HashSet::with_capacity(capacity);
    let mut matches: Vec<WordMatch> = Vec::with_capacity(capacity);

    for (i, qword) in qtext.words.iter().enumerate() {
        let mut found: Option<WordMatch> = None;

        for (j, rword) in rtext.words.iter().enumerate() {
            if taken.contains(&j) { continue; }
            if let Some(mut m) = word_match(rword, qword, &rtext.chars, &qtext.chars) {
                m.query.ix  = i;
                m.record.ix = j;
                if found.is_none() && !m.record.primary {
                    found = Some(m);
                    continue;
                }
                if m.record.primary {
                    found = Some(m);
                    break;
                }
            }
        }

        if let Some(m) = found {
            taken.insert(m.record.ix);
            matches.push(m);
        }
    }

    matches
}


pub fn word_match(rword: &Word, qword: &Word, rchars: &[char], qchars: &[char]) -> Option<WordMatch> {
    if qword.is_empty() || rword.is_empty() {
        return None;
    }
    if !length_check(rword, qword) {
        return None;
    }
    if !jaccard_check(rword, qword, rchars, qchars) {
        return None;
    }

    let rchars = rword.view(rchars);
    let qchars = qword.view(qchars);
    let mut best_match = None;

    DAMLEV.with(|damlev| {
        damlev.distance(qchars, rchars);
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
                            query:  MatchSide::new(qlen,        (0, qlen), qword.is_primary()),
                            record: MatchSide::new(rword.len(), (0, rlen), rword.is_primary()),
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


pub fn length_check(rword: &Word, qword: &Word) -> bool {
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


pub fn jaccard_check(rword: &Word, qword: &Word, rchars: &[char], qchars: &[char]) -> bool {
    let rchars = rword.view(rchars);
    let qchars = qword.view(qchars);
    let rslice = if qword.fin { &rchars } else { &rchars[.. min!(qword.len(), rword.len())] };
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
        let qchars = "".chars().collect::<Vec<_>>();
        let rchars = "".chars().collect::<Vec<_>>();
        let qword  = Word::new(qchars.len()).fin(false);
        let rword  = Word::new(rchars.len());
        assert_eq!(word_match(&rword, &qword, &rchars[..], &qchars[..]), None);
    }


    #[test]
    fn match_word_empty_record() {
        let qchars = "mailbox".chars().collect::<Vec<_>>();
        let rchars = "".chars().collect::<Vec<_>>();
        let qword  = Word::new(qchars.len()).fin(false);
        let rword  = Word::new(rchars.len());
        assert_eq!(word_match(&rword, &qword, &rchars[..], &qchars[..]), None);
    }


    #[test]
    fn match_word_empty_query() {
        let qchars = "".chars().collect::<Vec<_>>();
        let rchars = "mailbox".chars().collect::<Vec<_>>();
        let qword  = Word::new(qchars.len()).fin(false);
        let rword  = Word::new(rchars.len());
        assert_eq!(word_match(&rword, &qword, &rchars[..], &qchars[..]), None);
    }


    // Match word: prefixed by same length word
    // ----------------------------------------------------------------

    #[test]
    fn match_word_full_strict() {
        let qchars = "mailbox".chars().collect::<Vec<_>>();
        let rchars = "mailbox".chars().collect::<Vec<_>>();
        let qword  = Word::new(qchars.len()).fin(false);
        let rword  = Word::new(rchars.len());
        assert_debug_snapshot!(word_match(&rword, &qword, &rchars[..], &qchars[..]));
    }


    #[test]
    fn match_word_full_fuzzy_insertion() {
        let qchars = "mailybox".chars().collect::<Vec<_>>();
        let rchars = "mailbox".chars().collect::<Vec<_>>();
        let qword  = Word::new(qchars.len()).fin(false);
        let rword  = Word::new(rchars.len());
        assert_debug_snapshot!(word_match(&rword, &qword, &rchars[..], &qchars[..]));
    }


    #[test]
    fn match_word_full_fuzzy_deletion() {
        let qchars = "mailox".chars().collect::<Vec<_>>();
        let rchars = "mailbox".chars().collect::<Vec<_>>();
        let qword  = Word::new(qchars.len()).fin(false);
        let rword  = Word::new(rchars.len());
        assert_debug_snapshot!(word_match(&rword, &qword, &rchars[..], &qchars[..]));
    }


    #[test]
    fn match_word_full_fuzzy_transposition() {
        let qchars = "maiblox".chars().collect::<Vec<_>>();
        let rchars = "mailbox".chars().collect::<Vec<_>>();
        let qword  = Word::new(qchars.len()).fin(false);
        let rword  = Word::new(rchars.len());
        assert_debug_snapshot!(word_match(&rword, &qword, &rchars[..], &qchars[..]));
    }


    #[test]
    fn match_word_full_query_too_long() {
        let qchars1 = "mailboxes".chars().collect::<Vec<_>>();
        let qchars2 = "mailboxes".chars().collect::<Vec<_>>();
        let rchars  = "mail"     .chars().collect::<Vec<_>>();
        let qword1  = Word::new(qchars1.len()).fin(true);
        let qword2  = Word::new(qchars2.len()).fin(false);
        let rword   = Word::new(rchars.len());
        assert_debug_snapshot!(word_match(&rword, &qword1, &rchars[..], &qchars1[..]));
        assert_debug_snapshot!(word_match(&rword, &qword2, &rchars[..], &qchars2[..]));
    }

    #[test]
    fn match_word_full_stem() {
        let     rchars  = "universe"  .chars().collect::<Vec<_>>();
        let     qchars1 = "university".chars().collect::<Vec<_>>();
        let     qchars2 = "university".chars().collect::<Vec<_>>();
        let mut rword   = Word::new(rchars.len());
        let mut qword1  = Word::new(qchars1.len());
        let     qword2  = Word::new(qchars2.len());

        let lang = lang_english();
        qword1.stem(&qchars1[..], &lang);
        rword .stem(&rchars[..],  &lang);

        assert_debug_snapshot!(word_match(&rword, &qword1, &rchars[..], &qchars1[..]));
        assert_debug_snapshot!(word_match(&rword, &qword2, &rchars[..], &qchars2[..]));
    }


    // Match word: prefixed by lesser length word
    // ----------------------------------------------------------------

    #[test]
    fn match_word_partial_strict() {
        let qchars = "mailb".chars().collect::<Vec<_>>();
        let rchars = "mailbox".chars().collect::<Vec<_>>();
        let qword = Word::new(qchars.len()).fin(false);
        let rword = Word::new(rchars.len());
        assert_debug_snapshot!(word_match(&rword, &qword, &rchars[..], &qchars[..]));
    }


    #[test]
    fn match_word_partial_fuzzy_insertion() {
        let qchars = "maiylb".chars().collect::<Vec<_>>();
        let rchars = "mailbox".chars().collect::<Vec<_>>();
        let qword  = Word::new(qchars.len()).fin(false);
        let rword  = Word::new(rchars.len());
        assert_debug_snapshot!(word_match(&rword, &qword, &rchars[..], &qchars[..]));
    }


    #[test]
    fn match_word_partial_fuzzy_deletion() {
        let qchars = "maib".chars().collect::<Vec<_>>();
        let rchars = "mailbox".chars().collect::<Vec<_>>();
        let qword  = Word::new(qchars.len()).fin(false);
        let rword  = Word::new(rchars.len());
        assert_debug_snapshot!(word_match(&rword, &qword, &rchars[..], &qchars[..]));
    }


    #[test]
    fn match_word_partial_fuzzy_transposition() {
        let qchars = "malib".chars().collect::<Vec<_>>();
        let rchars = "mailbox".chars().collect::<Vec<_>>();
        let qword  = Word::new(qchars.len()).fin(false);
        let rword  = Word::new(rchars.len());
        assert_debug_snapshot!(word_match(&rword, &qword, &rchars[..], &qchars[..]));
    }


    // Match text
    // ----------------------------------------------------------------

    #[test]
    fn match_text_empty_both() {
        let qtext = text("").fin(false);
        let rtext = text("");
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }


    #[test]
    fn match_text_empty_one() {
        let qtext = text("mailbox").fin(false);
        let rtext = text("");
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
        assert_debug_snapshot!(text_match(&qtext.to_ref(), &rtext.to_ref()));
    }


    #[test]
    fn match_text_singleton_equality() {
        let qtext = text("mailbox").fin(false);
        let rtext = text("mailbox");
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }


    #[test]
    fn match_text_singleton_typos() {
        let qtext = text("maiblox").fin(false);
        let rtext = text("mailbox");
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }


    #[test]
    fn match_text_pair_first() {
        let qtext = text("yelow").fin(false);
        let rtext = text("yellow mailbox");
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }


    #[test]
    fn match_text_pair_second() {
        let qtext = text("maiblox").fin(false);
        let rtext = text("yellow mailbox");
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }


    #[test]
    fn match_text_pair_unfinished() {
        let qtext = text("maiblox yel").fin(false);
        let rtext = text("yellow mailbox");
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }


    #[test]
    fn match_text_intersection() {
        let qtext = text("big malibox yelo").fin(false);
        let rtext = text("small yellow metal mailbox");
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }


    #[test]
    fn match_text_best_rword() {
        let mut qtext = text("the").fin(false);
        let mut rtext = text("the theme");
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));

        let lang = lang_english();
        qtext = qtext.mark_pos(&lang);
        rtext = rtext.mark_pos(&lang);
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }


    #[test]
    fn match_text_regression_best_match() {
        let qtext = text("sneak").fin(false);
        let rtext = text("sneaky");
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }
}