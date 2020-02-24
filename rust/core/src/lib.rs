mod utils;

mod damlev;
mod lexis;
mod search;

use std::cell::RefCell;
use search::{Record, Hit, tokenize_query};


thread_local! {
    static SEPARATORS: RefCell<(Vec<char>, Vec<char>)> = RefCell::new((Vec::new(), Vec::new()));
    static RECORDS:    RefCell<Vec<Record>>            = RefCell::new(Vec::new());
    static HIGHLIGHTS: RefCell<Vec<Vec<char>>>         = RefCell::new(Vec::new());
}


pub fn get_highlights() -> String {
    HIGHLIGHTS.with(|cell| {
        let buffer = &*cell.borrow();
        let mut result = String::new();
        for highlighted in buffer {
            for ch in highlighted {
                result.push(*ch);
            }
            result.push('\0');
        }
        result
    })
}


pub fn highlight_using(l: &str, r: &str) {
    SEPARATORS.with(|cell| {
        let buffer = &mut *cell.borrow_mut();
        buffer.0 = l.chars().collect();
        buffer.1 = r.chars().collect();
    })
}


pub fn set_records(ids: &[usize], sources: String) {
    RECORDS.with(|cell| {
        let records = &mut *cell.borrow_mut();
        records.clear();
        for (source, id) in sources.split('\0').zip(ids.iter()) {
            let source: Vec<char> = source.chars().collect();
            records.push(Record::new(*id, &source));
        }
        if records.len() != ids.len() {
            panic!("Got less texts than expected");
        }
    });
}


pub fn run_search(query: String) -> Vec<usize> {
    RECORDS.with(|cell| {
        let query: Vec<char> = query.chars().collect();
        let query = tokenize_query(&query);

        let records  = &*cell.borrow();
        let mut hits = records.iter()
            .map(|r| r.to_hit())
            .collect();

        search::search(&query, &mut hits);
        save_highlights(&hits);

        hits.iter()
            .map(|h| h.id)
            .collect()
    })
}


fn save_highlights<T: AsRef<[char]>>(hits: &[Hit<T>]) {
    SEPARATORS.with(|cell_sep| {
    HIGHLIGHTS.with(|cell_hl| {
        let sep = &*cell_sep.borrow_mut();
        let hl  = &mut *cell_hl.borrow_mut();
        *hl = search::highlight(&hits, &sep.0, &sep.1);
    }); });
}
