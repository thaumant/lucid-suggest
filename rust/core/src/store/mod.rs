mod trigrams;
mod trigram_index;

use std::cell::RefCell;
use crate::utils::to_vec;
use crate::tokenization::{TextOwn, tokenize_record};
use crate::lang::Lang;
use trigram_index::TrigramIndex;


pub static DEFAULT_LIMIT: usize = 10;


#[derive(Debug)]
pub struct Record {
    pub id:     usize,
    pub title:  TextOwn,
    pub rating: usize,
}


impl Record {
    pub fn new(id: usize, source: &str, rating: usize, lang: &Option<Lang>) -> Record {
        Record {
            id,
            title: tokenize_record(source, lang),
            rating,
        }
    }
}


pub struct Store {
    pub records: Vec<Record>,
    pub limit:   usize,
    pub lang:    Option<Lang>,
    dividers:    (Vec<char>, Vec<char>),
    pub index:   RefCell<TrigramIndex>,
}


impl Store {
    pub fn new() -> Self {
        Self {
            records:  Vec::new(),
            limit:    DEFAULT_LIMIT,
            lang:     None,
            dividers: (vec!['['], vec![']']),
            index:    RefCell::new(TrigramIndex::new()),
        }
    }

    pub fn add(&mut self, record: Record) {
        self.index.borrow_mut().add(&record);
        self.records.push(record);
    }

    pub fn clear(&mut self) {
        self.records.clear();
    }

    pub fn highlight_with(&mut self, dividers: (&str, &str)) {
        let left:  Vec<char> = to_vec(dividers.0);
        let right: Vec<char> = to_vec(dividers.1);
        self.dividers = (left, right);
    }

    pub fn dividers<'a>(&'a self) -> (&'a [char], &'a [char]) {
        (&self.dividers.0, &self.dividers.1)
    }
}
