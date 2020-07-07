mod trigram_index;

use std::cell::RefCell;
use crate::utils::to_vec;
use crate::tokenization::{TextOwn, tokenize_record};
use crate::lang::Lang;
use trigram_index::TrigramIndex;


pub static DEFAULT_LIMIT: usize = 10;


#[derive(Debug)]
pub struct Record {
    pub ix:     usize,
    pub id:     usize,
    pub title:  TextOwn,
    pub rating: usize,
}


impl Record {
    pub fn new(id: usize, source: &str, rating: usize, lang: &Lang) -> Record {
        Record {
            ix: 0,
            id,
            title: tokenize_record(source, lang),
            rating,
        }
    }
}


pub struct Store {
    pub next_ix:  usize,
    pub records:  Vec<Record>,
    pub limit:    usize,
    pub lang:     Lang,
    pub dividers: (Vec<char>, Vec<char>),
    pub index:    RefCell<TrigramIndex>,
    pub top_ixs:  RefCell<Option<Vec<usize>>>,
}


impl Store {
    pub fn new() -> Self {
        Self {
            next_ix:  0,
            records:  Vec::new(),
            limit:    DEFAULT_LIMIT,
            lang:     Lang::new(),
            dividers: (vec!['['], vec![']']),
            index:    RefCell::new(TrigramIndex::new()),
            top_ixs:  RefCell::new(None),
        }
    }

    pub fn add(&mut self, mut record: Record) {
        let Self { next_ix, index, records, .. } = self;
        debug_assert!(*next_ix == records.len(), "Invalid store.next_ix");
        record.ix = *next_ix;
        index.borrow_mut().add(&record);
        records.push(record);
        *next_ix += 1;
    }

    pub fn clear(&mut self) {
        self.records.clear();
        self.next_ix = 0;
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
