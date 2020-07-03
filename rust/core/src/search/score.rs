use crate::tokenization::{Word, TextRef};
use crate::matching::text_match;
use crate::search::{Hit, ScoreType};


pub fn score(query: &TextRef, hit: &mut Hit) {
    let (rmatches, qmatches) = text_match(&hit.title, &query);
    hit.rmatches = rmatches;
    hit.qmatches = qmatches;

    hit.scores[ScoreType::Chars]   = score_chars_up(hit);
    hit.scores[ScoreType::Words]   = score_words_up(hit);
    hit.scores[ScoreType::Tails]   = score_tails_down(hit);
    hit.scores[ScoreType::Trans]   = score_trans_down(hit);
    hit.scores[ScoreType::Fin]     = score_fin_up(hit);
    hit.scores[ScoreType::Offset]  = score_offset_down(hit);
    hit.scores[ScoreType::Rating]  = score_rating_up(hit);
    hit.scores[ScoreType::WordLen] = score_word_len_down(hit);
    hit.scores[ScoreType::CharLen] = score_char_len_down(hit);
}


pub fn score_chars_up(hit: &Hit) -> isize {
    hit.rmatches
        .iter()
        .map(|m| m.slice.1 - 2 * (m.typos.ceil() as usize))
        .sum::<usize>() as isize
}


pub fn score_words_up(hit: &Hit) -> isize {
    hit.rmatches.iter()
        .filter(|m| !m.func)
        .count() as isize
}


pub fn score_tails_down(hit: &Hit) -> isize {
    let tails = hit.rmatches
        .iter()
        .map(|m| m.slice.0 + m.len - m.slice.1)
        .sum::<usize>();
    -(tails as isize)
}


pub fn score_trans_down(hit: &Hit) -> isize {
    if hit.rmatches.is_empty() { return 0; }
    let mut count = 0;
    let prevs = &hit.rmatches[ .. hit.rmatches.len() - 1];
    let nexts = &hit.rmatches[1..];
    for (prev, next) in prevs.iter().zip(nexts.iter()) {
        if prev.ix + 1 > next.ix { count += prev.ix + 1 - next.ix; }
        if prev.ix + 1 < next.ix { count += next.ix - prev.ix - 1; }
    }
    -(count as isize)
}


pub fn score_fin_up(hit: &Hit) -> isize {
    if let Some(m) = hit.rmatches.last() {
        m.fin as isize
    } else {
        1
    }
}


pub fn score_offset_down(hit: &Hit) -> isize {
    let offset = hit.rmatches.iter()
        .map(|m| m.ix)
        .min()
        .unwrap_or(0);
    -(offset as isize)
}


pub fn score_rating_up(hit: &Hit) -> isize {
    hit.rating as isize
}


pub fn score_word_len_down(hit: &Hit) -> isize {
    -(hit.title.words.len() as isize)
}


pub fn score_char_len_down(hit: &Hit) -> isize {
    -(hit.title.words.iter().map(|w| w.len()).sum::<usize>() as isize)
}


#[cfg(test)]
mod tests {
    use crate::lang::{Lang, lang_english};
    use crate::tokenization::tokenize_query;
    use crate::store::Record;
    use crate::search::{Hit, ScoreType};
    use super::score;

    #[test]
    fn score_chars() {
        let lang   = Lang::new();
        let q      = tokenize_query("quarter of it", &lang);
        let r1     = Record::new(10, "half of it",  0, &lang);
        let r2     = Record::new(20, "quarter",     0, &lang);
        let r3     = Record::new(30, "whole thing", 0, &lang);
        let mut h1 = Hit::from_record(&r1);
        let mut h2 = Hit::from_record(&r2);
        let mut h3 = Hit::from_record(&r3);
        score(&q.to_ref(), &mut h1);
        score(&q.to_ref(), &mut h2);
        score(&q.to_ref(), &mut h3);
        assert_eq!(h1.scores[ScoreType::Chars], 4);
        assert_eq!(h2.scores[ScoreType::Chars], 7);
        assert_eq!(h3.scores[ScoreType::Chars], 0);
    }

    #[test]
    fn score_chars_typos() {
        let lang   = Lang::new();
        let r      = Record::new(10, "small yellow metal mailbox", 0, &lang);
        let mut h1 = Hit::from_record(&r);
        let mut h2 = Hit::from_record(&r);
        let mut h3 = Hit::from_record(&r);
        let q1     = tokenize_query("yellow mailbox", &lang);
        let q2     = tokenize_query("yelow maiblox", &lang);
        let q3     = tokenize_query("yellow mail", &lang);
        score(&q1.to_ref(), &mut h1);
        score(&q2.to_ref(), &mut h2);
        score(&q3.to_ref(), &mut h3);
        assert_eq!(h1.scores[ScoreType::Chars], 13);
        assert_eq!(h2.scores[ScoreType::Chars], 9);
        assert_eq!(h3.scores[ScoreType::Chars], 10);
    }


    #[test]
    fn score_chars_regress_1() {
        let lang   = lang_english();
        let q      = tokenize_query("orn", &lang);
        let r1     = Record::new(10, "ornament", 0, &lang);
        let r2     = Record::new(20, "orange",   0, &lang);
        let mut h1 = Hit::from_record(&r1);
        let mut h2 = Hit::from_record(&r2);
        score(&q.to_ref(), &mut h1);
        score(&q.to_ref(), &mut h2);
        assert!(h1.scores[ScoreType::Chars] > h2.scores[ScoreType::Chars]);
    }


    // TODO try tail char classes for better scoring
    #[test]
    fn score_tails() {
        let lang   = Lang::new();
        let q      = tokenize_query("green", &lang);
        let r1     = Record::new(10, "green",    0, &lang);
        let r2     = Record::new(20, "greens",   0, &lang);
        let r3     = Record::new(30, "greeny",   0, &lang);
        let r4     = Record::new(40, "greenies", 0, &lang);
        let mut h1 = Hit::from_record(&r1);
        let mut h2 = Hit::from_record(&r2);
        let mut h3 = Hit::from_record(&r3);
        let mut h4 = Hit::from_record(&r4);
        score(&q.to_ref(), &mut h1);
        score(&q.to_ref(), &mut h2);
        score(&q.to_ref(), &mut h3);
        score(&q.to_ref(), &mut h4);
        assert_eq!(h1.scores[ScoreType::Tails], -0);
        assert_eq!(h2.scores[ScoreType::Tails], -1);
        assert_eq!(h3.scores[ScoreType::Tails], -1);
        assert_eq!(h4.scores[ScoreType::Tails], -3);
    }

    #[test]
    fn score_offset() {
        let lang   = Lang::new();
        let r      = Record::new(10, "small yellow metal mailbox", 0, &lang);
        let mut h1 = Hit::from_record(&r);
        let mut h2 = Hit::from_record(&r);
        let mut h3 = Hit::from_record(&r);
        let q1     = tokenize_query("smal mailbox", &lang);
        let q2     = tokenize_query("yelow mailbox", &lang);
        let q3     = tokenize_query("metol maiblox", &lang);
        score(&q1.to_ref(), &mut h1);
        score(&q2.to_ref(), &mut h2);
        score(&q3.to_ref(), &mut h3);
        assert_eq!(h1.scores[ScoreType::Offset], -0);
        assert_eq!(h2.scores[ScoreType::Offset], -1);
        assert_eq!(h3.scores[ScoreType::Offset], -2);
    }
}
