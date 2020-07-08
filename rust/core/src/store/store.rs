use std::cell::RefCell;
use crate::utils::to_vec;
use crate::lang::Lang;
use super::{Record, TrigramIndex, DEFAULT_LIMIT};


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
