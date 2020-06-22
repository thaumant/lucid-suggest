use std::collections::HashSet;
use crate::tokenization::Text;
use super::{WordMatch, word_match};


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
}
