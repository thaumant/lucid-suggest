use std::collections::HashSet;
use crate::tokenization::{Word, Text};
use super::WordMatch;
use super::word::{word_match, build_matrix};


pub fn text_match(rtext: &Text<&[char]>, qtext: &Text<&[char]>) -> Vec<WordMatch> {
    let capacity = min!(rtext.words.len(), qtext.words.len());
    let mut rtaken: HashSet<usize>  = HashSet::with_capacity(capacity);
    let mut qtaken: HashSet<usize>  = HashSet::with_capacity(capacity);
    let mut matches: Vec<WordMatch> = Vec::with_capacity(capacity);

    for qword in qtext.words.iter() {
        let mut found: Option<WordMatch> = None;

        if qtaken.contains(&qword.ix) { continue; }

        let qnext = Some(())
            .filter(|_| !qtaken.contains(&(qword.ix + 1)))
            .and_then(|_| qtext.words.get(qword.ix + 1));

        let qpair = qnext.map(|w| join_words(qword, w));

        for rword in rtext.words.iter() {
            if rtaken.contains(&rword.ix) { continue; }

            let rnext = Some(())
                .filter(|_| !rtaken.contains(&(rword.ix + 1)))
                .and_then(|_| rtext.words.get(rword.ix + 1));

            let rpair = rnext.map(|w| join_words(rword, w));

            build_matrix(
                &rpair.clone().unwrap_or(rword.clone()),
                &qpair.clone().unwrap_or(qword.clone()),
                &rtext.chars,
                &qtext.chars
            );

            let joined_matches = None
                .or_else(|| {
                    let m = word_match(&rpair?, qword, &rtext.chars, &qtext.chars, true)?;
                    if !m.fin { return None; }
                    Some(m.split_record(rword, rnext?))
                })
                .or_else(|| {
                    let m = word_match(rword, &qpair.clone()?, &rtext.chars, &qtext.chars, true)?;
                    Some(m.split_query(qword, qnext?))
                });

            if let Some((m1, m2)) = joined_matches {
                found.take();
                qtaken.insert(m1.query.ix);
                qtaken.insert(m2.query.ix);
                rtaken.insert(m1.record.ix);
                rtaken.insert(m2.record.ix);
                matches.push(m1);
                matches.push(m2);
                break;
            }

            if let Some(m) = word_match(rword, qword, &rtext.chars, &qtext.chars, true) {
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
            rtaken.insert(m.record.ix);
            qtaken.insert(m.query.ix);
            matches.push(m);
        }
    }

    matches
}


pub fn join_words(w1: &Word, w2: &Word) -> Word {
    Word {
        ix:    w1.ix,
        place: (w1.place.0, w2.place.1),
        stem:  w2.place.0 - w1.place.0 + w2.stem,
        pos:   None,
        fin:   w2.fin,
    }
}


#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use crate::tokenization::{Chars, Text};
    use crate::lang::lang_english;
    use super::{text_match};


    fn text(s: &str) -> Text<Vec<char>> {
        Text::from_str(s).split(&Chars::Whitespaces)
    }


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

    #[test]
    fn match_text_joined_query() {
        let qtext = text("wi fi router").fin(false);
        let rtext = text("wifi router");
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }

    #[test]
    fn match_text_joined_record() {
        let qtext = text("wifi router").fin(false);
        let rtext = text("wi fi router");
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }
}
