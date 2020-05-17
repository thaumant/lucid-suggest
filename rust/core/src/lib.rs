mod utils;

mod damlev;
mod jaccard;
mod lexis;
mod store;
mod search;
mod lang;

use std::cell::RefCell;
use std::collections::HashMap;
use lang::lang_english;
pub use lexis::{Word, Text, tokenize_query};
pub use store::{Record, Store};
pub use search::{search, SearchResult};


thread_local! {
    static STORES:  RefCell<HashMap<usize, Store>>             = RefCell::new(HashMap::new());
    static RESULTS: RefCell<HashMap<usize, Vec<SearchResult>>> = RefCell::new(HashMap::new());
}


pub fn create_store() -> usize {
    let store_id = STORES.with(|cell| {
        let stores   = &mut *cell.borrow_mut();
        let store    = Store::new();
        let store_id = *stores.keys().max().unwrap_or(&0) + 1;
        stores.insert(store_id, store);
        store_id
    });

    RESULTS.with(|cell| {
        let buffers = &mut *cell.borrow_mut();
        buffers.insert(store_id, Vec::with_capacity(10));
    });

    store_id
}


pub fn highlight_with(store_id: usize, separators: (&str, &str)) {
    using_store(store_id, |store| {
        store.highlight_with(separators);
    });
}


pub fn set_lang(store_id: usize, lang_code: &str) {
    using_store(store_id, |store| {
        let lang = match lang_code {
            "en" => lang_english(),
            _    => panic!("Invalid language: {}", lang_code),
        };
        store.lang = Some(lang);
    });
}


pub fn set_records<'a, I>(store_id: usize, records: I) where I: IntoIterator<Item=(usize, &'a str, usize)> {
    using_store(store_id, |store| {
        store.clear();
        for (id, title, rating) in records {
            store.add(Record::new(id, title, rating, &store.lang));
        }
    });
}


pub fn run_search(store_id: usize, query: &str) {
    using_store(store_id, |store| {
    using_results(store_id, |buffer| {
        let query = tokenize_query(query, &None);
        let query = query.to_ref();
        buffer.clear();
        for result in search(&store, &query) {
            buffer.push(result);
        }
    }); });
}


pub fn using_store<T, F>(store_id: usize, f: F) -> T where F: (FnOnce(&mut Store) -> T) {
    STORES.with(|cell| {
        let stores = &mut *cell.borrow_mut();
        let store  = stores.get_mut(&store_id).unwrap();
        f(store)
    })
}


pub fn using_results<T, F>(store_id: usize, f: F) -> T where F: (FnOnce(&mut Vec<SearchResult>) -> T) {
    RESULTS.with(|cell| {
        let buffers = &mut *cell.borrow_mut();
        let buffer  = buffers.get_mut(&store_id).unwrap();
        f(buffer)
    })
}
