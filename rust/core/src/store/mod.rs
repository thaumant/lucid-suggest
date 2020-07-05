mod trigrams;
mod trigram_index;

use fnv::{FnvHashMap as HashMap};
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
    pub fn new(id: usize, source: &str, rating: usize, lang: &Lang) -> Record {
        Record {
            id,
            title: tokenize_record(source, lang),
            rating,
        }
    }
}


pub struct Store {
    pub records:  HashMap<usize, Record>,
    pub limit:    usize,
    pub lang:     Lang,
    pub dividers: (Vec<char>, Vec<char>),
    pub index:    RefCell<TrigramIndex>,
    pub top_ids:  RefCell<Option<Vec<usize>>>,
}


impl Store {
    pub fn new() -> Self {
        Self {
            records:  HashMap::default(),
            limit:    DEFAULT_LIMIT,
            lang:     Lang::new(),
            dividers: (vec!['['], vec![']']),
            index:    RefCell::new(TrigramIndex::new()),
            top_ids:  RefCell::new(None),
        }
    }

    pub fn add(&mut self, record: Record) {
        self.index.borrow_mut().add(&record);
        self.records.insert(record.id, record);
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
