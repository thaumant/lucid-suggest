use std::default::Default;
use crate::lexis::{Text, WordMatch};
use super::Hit;


#[derive(Debug, Clone)]
pub struct Scores {
    pub matches: Vec<WordMatch>,
    pub typos:   usize,
    pub offset:  usize,
    pub trans:   usize,
    pub fin:     bool,
}


impl Default for Scores {
    fn default() -> Scores {
        Scores {
            matches: Vec::new(),
            typos:   0,
            offset:  0,
            trans:   0,
            fin:     true,
        }
    }
}


pub fn score(query: &Text, hits: &mut Vec<Hit>) {
    for hit in hits.iter_mut() {
        hit.scores.matches = hit.text.matches(&query);
        hit.scores.typos   = score_typos(&hit.scores.matches);
        hit.scores.offset  = score_offset(&hit.scores.matches);
        hit.scores.trans   = score_transpositions(&hit.scores.matches);
        hit.scores.fin     = score_fin(&hit.scores.matches);
    }
}


pub fn score_typos(matches: &[WordMatch]) -> usize {
    let mut typos = 0;
    for m in matches {
        typos += m.typos + m.record.slice.0 + (m.record.len - m.record.slice.1);
    }
    typos
}


pub fn score_offset(matches: &[WordMatch]) -> usize {
    matches.iter()
        .map(|m| m.record.pos)
        .min()
        .unwrap_or(0)
}


pub fn score_transpositions(matches: &[WordMatch]) -> usize {
    if matches.is_empty() { return 0; }

    let mut transpositions = 0;

    let prevs = &matches[ .. matches.len() - 1];
    let nexts = &matches[1..];
    for (prev, next) in prevs.iter().zip(nexts.iter()) {
        let prev_ix = prev.record.pos + 1;
        let next_ix = next.record.pos;
        if prev_ix > next_ix { transpositions += prev_ix - next_ix; }
        if prev_ix < next_ix { transpositions += next_ix - prev_ix; }
    }

    transpositions
}


pub fn score_fin(matches: &[WordMatch]) -> bool {
    if let Some(m) = matches.last() {
        m.fin
    } else {
        true
    }
}


#[cfg(test)]
mod tests {
    use crate::lexis::{Text, Chars};
    use super::{score_typos, score_offset};

    fn chars(s: &str) -> Vec<char> {
        s.chars().collect()
    }

    fn record(chars: &[char]) -> Text {
        Text::new(chars).split(&Chars::Whitespaces)
    }

    fn query(s: &[char]) -> Text {
        record(s).fin(false)
    }


    #[test]
    fn test_score_typos() {
        let c0 = chars("small yellow metal mailbox");
        let c1 = chars("yellow mailbox");
        let c2 = chars("yelow maiblox");
        let c3 = chars("yellow mail");
        
        let r  = record(&c0);
        let q1 = query(&c1);
        let q2 = query(&c2);
        let q3 = query(&c3);

        assert_eq!(score_typos(&r.matches(&q1)), 0);
        assert_eq!(score_typos(&r.matches(&q2)), 2);
        assert_eq!(score_typos(&r.matches(&q3)), 3);
    }

    #[test]
    fn test_score_offset() {
        let c0 = chars("small yellow metal mailbox");
        let c1 = chars("smal mailbox");
        let c2 = chars("yelow mailbox");
        let c3 = chars("metol maiblox");
        
        let r  = record(&c0);
        let q1 = query(&c1);
        let q2 = query(&c2);
        let q3 = query(&c3);

        assert_eq!(score_offset(&r.matches(&q1)), 0);
        assert_eq!(score_offset(&r.matches(&q2)), 1);
        assert_eq!(score_offset(&r.matches(&q3)), 2);
    }
}