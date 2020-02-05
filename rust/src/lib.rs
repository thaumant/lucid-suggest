mod utils;

mod damlev;
mod lexis;
mod search;

use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use lexis::{Text, Chars};
use search::Hit;


// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }

// macro_rules! console_log {
//     ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
// }


#[derive(Clone, Debug)]
pub struct Record {
    id: usize,
    source: Vec<char>,
}


thread_local! {
    static SEPARATORS: RefCell<(Vec<char>, Vec<char>)> = RefCell::new((Vec::new(), Vec::new()));
    static RECORDS:    RefCell<Vec<Record>>            = RefCell::new(Vec::new());
    static HIGHLIGHTS: RefCell<Vec<Vec<char>>>         = RefCell::new(Vec::new());
}


#[wasm_bindgen]
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


#[wasm_bindgen]
pub fn highlight_using(l: &str, r: &str) {
    SEPARATORS.with(|cell| {
        let buffer = &mut *cell.borrow_mut();
        buffer.0 = l.chars().collect();
        buffer.1 = r.chars().collect();
    })
}


#[wasm_bindgen]
pub fn set_records(ids: &[usize], sources: String) {
    RECORDS.with(|cell| {
        let records = &mut *cell.borrow_mut();
        records.clear();
        for (source, id) in sources.split('\0').zip(ids.iter()) {
            records.push(Record {
                id: *id,
                source: source.chars().collect(),
            });
        }
        if records.len() != ids.len() {
            panic!("Got less texts than expected");
        }
    });
}


#[wasm_bindgen]
pub fn run_search(query: String) -> Vec<usize> {
    RECORDS.with(|cell| {
        let query: Vec<char> = query.chars().collect();
        let query: Text = tokenize_query(&query);

        let records  = &*cell.borrow();
        let mut hits = records.iter()
            .map(|r| Hit::new(r.id, tokenize_record(&r.source)))
            .collect();

        search::search(&query, &mut hits);
        save_highlights(&hits);

        hits.iter()
            .map(|h| h.id)
            .collect()
    })
}


fn save_highlights(hits: &[Hit]) {
    SEPARATORS.with(|cell_sep| {
    HIGHLIGHTS.with(|cell_hl| {
        let sep = &*cell_sep.borrow_mut();
        let hl  = &mut *cell_hl.borrow_mut();
        *hl = search::highlight(&hits, &sep.0, &sep.1);
    }); });
}


fn tokenize_record<'a>(source: &'a [char]) -> Text<'a> {
    Text::new(source)
        .split(&[Chars::Whitespaces, Chars::Control])
        .strip(&[Chars::NotAlphaNum])
        .lower()
}


fn tokenize_query<'a>(source: &'a [char]) -> Text<'a> {
    Text::new(source)
        .fin(false)
        .split(&[Chars::Whitespaces, Chars::Control, Chars::Punctuation])
        .strip(&[Chars::NotAlphaNum])
        .lower()
}
