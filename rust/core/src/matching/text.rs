use std::cmp::Ordering::{Equal, Less};
use std::cell::RefCell;
use crate::tokenization::{Word, TextRef};
use super::WordMatch;
use super::word::word_match;

thread_local! {
    static RMATCHES: RefCell<Vec<Option<WordMatch>>> = RefCell::new(Vec::with_capacity(20));
    static QMATCHES: RefCell<Vec<Option<WordMatch>>> = RefCell::new(Vec::with_capacity(20));
}


pub fn text_match(rtext: &TextRef, qtext: &TextRef) -> (Vec<WordMatch>, Vec<WordMatch>) {
    RMATCHES.with(|rcell| {
    QMATCHES.with(|qcell| {
        let rmatches = &mut *rcell.borrow_mut();
        let qmatches = &mut *qcell.borrow_mut();
        rmatches.clear();
        qmatches.clear();
        rmatches.resize(rtext.words.len(), None);
        qmatches.resize(qtext.words.len(), None);

        for qword in qtext.words.iter() {
            if qmatches[qword.offset].is_some() { continue; }
            let qword = qword.to_view(qtext);

            let mut candidate: Option<(WordMatch, WordMatch)> = None;

            for rword in rtext.words.iter() {
                if rmatches[rword.offset].is_some() { continue; }
                let rword = rword.to_view(rtext);
                let mut stop = false;

                None.or_else(|| {
                        let rnext = rtext.words.get(rword.offset + 1)?.to_view(rtext);
                        if qword.len() < rword.len() + rword.dist(&rnext) { return None; }
                        if rmatches.get(rword.offset + 1)?.is_some() { return None; }
                        let (rmatch,  qmatch)  = word_match(&rword.join(&rnext), &qword)?;
                        let (rmatch1, rmatch2) = rmatch.split(&rword, &rnext)?;
                        let roffset1 = rmatch1.offset;
                        let roffset2 = rmatch2.offset;
                        let qoffset  = qmatch.offset;
                        rmatches[roffset1] = Some(rmatch1);
                        rmatches[roffset2] = Some(rmatch2);
                        qmatches[qoffset]  = Some(qmatch);
                        candidate.take();
                        stop = true;
                        Some(())
                    })
                    .or_else(|| {
                        let qnext = qtext.words.get(qword.offset + 1)?.to_view(qtext);
                        if rword.len() < qword.len() + qword.dist(&qnext) { return None; }
                        if qmatches.get(qword.offset + 1)?.is_some() { return None; }
                        let (rmatch,  qmatch)  = word_match(&rword, &qword.join(&qnext))?;
                        let (qmatch1, qmatch2) = qmatch.split(&qword, &qnext)?;
                        let roffset        = rmatch.offset;
                        let qoffset1       = qmatch1.offset;
                        let qoffset2       = qmatch2.offset;
                        rmatches[roffset]  = Some(rmatch);
                        qmatches[qoffset1] = Some(qmatch1);
                        qmatches[qoffset2] = Some(qmatch2);
                        candidate.take();
                        stop = true;
                        Some(())
                    })
                    .or_else(|| {
                        let (rmatch2, qmatch2) = word_match(&rword, &qword)?;
                        let score2 = rmatch2.match_len() - 2 * (rmatch2.typos.ceil() as usize);
                        let score1 = candidate
                            .as_ref()
                            .map(|(m, _)| m.match_len() - 2 * (m.typos.ceil() as usize))
                            .unwrap_or(0);
                        let replace = match (candidate.as_ref(), score1.cmp(&score2)) {
                            (None, _) => true,
                            (Some(_), Less) => true,
                            (Some(_), Equal) if !rmatch2.func => true,
                            _ => false,
                        };
                        if replace {
                            stop      = !rmatch2.func;
                            candidate = Some((rmatch2, qmatch2));
                        }
                        Some(())
                    });

                if stop {
                    break;
                }
            }

            if let Some((rmatch, qmatch)) = candidate {
                let roffset       = rmatch.offset;
                let qoffset       = qmatch.offset;
                rmatches[roffset] = Some(rmatch);
                qmatches[qoffset] = Some(qmatch);
            }
        }

        let rmatches2 = rmatches.drain(..).filter_map(|m| m).collect::<Vec<_>>();
        let qmatches2 = qmatches.drain(..).filter_map(|m| m).collect::<Vec<_>>();

        (rmatches2, qmatches2)
    }) })
}


#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use crate::tokenization::{Text, TextOwn};
    use crate::lang::{CharClass, lang_basic, lang_english, lang_spanish};
    use super::{text_match};


    fn text(s: &str) -> TextOwn {
        let lang = lang_basic();
        Text::from_str(s)
            .split(&[CharClass::Punctuation, CharClass::Whitespace], &lang)
            .set_char_classes(&lang)
    }


    #[test]
    fn match_text_empty_both() {
        let rtext = text("");
        let qtext = text("").fin(false);
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }


    #[test]
    fn match_text_empty_one() {
        let rtext = text("");
        let qtext = text("mailbox").fin(false);
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
        assert_debug_snapshot!(text_match(&qtext.to_ref(), &rtext.to_ref()));
    }


    #[test]
    fn match_text_singleton_equality() {
        let rtext = text("mailbox");
        let qtext = text("mailbox").fin(false);
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }


    #[test]
    fn match_text_singleton_typos() {
        let rtext = text("mailbox");
        let qtext = text("maiblox").fin(false);
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }


    #[test]
    fn match_text_pair_first() {
        let rtext = text("yellow mailbox");
        let qtext = text("yelow").fin(false);
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }


    #[test]
    fn match_text_pair_second() {
        let rtext = text("yellow mailbox");
        let qtext = text("maiblox").fin(false);
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }


    #[test]
    fn match_text_pair_unfinished() {
        let rtext = text("yellow mailbox");
        let qtext = text("maiblox yel").fin(false);
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }


    #[test]
    fn match_text_intersection() {
        let rtext = text("small yellow metal mailbox");
        let qtext = text("big malibox yelo").fin(false);
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }


    #[test]
    fn match_text_best_rword_first() {
        let rtext = text("theory theme");
        let qtext = text("the").fin(false);
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }


    #[test]
    fn match_text_best_rword_nonfunc() {
        let lang  = lang_english();
        let rtext = text("the theme").set_pos(&lang);
        let qtext = text("the").fin(false).set_pos(&lang);
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }


    #[test]
    fn match_text_best_rword_typos() {
        let lang  = lang_spanish();
        let rtext = text("Cepillo de dientes").set_pos(&lang);
        let qtext = text("de").fin(false).set_pos(&lang);
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }


    #[test]
    fn match_text_regression_best_match() {
        let rtext = text("sneaky");
        let qtext = text("sneak").fin(false);
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }

    #[test]
    fn match_text_joined_query() {
        let rtext = text("wifi router");
        let qtext = text("wi fi router").fin(false);
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }

    #[test]
    fn match_text_joined_query_unfihished() {
        let rtext = text("microbiology");
        let qtext = text("micro bio").fin(false);
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }

    #[test]
    fn match_text_joined_query_typos() {
        let rtext = text("microbiology");
        let qtext = text("mcro byology").fin(false);
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }

    #[test]
    fn match_text_joined_query_short() {
        let rtext = text("t-light");
        let qtext = text("tli").fin(false);
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }

    #[test]
    fn match_text_joined_record() {
        let rtext = text("wi fi router");
        let qtext = text("wifi router").fin(false);
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }

    #[test]
    fn match_text_joined_record_typos() {
        let rtext = text("micro biology");
        let qtext = text("mcrobiology").fin(false);
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }

    #[test]
    fn match_text_joined_record_unfinished() {
        let rtext = text("micro biology");
        let qtext = text("microbio").fin(false);
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }

    #[test]
    fn match_text_joined_regression_1() {
        let rtext = text("special, year");
        let qtext = text("especiall").fin(false);
        assert_debug_snapshot!(text_match(&rtext.to_ref(), &qtext.to_ref()));
    }

    #[test]
    fn match_text_joined_regression_2() {
        let rtext1 = text("50's");
        let rtext2 = text("500w");
        let qtext  = text("50s").fin(false);
        assert_debug_snapshot!(text_match(&rtext1.to_ref(), &qtext.to_ref()));
        assert_debug_snapshot!(text_match(&rtext2.to_ref(), &qtext.to_ref()));
    }
}
