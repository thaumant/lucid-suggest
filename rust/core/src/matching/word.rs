use crate::tokenization::{Word, WordView};
use super::WordMatch;
use super::damlev::DamerauLevenshtein;
use super::jaccard::Jaccard;


const LENGTH_THRESHOLD:  f64 = 0.26;
const JACCARD_THRESHOLD: f64 = 0.51;
const DAMLEV_THRESHOLD:  f64 = 0.21;

thread_local! {
    static DAMLEV:  DamerauLevenshtein = DamerauLevenshtein::new();
    static JACCARD: Jaccard<char>      = Jaccard::new();
}


pub fn word_match(rword: &WordView, qword: &WordView) -> Option<(WordMatch, WordMatch)> {
    if qword.is_empty() || rword.is_empty() {
        return None;
    }
    if !length_check(rword, qword) {
        return None;
    }
    if !jaccard_check(rword, qword) {
        return None;
    }

    let mut best_match: Option<(WordMatch, WordMatch)> = None;

    DAMLEV.with(|damlev| {
        damlev.distance(qword, rword);
        let dists = &*damlev.dists.borrow();

        let left  = if qword.fin { max!(qword.stem, rword.stem) } else { qword.stem } - 1;
        let right = max!(qword.len(), rword.len()) + 1;

        if right <= left { return; }

        let range = (left .. right).rev();

        for rlen in range.clone() {
            for qlen in range.clone() {
                // Out of bounds.
                if qlen > qword.len() { continue; }
                if rlen > rword.len() { continue; }
                if qlen < qword.stem  { continue; }
                // Left margin is for insertion/deletion, not for both prefixes at the same time.
                if rlen == left && qlen == left  { continue; }
                // Compare full words only if query is finished.
                if qword.fin && rlen < rword.stem { break; }
                // Words with 2+ insertions/deletions are mismatched by default.
                if (qlen as isize - rlen as isize).abs() > 1 { continue; }

                let dist = dists.get(qlen + 1, rlen + 1);

                let rel = dist / max!(qlen, rlen, 1) as f64;
                if rel > DAMLEV_THRESHOLD { continue; }

                best_match = best_match
                    .take()
                    .filter(|m| m.0.typos <= dist)
                    .or_else(|| Some(WordMatch::new_pair(
                        rword,
                        qword,
                        rlen,
                        qlen,
                        dist,
                    )));

                if dist <= std::f64::EPSILON {
                    break;
                }
            }
        }
    });

    best_match
}


pub fn length_check(rword: &WordView, qword: &WordView) -> bool {
    let qlen  = qword.len();
    let rlen  = if qword.fin { rword.len() } else { min!(qlen, rword.len()) };

    if qlen <= 1 || rlen <= 1 {
        return qlen == rlen;
    }

    let long  = max!(qlen, rlen);
    let short = min!(qlen, rlen);
    let dist  = 1.0 - (short as f64 / long as f64);

    dist < LENGTH_THRESHOLD
}


pub fn jaccard_check(rword: &WordView, qword: &WordView) -> bool {
    let rslice = if qword.fin {
        rword.chars()
    } else {
        &rword.chars()[.. min!(qword.len() + 1, rword.len())]
    };
    let dist   = JACCARD.with(|j| j.rel_dist(rslice, qword.chars()));
    dist < JACCARD_THRESHOLD
}


#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use crate::tokenization::TextOwn;
    use crate::lang::{Lang, CharClass, lang_english};
    use super::{length_check, jaccard_check, word_match};


    fn text(s: &str) -> TextOwn {
        let lang = Lang::new();
        TextOwn::from_str(s).split(&CharClass::Whitespace, &lang)
    }

    #[test]
    fn length_check_len_4() {
        let sample = [
            (false, "m"),
            (false, "ma"),
            (true,  "mai"),
            (true,  "mail"),
            (true,  "mailb"),
            (false, "mailbo"),
            (false, "mailbox"),
        ];
        for &(expect, query) in sample.iter() {
            let rtext  = text("mail");
            let qtext  = text(query);
            let result = length_check(&rtext.view(0), &qtext.view(0));
            assert_eq!(result, expect, "Failed length_check(\"mail\", \"{}\") == {}", query, expect);
        }
    }

    #[test]
    fn length_check_len_7() {
        let sample = [
            (false, "m"),
            (false, "ma"),
            (false, "mai"),
            (false, "mail"),
            (false, "mailb"),
            (true,  "mailbo"),
            (true,  "mailbox"),
            (true,  "mailboxe"),
            (true,  "mailboxes"),
            (false, "mailboxess"),
        ];
        for &(expect, query) in sample.iter() {
            let rtext  = text("mailbox");
            let qtext  = text(query);
            let result = length_check(&rtext.view(0), &qtext.view(0));
            assert_eq!(result, expect, "Failed length_check(\"mailbox\", \"{}\") == {}", query, expect);
        }
    }

    #[test]
    fn jaccard_check_len_4() {
        let sample = [
            (true,  "mail"),
            (true,  "bail"),
            (false, "bait"),
            (false, "balt"),
            (false, "bolt"),
        ];
        for &(expect, query) in sample.iter() {
            let rtext  = text("mail");
            let qtext  = text(query);
            let result = jaccard_check(&rtext.view(0), &qtext.view(0));
            assert_eq!(result, expect, "Failed jaccard_check(\"mail\", \"{}\") == {}", query, expect);
        }
    }

    #[test]
    fn jaccard_check_len_7() {
        let sample = [
            (true,  "mailbox"),
            (true,  "mailbot"),
            (true,  "railbot"),
            (false, "raidbot"),
            (false, "roidbot"),
        ];
        for &(expect, query) in sample.iter() {
            let rtext  = text("mailbox");
            let qtext  = text(query);
            let result = jaccard_check(&rtext.view(0), &qtext.view(0));
            assert_eq!(result, expect, "Failed jaccard_check(\"mailbox\", \"{}\") == {}", query, expect);
        }
    }

    #[test]
    fn jaccard_check_len_7_reduction() {
        let sample = [
            (true,  "mailbox"),
            (true,  "mailbxx"),
            (true,  "mailxxx"),
            (true,  "maixxxx"),
            (false, "maxxxxx"),
        ];
        for &(expect, query) in sample.iter() {
            let rtext  = text("mailbox");
            let qtext  = text(query);
            let result = jaccard_check(&rtext.view(0), &qtext.view(0));
            assert_eq!(result, expect, "Failed jaccard_check(\"mailbox\", \"{}\") == {}", query, expect);
        }
    }

    #[test]
    fn jaccard_check_unfinished() {
        let sample = [
            (true,  "m"),
            (true,  "ma"),
            (true,  "mai"),
            (true,  "mail"),
            (true,  "mailb"),
            (true,  "mailbo"),
            (true,  "mailbox"),
            (true,  "mailboxe"),
            (false, "mailboxes"),
        ];
        for &(expect, query) in sample.iter() {
            let rtext  = text("mail");
            let qtext  = text(query).fin(false);
            let result = jaccard_check(&rtext.view(0), &qtext.view(0));
            assert_eq!(result, expect, "Failed jaccard_check(\"mail\", \"{}\") == {}", query, expect);
        }
    }

    #[test]
    fn length_check_unfinished() {
        let sample = [
            (true,  "m"),
            (true,  "ma"),
            (true,  "mai"),
            (true,  "mail"),
            (true,  "mailb"),
            (false, "mailbo"),
            (false, "mailbox"),
        ];
        for &(expect, query) in sample.iter() {
            let rtext  = text("mail");
            let qtext  = text(query).fin(false);
            let result = length_check(&rtext.view(0), &qtext.view(0));
            assert_eq!(result, expect, "Failed length_check(\"mail\", \"{}\") == {}", query, expect);
        }
    }

    // Match word
    // ----------------------------------------------------------------

    #[test]
    fn match_word_empty_both() {
        let qtext  = TextOwn::from_str("");
        let rtext  = TextOwn::from_str("");
        assert_eq!(word_match(&rtext.view(0), &qtext.view(0)), None);
    }


    #[test]
    fn match_word_empty_record() {
        let qtext  = TextOwn::from_str("mailbox").fin(false);
        let rtext  = TextOwn::from_str("");
        assert_eq!(word_match(&rtext.view(0), &qtext.view(0)), None);
    }


    #[test]
    fn match_word_empty_query() {
        let qtext  = TextOwn::from_str("").fin(false);
        let rtext  = TextOwn::from_str("mailbox");
        assert_eq!(word_match(&rtext.view(0), &qtext.view(0)), None);
    }


    // Match word: prefixed by same length word
    // ----------------------------------------------------------------

    #[test]
    fn match_word_full_strict() {
        let qtext  = TextOwn::from_str("mailbox").fin(false);
        let rtext  = TextOwn::from_str("mailbox");
        assert_debug_snapshot!(word_match(&rtext.view(0), &qtext.view(0)));
    }


    #[test]
    fn match_word_full_fuzzy_insertion() {
        let qtext  = TextOwn::from_str("mailybox").fin(false);
        let rtext  = TextOwn::from_str("mailbox");
        assert_debug_snapshot!(word_match(&rtext.view(0), &qtext.view(0)));
    }


    #[test]
    fn match_word_full_fuzzy_deletion() {
        let qtext  = TextOwn::from_str("mailox").fin(false);
        let rtext  = TextOwn::from_str("mailbox");
        assert_debug_snapshot!(word_match(&rtext.view(0), &qtext.view(0)));
    }


    #[test]
    fn match_word_full_fuzzy_transposition() {
        let qtext  = TextOwn::from_str("maiblox").fin(false);
        let rtext  = TextOwn::from_str("mailbox");
        assert_debug_snapshot!(word_match(&rtext.view(0), &qtext.view(0)));
    }


    #[test]
    fn match_word_full_query_too_long() {
        let qtext1 = TextOwn::from_str("mailboxes").fin(true);
        let qtext2 = TextOwn::from_str("mailboxes").fin(false);
        let rtext  = TextOwn::from_str("mail");
        assert_debug_snapshot!(word_match(&rtext.view(0), &qtext1.view(0)));
        assert_debug_snapshot!(word_match(&rtext.view(0), &qtext2.view(0)));
    }

    #[test]
    fn match_word_full_stem() {
        let lang   = lang_english();
        let qtext1 = TextOwn::from_str("university").set_stem(&lang);
        let qtext2 = TextOwn::from_str("university");
        let rtext  = TextOwn::from_str("universe").set_stem(&lang);
        assert_debug_snapshot!(word_match(&rtext.view(0), &qtext1.view(0)));
        assert_debug_snapshot!(word_match(&rtext.view(0), &qtext2.view(0)));
    }


    // Match word: prefixed by lesser length word
    // ----------------------------------------------------------------

    #[test]
    fn match_word_partial_strict() {
        let qtext  = TextOwn::from_str("mailb").fin(false);
        let rtext  = TextOwn::from_str("mailbox");
        assert_debug_snapshot!(word_match(&rtext.view(0), &qtext.view(0)));
    }


    #[test]
    fn match_word_partial_fuzzy_insertion() {
        let qtext  = TextOwn::from_str("maiylb").fin(false);
        let rtext  = TextOwn::from_str("mailbox");
        assert_debug_snapshot!(word_match(&rtext.view(0), &qtext.view(0)));
    }


    #[test]
    fn match_word_partial_fuzzy_deletion() {
        let qtext  = TextOwn::from_str("maib").fin(false);
        let rtext  = TextOwn::from_str("mailbox");
        assert_debug_snapshot!(word_match(&rtext.view(0), &qtext.view(0)));
    }


    #[test]
    fn match_word_partial_fuzzy_transposition() {
        let qtext  = TextOwn::from_str("malib").fin(false);
        let rtext  = TextOwn::from_str("mailbox");
        assert_debug_snapshot!(word_match(&rtext.view(0), &qtext.view(0)));
    }
}
