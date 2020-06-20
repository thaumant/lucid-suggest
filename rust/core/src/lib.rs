mod utils;

mod damlev;
mod jaccard;
pub mod lexis;

mod store;
mod search;
pub mod lang;

use std::cell::RefCell;
use std::collections::HashMap;

pub use lexis::{Word, Text, tokenize_query};
pub use store::{Record, Store, DEFAULT_LIMIT};
pub use search::{search, SearchResult};


thread_local! {
    static STORES:  RefCell<HashMap<usize, Store>>             = RefCell::new(HashMap::new());
    static RESULTS: RefCell<HashMap<usize, Vec<SearchResult>>> = RefCell::new(HashMap::new());
}


pub fn create_store(id: usize) {
    STORES.with(|cell| {
        let stores = &mut *cell.borrow_mut();
        if stores.contains_key(&id) {
            panic!("Duplicate store id {}", id);
        }
        let mut store = Store::new();
        store.lang = get_lang();
        stores.insert(id, store);
    });

    RESULTS.with(|cell| {
        let buffers = &mut *cell.borrow_mut();
        if buffers.contains_key(&id) {
            panic!("Duplicate store id {}", id);
        }
        buffers.insert(id, Vec::with_capacity(DEFAULT_LIMIT));
    });
}

#[cfg(lang = "de")] pub fn get_lang() -> Option<lang::Lang> { Some(lang::lang_german()) }
#[cfg(lang = "en")] pub fn get_lang() -> Option<lang::Lang> { Some(lang::lang_english()) }
#[cfg(lang = "es")] pub fn get_lang() -> Option<lang::Lang> { Some(lang::lang_spanish()) }
#[cfg(lang = "pt")] pub fn get_lang() -> Option<lang::Lang> { Some(lang::lang_portuguese()) }
#[cfg(lang = "ru")] pub fn get_lang() -> Option<lang::Lang> { Some(lang::lang_russian()) }
#[cfg(not(any(
    lang = "de",
    lang = "en",
    lang = "es",
    lang = "pt",
    lang = "ru",
)))] pub fn get_lang() -> Option<lang::Lang> { None }


pub fn destroy_store(id: usize) {
    STORES.with(|cell| {
        let stores = &mut *cell.borrow_mut();
        if !stores.contains_key(&id) {
            panic!("Missing store id {}", id);
        }
        stores.remove(&id);
    });

    RESULTS.with(|cell| {
        let buffers = &mut *cell.borrow_mut();
        if !buffers.contains_key(&id) {
            panic!("Missing store id {}", id);
        }
        buffers.remove(&id);
    });
}


pub fn highlight_with(store_id: usize, separators: (&str, &str)) {
    using_store(store_id, |store| {
        store.highlight_with(separators);
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


pub fn set_limit(store_id: usize, limit: usize)  {
    using_store(store_id, |store| {
    using_results(store_id, |buffer| {
        store.limit = limit;
        if limit > buffer.capacity() {
            buffer.reserve_exact(limit - buffer.len());
        }
    }); });
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
