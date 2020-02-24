mod score;
mod highlight;

use std::borrow::Cow;
use std::cmp::Ordering;
use crate::lexis::{Text, Chars};

pub use score::{score, Scores};
pub use highlight::highlight;


#[derive(Clone, Debug)]
pub struct Record {
    id: usize,
    text: Text<Vec<char>>,
}


impl Record {
    pub fn new(id: usize, source: &[char]) -> Record {
        Record {
            id,
            text: tokenize_record(source).to_owned(),
        }
    }

    pub fn to_hit<'a>(&'a self) -> Hit<Cow<'a, [char]>> {
        Hit {
            id: self.id,
            text: self.text.to_cow(),
            scores: Default::default(),
        }
    }
}


#[derive(Debug, Clone)]
pub struct Hit<T: AsRef<[char]>> {
    pub id:      usize,
    pub text:    Text<T>,
    pub scores:  Scores,
}


impl<T: AsRef<[char]>> Hit<T> {
    pub fn new(id: usize, text: Text<T>) -> Hit<T> {
        Hit { id, text, scores: Default::default() }
    }
}


pub fn search<T: AsRef<[char]>, U: AsRef<[char]>>(query: &Text<T>, hits: &mut Vec<Hit<U>>) {
    score(query, hits);
    filter(query, hits);
    sort(hits);
}


pub fn filter<T: AsRef<[char]>, U: AsRef<[char]>>(query: &Text<T>, hits: &mut Vec<Hit<U>>) {
    if query.is_empty() {
        return;
    }
    hits.retain(|hit| {
        let matches = &hit.scores.matches;
        if matches.len() == 0 { return false; }
        if matches.len() == 1 && query.words.len() > 1 {
            let word_match = &matches[0];
            let unfinished = !word_match.fin;
            let first_half = (word_match.query.len * 2) < word_match.record.len;
            if unfinished && first_half { return false; }
        }
        true
    });
}


pub fn sort<T: AsRef<[char]>>(hits: &mut Vec<Hit<T>>) {
    hits.sort_by(|hit1, hit2| {
        let s1 = &hit1.scores;
        let s2 = &hit2.scores;
        if s1.matches.len() != s2.matches.len() { return s1.matches.len().cmp(&s2.matches.len()).reverse(); }
        if s1.typos  != s2.typos  { return s1.typos.cmp(&s2.typos); }
        if s1.trans  != s2.trans  { return s1.trans.cmp(&s2.trans); }
        if s1.fin    != s2.fin    { return s1.fin.cmp(&s2.fin).reverse(); }
        if s1.offset != s2.offset { return s1.offset.cmp(&s2.offset); }
        Ordering::Equal
    });
}


pub fn tokenize_record<'a>(source: &'a [char]) -> Text<Cow<'a, [char]>> {
    Text::new_cow(Cow::Borrowed(source))
        .split(&[Chars::Whitespaces, Chars::Control])
        .strip(&[Chars::NotAlphaNum])
        .lower()
}


pub fn tokenize_query<'a>(source: &'a [char]) -> Text<Cow<'a, [char]>> {
    Text::new_cow(Cow::Borrowed(source))
        .fin(false)
        .split(&[Chars::Whitespaces, Chars::Control, Chars::Punctuation])
        .strip(&[Chars::NotAlphaNum])
        .lower()
}


#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use crate::lexis::{Text, Chars};
    use super::{Hit, search};
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
    fn search_empty() {
        let cr1 = chars("brown plush bear");
        let cr2 = chars("metal detector");
        let cr3 = chars("yellow metal mailbox");

        let mut hits = vec![
            Hit::new(10, record(&cr1)),
            Hit::new(20, record(&cr2)),
            Hit::new(30, record(&cr3)),
        ];

        let cq = chars("");
        let q  = query(&cq);

        search(&q, &mut hits);

        assert_debug_snapshot!(hits);
    }

    #[test]
    fn search_equal() {
        let cr1 = chars("brown plush bear");
        let cr2 = chars("metal detector");
        let cr3 = chars("yellow metal mailbox");

        let mut hits = vec![
            Hit::new(10, record(&cr1)),
            Hit::new(20, record(&cr2)),
            Hit::new(30, record(&cr3)),
        ];

        let cq = chars("yelow metall maiblox");
        let q  = query(&cq);

        search(&q, &mut hits);

        assert_debug_snapshot!(hits);
    }

    #[test]
    fn search_partial() {
        let cr1 = chars("brown plush bear");
        let cr2 = chars("metal detector");
        let cr3 = chars("yellow metal mailbox");

        let mut hits1 = vec![
            Hit::new(10, record(&cr1)),
            Hit::new(20, record(&cr2)),
            Hit::new(30, record(&cr3)),
        ];
        let mut hits2 = hits1.clone();
        let mut hits3 = hits1.clone();

        let cq1 = chars("metall mailbox");
        let cq2 = chars("metall mail");
        let cq3 = chars("met");
        let q1  = query(&cq1);
        let q2  = query(&cq2);
        let q3  = query(&cq3);

        search(&q1, &mut hits1);
        search(&q2, &mut hits2);
        search(&q3, &mut hits3);

        assert_debug_snapshot!(hits1);
        assert_debug_snapshot!(hits2);
        assert_debug_snapshot!(hits3);
    }

    #[test]
    fn search_intersection() {
        let cr1 = chars("brown plush bear");
        let cr2 = chars("metal detector");
        let cr3 = chars("yellow metal mailbox");

        let mut hits1 = vec![
            Hit::new(10, record(&cr1)),
            Hit::new(20, record(&cr2)),
            Hit::new(30, record(&cr3)),
        ];
        let mut hits2 = hits1.clone();

        let cq1 = chars("red wooden mailbox");
        let cq2 = chars("red wooden mail");
        let q1  = query(&cq1);
        let q2  = query(&cq2);

        search(&q1, &mut hits1);
        search(&q2, &mut hits2);

        assert_debug_snapshot!(hits1);
        assert_debug_snapshot!(hits2);
    }

    #[test]
    fn search_min_match() {
        let cr1 = chars("brown plush bear");
        let cr2 = chars("metal detector");
        let cr3 = chars("yellow metal mailbox");

        let mut hits1 = vec![
            Hit::new(10, record(&cr1)),
            Hit::new(20, record(&cr2)),
            Hit::new(30, record(&cr3)),
        ];
        let mut hits2 = hits1.clone();

        let cq1 = chars("wooden mai");
        let cq2 = chars("wooden mail");
        let q1  = query(&cq1);
        let q2  = query(&cq2);

        search(&q1, &mut hits1);
        search(&q2, &mut hits2);

        assert_debug_snapshot!(hits1);
        assert_debug_snapshot!(hits2);
    }

    #[test]
    fn search_transpositions() {
        let cr1 = chars("metal mailbox");
        let cr2 = chars("mailbox metal");

        let mut hits = vec![
            Hit::new(10, record(&cr1)),
            Hit::new(20, record(&cr2)),
        ];

        let cq = chars("mailbox met");
        let q  = query(&cq);

        search(&q, &mut hits);

        assert_debug_snapshot!(hits);
    }
}
