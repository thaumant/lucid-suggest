use crate::tokenization::Word;
use super::WordMatch;
use super::damlev::DamerauLevenshtein;
use super::jaccard::Jaccard;


const LENGTH_THRESHOLD:  f64 = 0.26;
const JACCARD_THRESHOLD: f64 = 0.41;
const DAMLEV_THRESHOLD:  f64 = 0.26;

thread_local! {
    static DAMLEV:  DamerauLevenshtein = DamerauLevenshtein::new();
    static JACCARD: Jaccard<char>      = Jaccard::new();
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
    let mut best_match: Option<WordMatch> = None;

    DAMLEV.with(|damlev| {
        damlev.distance(qchars, rchars);
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
                // Left margin is for insertion/deletion, not for both prefixes at the same time.
                if rlen == left && qlen == left  { continue; }
                // Compare full words only if query is finished.
                if qword.fin && rlen < rword.stem { break; }
                if qword.fin && qlen < qword.stem { break; }
                // Words with 2+ insertions/deletions are mismatched by default.
                if (qlen as isize - rlen as isize).abs() > 1 { continue; }

                let dist = dists.get(qlen + 1, rlen + 1);
                let rel  = dist as f64 / max!(qlen, rlen, 1) as f64;

                if rel > DAMLEV_THRESHOLD { continue; }

                best_match = best_match
                    .take()
                    .filter(|m| m.typos <= dist)
                    .or_else(|| Some(WordMatch::new(
                        qword,
                        rword,
                        qlen,
                        rlen,
                        dist,
                    )));

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

    if qlen <= 1 || rlen <= 1 {
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
    use crate::utils::to_vec;
    use crate::tokenization::{Chars, Word, Text};
    use crate::lang::lang_english;
    use super::{length_check, jaccard_check, word_match};


    fn text(s: &str) -> Text<Vec<char>> {
        Text::from_str(s).split(&Chars::Whitespaces)
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
            let result = length_check(&rtext.words[0], &qtext.words[0]);
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
            let result = length_check(&rtext.words[0], &qtext.words[0]);
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
            let result = jaccard_check(&rtext.words[0], &qtext.words[0], &rtext.chars, &qtext.chars);
            assert_eq!(result, expect, "Failed jaccard_check(\"mail\", \"{}\") == {}", query, expect);
        }
    }

    #[test]
    fn jaccard_check_len_7() {
        let sample = [
            (true,  "mailbox"),
            (true,  "mailbot"),
            (false, "railbot"),
            (false, "raidbot"),
            (false, "roidbot"),
        ];
        for &(expect, query) in sample.iter() {
            let rtext  = text("mailbox");
            let qtext  = text(query);
            let result = jaccard_check(&rtext.words[0], &qtext.words[0], &rtext.chars, &qtext.chars);
            assert_eq!(result, expect, "Failed jaccard_check(\"mailbox\", \"{}\") == {}", query, expect);
        }
    }

    #[test]
    fn jaccard_check_len_7_reduction() {
        let sample = [
            (true,  "mailbox"),
            (true,  "mailbxx"),
            (true,  "mailxxx"),
            (false, "maixxxx"),
            (false, "maxxxxx"),
        ];
        for &(expect, query) in sample.iter() {
            let rtext  = text("mailbox");
            let qtext  = text(query);
            let result = jaccard_check(&rtext.words[0], &qtext.words[0], &rtext.chars, &qtext.chars);
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
            (false, "mailbox"),
        ];
        for &(expect, query) in sample.iter() {
            let rtext  = text("mail");
            let qtext  = text(query).fin(false);
            let result = jaccard_check(&rtext.words[0], &qtext.words[0], &rtext.chars, &qtext.chars);
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
            let result = length_check(&rtext.words[0], &qtext.words[0]);
            assert_eq!(result, expect, "Failed length_check(\"mail\", \"{}\") == {}", query, expect);
        }
    }

    // Match word
    // ----------------------------------------------------------------

    #[test]
    fn match_word_empty_both() {
        let qchars = to_vec("");
        let rchars = to_vec("");
        let qword  = Word::new(qchars.len()).fin(false);
        let rword  = Word::new(rchars.len());
        assert_eq!(word_match(&rword, &qword, &rchars[..], &qchars[..]), None);
    }


    #[test]
    fn match_word_empty_record() {
        let qchars = to_vec("mailbox");
        let rchars = to_vec("");
        let qword  = Word::new(qchars.len()).fin(false);
        let rword  = Word::new(rchars.len());
        assert_eq!(word_match(&rword, &qword, &rchars[..], &qchars[..]), None);
    }


    #[test]
    fn match_word_empty_query() {
        let qchars = to_vec("");
        let rchars = to_vec("mailbox");
        let qword  = Word::new(qchars.len()).fin(false);
        let rword  = Word::new(rchars.len());
        assert_eq!(word_match(&rword, &qword, &rchars[..], &qchars[..]), None);
    }


    // Match word: prefixed by same length word
    // ----------------------------------------------------------------

    #[test]
    fn match_word_full_strict() {
        let qchars = to_vec("mailbox");
        let rchars = to_vec("mailbox");
        let qword  = Word::new(qchars.len()).fin(false);
        let rword  = Word::new(rchars.len());
        assert_debug_snapshot!(word_match(&rword, &qword, &rchars[..], &qchars[..]));
    }


    #[test]
    fn match_word_full_fuzzy_insertion() {
        let qchars = to_vec("mailybox");
        let rchars = to_vec("mailbox");
        let qword  = Word::new(qchars.len()).fin(false);
        let rword  = Word::new(rchars.len());
        assert_debug_snapshot!(word_match(&rword, &qword, &rchars[..], &qchars[..]));
    }


    #[test]
    fn match_word_full_fuzzy_deletion() {
        let qchars = to_vec("mailox");
        let rchars = to_vec("mailbox");
        let qword  = Word::new(qchars.len()).fin(false);
        let rword  = Word::new(rchars.len());
        assert_debug_snapshot!(word_match(&rword, &qword, &rchars[..], &qchars[..]));
    }


    #[test]
    fn match_word_full_fuzzy_transposition() {
        let qchars = to_vec("maiblox");
        let rchars = to_vec("mailbox");
        let qword  = Word::new(qchars.len()).fin(false);
        let rword  = Word::new(rchars.len());
        assert_debug_snapshot!(word_match(&rword, &qword, &rchars[..], &qchars[..]));
    }


    #[test]
    fn match_word_full_query_too_long() {
        let qchars1 = to_vec("mailboxes");
        let qchars2 = to_vec("mailboxes");
        let rchars  = to_vec("mail");
        let qword1  = Word::new(qchars1.len()).fin(true);
        let qword2  = Word::new(qchars2.len()).fin(false);
        let rword   = Word::new(rchars.len());
        assert_debug_snapshot!(word_match(&rword, &qword1, &rchars[..], &qchars1[..]));
        assert_debug_snapshot!(word_match(&rword, &qword2, &rchars[..], &qchars2[..]));
    }

    #[test]
    fn match_word_full_stem() {
        let     rchars  = to_vec("universe");
        let     qchars1 = to_vec("university");
        let     qchars2 = to_vec("university");
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
        let qchars = to_vec("mailb");
        let rchars = to_vec("mailbox");
        let qword = Word::new(qchars.len()).fin(false);
        let rword = Word::new(rchars.len());
        assert_debug_snapshot!(word_match(&rword, &qword, &rchars[..], &qchars[..]));
    }


    #[test]
    fn match_word_partial_fuzzy_insertion() {
        let qchars = to_vec("maiylb");
        let rchars = to_vec("mailbox");
        let qword  = Word::new(qchars.len()).fin(false);
        let rword  = Word::new(rchars.len());
        assert_debug_snapshot!(word_match(&rword, &qword, &rchars[..], &qchars[..]));
    }


    #[test]
    fn match_word_partial_fuzzy_deletion() {
        let qchars = to_vec("maib");
        let rchars = to_vec("mailbox");
        let qword  = Word::new(qchars.len()).fin(false);
        let rword  = Word::new(rchars.len());
        assert_debug_snapshot!(word_match(&rword, &qword, &rchars[..], &qchars[..]));
    }


    #[test]
    fn match_word_partial_fuzzy_transposition() {
        let qchars = to_vec("malib");
        let rchars = to_vec("mailbox");
        let qword  = Word::new(qchars.len()).fin(false);
        let rword  = Word::new(rchars.len());
        assert_debug_snapshot!(word_match(&rword, &qword, &rchars[..], &qchars[..]));
    }
}
