use fnv::{FnvHashSet as HashSet};
use crate::tokenization::Text;
use super::WordMatch;
use super::word::word_match;


pub fn text_match(rtext: &Text<&[char]>, qtext: &Text<&[char]>) -> Vec<WordMatch> {
    let rchars   = &rtext.chars;
    let qchars   = &qtext.chars;
    let capacity = min!(rtext.words.len(), qtext.words.len());
    let mut rtaken: HashSet<usize>  = HashSet::with_capacity_and_hasher(capacity, Default::default());
    let mut qtaken: HashSet<usize>  = HashSet::with_capacity_and_hasher(capacity, Default::default());
    let mut matches: Vec<WordMatch> = Vec::with_capacity(capacity);

    for qword in qtext.words.iter() {
        if qtaken.contains(&qword.ix) { continue; }

        let mut candidate: Option<(WordMatch, Option<WordMatch>)> = None;

        for rword in rtext.words.iter() {
            if rtaken.contains(&rword.ix) { continue; }

            let new_candidate = None
                .or_else(|| {
                    let rnext = rtext.words.get(rword.ix + 1)?;
                    if qword.len() <= rword.len() + rword.dist(rnext) { return None; }
                    if rtaken.contains(&(rword.ix + 1)) { return None; }
                    let m        = word_match(&rword.join(rnext), qword, rchars, qchars)?;
                    let (m1, m2) = m.split_record(rword, rnext);
                    Some((m1, Some(m2)))
                })
                .or_else(|| {
                    let qnext = qtext.words.get(qword.ix + 1)?;
                    if rword.len() <= qword.len() + qword.dist(qnext) { return None; }
                    if qtaken.contains(&(qword.ix + 1)) { return None; }
                    let m        = word_match(rword, &qword.join(qnext), rchars, qchars)?;
                    let (m1, m2) = m.split_query(qword, qnext);
                    Some((m1, Some(m2)))
                })
                .or_else(|| {
                    let m = word_match(rword, qword, rchars, qchars)?;
                    Some((m, None))
                });

            if let Some(new_candidate) = new_candidate {
                if !new_candidate.0.record.function || new_candidate.1.is_some() {
                    candidate = Some(new_candidate);
                    break;
                }
                if !candidate.is_some() {
                    candidate = Some(new_candidate);
                    continue;
                }
            }
        }

        if let Some((m1, m2)) = candidate {
            qtaken.insert(m1.query.ix);
            rtaken.insert(m1.record.ix);
            matches.push(m1);
            if let Some(m2) = m2 {
                qtaken.insert(m2.query.ix);
                rtaken.insert(m2.record.ix);
                matches.push(m2);
            }
        }
    }

    matches
}


#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use crate::tokenization::Text;
    use crate::lang::{Chars, lang_english};
    use super::{text_match};


    fn text(s: &str) -> Text<Vec<char>> {
        Text::from_str(s).split(&[Chars::Punctuation, Chars::Whitespaces])
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

        let lang = Some(lang_english());
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
    fn match_text_joined_query_unfihished() {
        let qtext = text("micro bio").fin(false);
        let rtext = text("microbiology");
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }

    #[test]
    fn match_text_joined_query_typos() {
        let qtext = text("mcro byology").fin(false);
        let rtext = text("microbiology");
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }

    #[test]
    fn match_text_joined_record() {
        let qtext = text("wifi router").fin(false);
        let rtext = text("wi fi router");
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }

    #[test]
    fn match_text_joined_record_typos() {
        let qtext = text("mcrobiology").fin(false);
        let rtext = text("micro biology");
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }

    #[test]
    fn match_text_joined_record_unfinished() {
        let qtext = text("microbio").fin(false);
        let rtext = text("micro biology");
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }

    #[test]
    fn match_text_joined_regression_1() {
        let qtext = text("especiall").fin(false);
        let rtext = text("special, year");
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }
}
