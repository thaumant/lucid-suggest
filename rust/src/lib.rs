mod utils;

mod damlev;
mod lexis;
mod search;

use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use lexis::{Text, Chars};
use search::Hit;
use std::borrow::Cow;


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
    text: Text<Vec<char>>,
}


impl Record {
    pub fn new(id: usize, source: &[char]) -> Record {
        Record {
            id,
            text: tokenize_record(source).to_owned(),
        }
    }

    pub fn to_hit<'a>(&'a self) -> Hit<Cow<'a, [char]>> {
        Hit {
            id: self.id,
            text: self.text.to_cow(),
            scores: Default::default(),
        }
    }
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
            let source: Vec<char> = source.chars().collect();
            records.push(Record::new(*id, &source));
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


fn tokenize_record<'a>(source: &'a [char]) -> Text<Cow<'a, [char]>> {
    Text::new_cow(Cow::Borrowed(source))
        .split(&[Chars::Whitespaces, Chars::Control])
        .strip(&[Chars::NotAlphaNum])
        .lower()
}


fn tokenize_query<'a>(source: &'a [char]) -> Text<Cow<'a, [char]>> {
    Text::new_cow(Cow::Borrowed(source))
        .fin(false)
        .split(&[Chars::Whitespaces, Chars::Control, Chars::Punctuation])
        .strip(&[Chars::NotAlphaNum])
        .lower()
}
