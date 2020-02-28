use crate::lexis::{Text, WordMatch};
use crate::search::{Hit, ScoreType};


pub struct Scorer<'a, Src: Iterator<Item=Hit<'a>>> {
    source: Src,
    query:  &'a Text<&'a [char]>,
}


impl<'a, Src: Iterator<Item=Hit<'a>>> Scorer<'a, Src> {
    pub fn new(source: Src, query: &'a Text<&'a [char]>) -> Self {
        Self { source, query }
    }
}


impl<'a, Src: Iterator<Item=Hit<'a>>> Iterator for Scorer<'a, Src> {
    type Item = Hit<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.source
            .next()
            .map(|mut hit| {
                score(self.query, &mut hit);
                hit
            })
    }
}


pub fn score<'a, T: AsRef<[char]>>(query: &Text<T>, hit: &mut Hit<'a>) {
    let matches = hit.text.matches(&query);

    hit.scores[ScoreType::Matches] = score_matches_up(&matches);
    hit.scores[ScoreType::Typos]   = score_typos_down(&matches);
    hit.scores[ScoreType::Trans]   = score_trans_down(&matches);
    hit.scores[ScoreType::Fin]     = score_fin_up(&matches);
    hit.scores[ScoreType::Offset]  = score_offset_down(&matches);

    hit.matches = matches;
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

    fn text(s: &str) -> Text<Vec<char>> {
        Text::from_str(s).split(&Chars::Whitespaces)
    }

    #[test]
    fn test_score_typos() {
        let r  = text("small yellow metal mailbox");
        let q1 = text("yellow mailbox").fin(false);
        let q2 = text("yelow maiblox").fin(false);
        let q3 = text("yellow mail").fin(false);
        assert_eq!(score_typos_down(&r.matches(&q1)), -0);
        assert_eq!(score_typos_down(&r.matches(&q2)), -2);
        assert_eq!(score_typos_down(&r.matches(&q3)), -3);
    }

    #[test]
    fn test_score_offset() {
        let r  = text("small yellow metal mailbox");
        let q1 = text("smal mailbox").fin(false);
        let q2 = text("yelow mailbox").fin(false);
        let q3 = text("metol maiblox").fin(false);
        assert_eq!(score_offset_down(&r.matches(&q1)), -0);
        assert_eq!(score_offset_down(&r.matches(&q2)), -1);
        assert_eq!(score_offset_down(&r.matches(&q3)), -2);
    }
}
