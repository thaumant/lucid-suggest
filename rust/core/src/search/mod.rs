mod score;
mod filter;
mod sort;
mod highlight;

use std::default::Default;
use crate::utils::LimitSortIterator;
use crate::lexis::{Text, WordMatch};
use crate::store::{Store, Record};


#[derive(Debug)]
pub struct Hit<'a> {
    pub id:      usize,
    pub title:   Text<&'a [char]>,
    pub rating:  usize,
    pub matches: Vec<WordMatch>,
    pub scores:  Scores,
}


impl<'a> Hit<'a> {
    pub fn from_record(record: &'a Record) -> Hit<'a> {
        Hit {
            id:      record.id,
            title:   record.title.to_ref(),
            rating:  record.rating,
            scores:  Default::default(),
            matches: Vec::new(),
        }
    }
}


pub enum ScoreType {
    SameWords = 0,
    Typos     = 1,
    Trans     = 2,
    Fin       = 3,
    Offset    = 4,
    Rating    = 5,
    WordLen   = 6,
    CharLen   = 7,
}


#[derive(Debug, Clone)]
pub struct Scores([isize; 8]);


impl Scores {
    pub fn iter(&self) -> impl Iterator<Item=&isize> {
        self.0.iter()
    }
}


impl std::ops::Index<ScoreType> for Scores {
    type Output = isize;

    fn index(&self, score: ScoreType) -> &Self::Output {
        &self.0[score as usize]
    }
}


impl std::ops::IndexMut<ScoreType> for Scores {
    fn index_mut(&mut self, score: ScoreType) -> &mut Self::Output {
        &mut self.0[score as usize]
    }
}


impl Default for Scores {
    fn default() -> Scores {
        Scores([0; 8])
    }
}


#[derive(Debug)]
pub struct SearchResult {
    pub id:    usize,
    pub title: String,
}


pub fn search<'a>(
    store:   &'a Store,
    query:   &'a Text<&'a [char]>,
) -> std::iter::Map<impl Iterator<Item=Hit<'a>>, impl (FnMut(Hit<'a>) -> SearchResult)> {
    let dividers = store.dividers();
    store.records.iter()
        .map(|record| {
            Hit::from_record(record)
        })
        .map(move |mut hit| {
            score::score(query, &mut hit);
            hit
        })
        .filter(move |hit| {
            filter::hit_matches(query, hit)
        })
        .limit_sort(store.limit, sort::compare_hits)
        .map(move |hit| {
            SearchResult {
                id:    hit.id,
                title: highlight::highlight(&hit, dividers),
            }
        })
}


#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use crate::lexis::tokenize_query;
    use crate::store::{Store, Record};
    use super::search;

    fn check(name: &str, queries: &[&str]) {
        let mut store = Store::new();
        store.add(Record::new(10, "brown plush bear", 10));
        store.add(Record::new(20, "metal detector", 20));
        store.add(Record::new(30, "yellow metal mailbox", 30));

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
