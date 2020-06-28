use crate::tokenization::Text;
use crate::matching::text_match;
use crate::search::{Hit, ScoreType};


pub fn score(query: &Text<&[char]>, hit: &mut Hit) {
    hit.matches = text_match(&hit.title, &query);

    hit.scores[ScoreType::SameWords]   = score_words_up(hit);
    hit.scores[ScoreType::SameNonFunc] = score_nonfunction_up(hit);
    hit.scores[ScoreType::Typos]       = score_typos_down(hit);
    hit.scores[ScoreType::Trans]       = score_trans_down(hit);
    hit.scores[ScoreType::Fin]         = score_fin_up(hit);
    hit.scores[ScoreType::Offset]      = score_offset_down(hit);
    hit.scores[ScoreType::Rating]      = score_rating_up(hit);
    hit.scores[ScoreType::WordLen]     = score_word_len_down(hit);
    hit.scores[ScoreType::CharLen]     = score_char_len_down(hit);
}


pub fn score_words_up(hit: &Hit) -> isize {
    hit.matches.len() as isize
}


pub fn score_nonfunction_up(hit: &Hit) -> isize {
    hit.matches.iter()
        .filter(|m| !m.record.function)
        .count() as isize
}


pub fn score_typos_down(hit: &Hit) -> isize {
    let mut typos = 0;
    for m in &hit.matches {
        typos += m.typos + m.record.slice.0 + (m.record.len - m.record.slice.1);
    }
    -(typos as isize)
}


pub fn score_trans_down(hit: &Hit) -> isize {
    if hit.matches.is_empty() { return 0; }

    let mut transpositions = 0;

    let prevs = &hit.matches[ .. hit.matches.len() - 1];
    let nexts = &hit.matches[1..];
    for (prev, next) in prevs.iter().zip(nexts.iter()) {
        let prev_ix = prev.record.ix + 1;
        let next_ix = next.record.ix;
        if prev_ix > next_ix { transpositions += prev_ix - next_ix; }
        if prev_ix < next_ix { transpositions += next_ix - prev_ix; }
    }

    -(transpositions as isize)
}


pub fn score_fin_up(hit: &Hit) -> isize {
    if let Some(m) = hit.matches.last() {
        m.fin as isize
    } else {
        1
    }
}


pub fn score_offset_down(hit: &Hit) -> isize {
    let offset = hit.matches.iter()
        .map(|m| m.record.ix)
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
    use crate::tokenization::tokenize_query;
    use crate::store::Record;
    use crate::search::{Hit, ScoreType};
    use super::score;

    #[test]
    fn test_score_typos() {
        let r      = Record::new(10, "small yellow metal mailbox", 0, &None);
        let mut h1 = Hit::from_record(&r);
        let mut h2 = Hit::from_record(&r);
        let mut h3 = Hit::from_record(&r);
        let q1     = tokenize_query("yellow mailbox", &None);
        let q2     = tokenize_query("yelow maiblox", &None);
        let q3     = tokenize_query("yellow mail", &None);
        score(&q1.to_ref(), &mut h1);
        score(&q2.to_ref(), &mut h2);
        score(&q3.to_ref(), &mut h3);
        assert_eq!(h1.scores[ScoreType::Typos], -0);
        assert_eq!(h2.scores[ScoreType::Typos], -2);
        assert_eq!(h3.scores[ScoreType::Typos], -3);
    }

    #[test]
    fn test_score_offset() {
        let r      = Record::new(10, "small yellow metal mailbox", 0, &None);
        let mut h1 = Hit::from_record(&r);
        let mut h2 = Hit::from_record(&r);
        let mut h3 = Hit::from_record(&r);
        let q1     = tokenize_query("smal mailbox", &None);
        let q2     = tokenize_query("yelow mailbox", &None);
        let q3     = tokenize_query("metol maiblox", &None);
        score(&q1.to_ref(), &mut h1);
        score(&q2.to_ref(), &mut h2);
        score(&q3.to_ref(), &mut h3);
        assert_eq!(h1.scores[ScoreType::Offset], -0);
        assert_eq!(h2.scores[ScoreType::Offset], -1);
        assert_eq!(h3.scores[ScoreType::Offset], -2);
    }
}
