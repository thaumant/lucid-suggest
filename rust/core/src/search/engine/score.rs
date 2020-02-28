use crate::lexis::{Text, WordMatch};
use super::Hit;


pub enum Score {
    Matches = 0,
    Typos   = 1,
    Trans   = 2,
    Fin     = 3,
    Offset  = 4,
}


#[derive(Debug, Clone)]
pub struct Scores {
    pub matches: Vec<WordMatch>,
    pub linear:  [isize; 5],
}


impl std::ops::Index<Score> for Scores {
    type Output = isize;

    fn index(&self, score: Score) -> &Self::Output {
        &self.linear[score as usize]
    }
}


impl std::ops::IndexMut<Score> for Scores {
    fn index_mut(&mut self, score: Score) -> &mut Self::Output {
        &mut self.linear[score as usize]
    }
}


impl Default for Scores {
    fn default() -> Scores {
        Scores {
            matches: Vec::new(),
            linear:  [0; 5],
        }
    }
}


pub fn score<T: AsRef<[char]>, U: AsRef<[char]>>(query: &Text<T>, hit: &mut Hit<U>) {
    let matches = hit.text.matches(&query);

    hit.scores[Score::Matches] = score_matches_up(&matches);
    hit.scores[Score::Typos]   = score_typos_down(&matches);
    hit.scores[Score::Trans]   = score_trans_down(&matches);
    hit.scores[Score::Fin]     = score_fin_up(&matches);
    hit.scores[Score::Offset]  = score_offset_down(&matches);

    hit.scores.matches = matches;
}


pub fn score_matches_up(matches: &[WordMatch]) -> isize {
    matches.len() as isize
}


pub fn score_typos_down(matches: &[WordMatch]) -> isize {
    let mut typos = 0;
    for m in matches {
        typos += m.typos + m.record.slice.0 + (m.record.len - m.record.slice.1);
    }
    -(typos as isize)
}


pub fn score_trans_down(matches: &[WordMatch]) -> isize {
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

    -(transpositions as isize)
}


pub fn score_fin_up(matches: &[WordMatch]) -> isize {
    if let Some(m) = matches.last() {
        m.fin as isize
    } else {
        1
    }
}


pub fn score_offset_down(matches: &[WordMatch]) -> isize {
    let offset = matches.iter()
        .map(|m| m.record.pos)
        .min()
        .unwrap_or(0);
    -(offset as isize)
}


#[cfg(test)]
mod tests {
    use crate::lexis::{Text, Chars};
    use super::{score_typos_down, score_offset_down};
    use std::borrow::Cow;

    fn chars(s: &str) -> Vec<char> {
        s.chars().collect()
    }

    fn record<'a>(chars: &'a [char]) -> Text<Cow<'a, [char]>> {
        Text::new_cow(Cow::Borrowed(&chars[..]))
            .split(&Chars::Whitespaces)
    }

    fn query<'a>(s: &'a [char]) -> Text<Cow<'a, [char]>> {
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

        assert_eq!(score_typos_down(&r.matches(&q1)), -0);
        assert_eq!(score_typos_down(&r.matches(&q2)), -2);
        assert_eq!(score_typos_down(&r.matches(&q3)), -3);
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

        assert_eq!(score_offset_down(&r.matches(&q1)), -0);
        assert_eq!(score_offset_down(&r.matches(&q2)), -1);
        assert_eq!(score_offset_down(&r.matches(&q3)), -2);
    }
}