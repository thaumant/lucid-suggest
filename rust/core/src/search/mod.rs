mod fullscan;
mod scorer;
mod filter;
mod limitsort;
mod highlighter;

use std::default::Default;
use crate::lexis::{Text, WordMatch};
use crate::store::{Store, Record};

pub use fullscan::FullScan;
pub use scorer::Scorer;
pub use filter::Filter;
pub use limitsort::LimitSort;
pub use highlighter::Highlighter;


#[derive(Debug)]
pub struct Hit<'a> {
    pub id:      usize,
    pub text:    Text<&'a [char]>,
    pub scores:  Scores,
}


impl<'a> Hit<'a> {
    pub fn from_record(record: &'a Record) -> Hit<'a> {
        Hit {
            id:     record.id,
            text:   record.text.to_ref(),
            scores: Default::default(),
        }
    }
}


pub enum ScoreType {
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


impl std::ops::Index<ScoreType> for Scores {
    type Output = isize;

    fn index(&self, score: ScoreType) -> &Self::Output {
        &self.linear[score as usize]
    }
}


impl std::ops::IndexMut<ScoreType> for Scores {
    fn index_mut(&mut self, score: ScoreType) -> &mut Self::Output {
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


#[derive(Debug)]
pub struct SearchResult {
    pub id:          usize,
    pub highlighted: String,
}


pub fn search<'a>(
    store: &'a Store,
    query: &'a Text<&'a [char]>,
) -> Highlighter<'a, impl Iterator<Item=Hit<'a>>> {
    let fullscan    = FullScan::new(store.records.iter());
    let scorer      = Scorer::new(fullscan, query);
    let filter      = Filter::new(scorer, query);
    let limitsort   = LimitSort::new(filter, store.limit);
    let highlighter = Highlighter::new(limitsort, store.separators());
    highlighter
}


#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use crate::lexis::tokenize_query;
    use crate::store::{Store, Record};
    use super::search;

    fn check(name: &str, queries: &[&str]) {
        let mut store = Store::new();
        store.add(Record::new(10, "brown plush bear"));
        store.add(Record::new(20, "metal detector"));
        store.add(Record::new(30, "yellow metal mailbox"));

        for (i, query) in queries.iter().enumerate() {
            let query   = tokenize_query(query);
            let query   = query.to_ref();
            let results = search(&store, &query).collect::<Vec<_>>();
            assert_debug_snapshot!(format!("{}-{}", name, i), results);
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
