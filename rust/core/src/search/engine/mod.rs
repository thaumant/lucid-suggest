mod score;

use std::cmp::Ordering;
use crate::lexis::Text;
use super::{Record, Hit};
use score::score;
pub use score::Scores;


pub struct Engine<'a> {
    query: Text<&'a [char]>,
    hits:  Vec<Hit<'a>>,
    limit: usize,
}


impl<'a> Engine<'a> {
    pub fn new(query: Text<&'a [char]>, limit: usize) -> Self {
        Self {
            query,
            hits: Vec::with_capacity(limit * 2),
            limit,
        }
    }

    pub fn push(&mut self, record: &'a Record) {
        let mut hit = Hit::from_record(record);
        score(&self.query, &mut hit);
        if self.matches(&hit) {
            self.hits.push(hit);
        }
        if self.hits.len() >= self.limit * 2 {
            self.sort_and_truncate();
        }
    }

    pub fn push_many<I: IntoIterator<Item=&'a Record>>(&mut self, records: I) {
        for record in records {
            self.push(record);
        }
    }

    pub fn matches(&self, hit: &Hit<'a>) -> bool {
        let matches = &hit.scores.matches;
        if self.query.is_empty() { return true; }
        if matches.len() == 0 { return false; }
        if matches.len() == 1 && self.query.words.len() > 1 {
            let word_match = &matches[0];
            let unfinished = !word_match.fin;
            let first_half = (word_match.query.len * 2) < word_match.record.len;
            if unfinished && first_half { return false; }
        }
        true
    }

    pub fn sort_and_truncate(&mut self) {
        self.hits.sort_by(|hit1, hit2| {
            hit1.scores.linear.iter()
                .zip(hit2.scores.linear.iter())
                .map(|(s1, s2)| s2.cmp(s1))
                .find(|&ord| ord != Ordering::Equal)
                .unwrap_or(Ordering::Equal)
        });
        self.hits.truncate(self.limit);
    }

    pub fn hits<'b>(&'b self) -> &'b [Hit<'a>] {
        &self.hits
    }
}


#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use crate::lexis::tokenize_query;
    use super::{Record, Engine};

    fn check(name: &str, queries: &[&str]) {
        let records = [
            Record::new(10, "brown plush bear"),
            Record::new(20, "metal detector"),
            Record::new(30, "yellow metal mailbox"),
        ];
        for (i, query) in queries.iter().enumerate() {
            let query = tokenize_query(query);
            let mut finder = Engine::new(query.to_ref(), 10);
            finder.push_many(&records);
            finder.sort_and_truncate();
            assert_debug_snapshot!(format!("{}-{}", name, i), finder.hits);
        }
    }

    #[test]
    fn search_empty() {
        check("empty", &[""]);
    }

    #[test]
    fn search_equal() {
        check("equal", &["yelow metall maiblox"]);
    }

    #[test]
    fn search_partial() {
        check("partial", &[
            "brown plush bear",
            "metal detector",
            "yellow metal mailbox",
        ]);
    }

    #[test]
    fn search_intersection() {
        check("intersection", &[
            "red wooden mailbox",
            "red wooden mail",
        ]);
    }

    #[test]
    fn search_min_match() {
        check("min_match", &[
            "wooden mai",
            "wooden mail",
        ]);
    }

    #[test]
    fn search_transpositions() {
        check("transpositions", &[
            "metal mailbox",
            "mailbox metal",
        ]);
    }
}
