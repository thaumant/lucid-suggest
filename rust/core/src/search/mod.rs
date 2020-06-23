mod score;
mod filter;
mod sort;
mod highlight;

use std::default::Default;
use crate::utils::LimitSortIterator;
use crate::tokenization::Text;
use crate::matching::WordMatch;
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


const SCORES_SIZE: usize = 9;


pub enum ScoreType {
    SameWords   = 0,
    SamePrimary = 1,
    Typos       = 2,
    Trans       = 3,
    Fin         = 4,
    Offset      = 5,
    Rating      = 6,
    WordLen     = 7,
    CharLen     = 8,
}


#[derive(Debug, Clone)]
pub struct Scores([isize; SCORES_SIZE]);


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
        Scores([0; SCORES_SIZE])
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
    use crate::tokenization::tokenize_query;
    use crate::lang::{Lang, lang_english};
    use crate::store::{Store, Record};
    use super::search;

    fn check(name: &str, lang: Option<Lang>, queries: &[&str]) {
        let mut store = Store::new();
        store.lang = lang;
        store.add(Record::new(10, "brown plush bear",     10, &store.lang));
        store.add(Record::new(20, "the metal detector",   20, &store.lang));
        store.add(Record::new(30, "yellow metal mailbox", 30, &store.lang));
        store.add(Record::new(40, "thesaurus",            40, &store.lang));
        store.add(Record::new(50, "wi-fi router",         50, &store.lang));

        for (i, query) in queries.iter().enumerate() {
            let query   = tokenize_query(query, &store.lang);
            let query   = query.to_ref();
            let results = search(&store, &query).collect::<Vec<_>>();
            assert_debug_snapshot!(format!("{}-{}", name, i), results);
        }
    }

    #[test]
    fn search_empty() {
        check("empty", None, &[""]);
    }

    #[test]
    fn search_equal() {
        check("equal", None, &["yelow metall maiblox"]);
    }

    #[test]
    fn search_partial() {
        check("partial", None, &[
            "brown plush bear",
            "metal detector",
            "yellow metal mailbox",
        ]);
    }

    #[test]
    fn search_intersection() {
        check("intersection", None, &[
            "red wooden mailbox",
            "red wooden mail",
        ]);
    }

    #[test]
    fn search_min_match() {
        check("min_match", None, &[
            "wooden mai",
            "wooden mail",
        ]);
    }

    #[test]
    fn search_transpositions() {
        check("transpositions", None, &[
            "metal mailbox",
            "mailbox metal",
        ]);
    }


    #[test]
    fn search_stemming() {
        let mut store = Store::new();
        store.lang = Some(lang_english());
        store.add(Record::new(30, "universe", 30, &store.lang));

        let query1   = tokenize_query("university", &None);
        let query2   = tokenize_query("university", &store.lang);
        let query1   = query1.to_ref();
        let query2   = query2.to_ref();
        let results1 = search(&store, &query1).collect::<Vec<_>>();
        let results2 = search(&store, &query2).collect::<Vec<_>>();

        assert_debug_snapshot!(results1);
        assert_debug_snapshot!(results2);
    }

    #[test]
    fn search_particles() {
        check("particles_nolang", None, &[
            "the",
        ]);

        check("particles", Some(lang_english()), &[
            "the",
        ]);
    }

    #[test]
    fn search_joined() {
        check("joined_query", None, &[
            "wifi",
        ]);

        check("joined_record", None, &[
            "the saurus",
        ]);
    }
}
