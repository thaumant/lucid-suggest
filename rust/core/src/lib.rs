mod utils;

mod matching;
pub mod tokenization;

mod store;
mod search;
pub mod lang;

use std::cell::RefCell;
use fnv::{FnvHashMap as HashMap};

pub use tokenization::{
    Word,
    WordShape,
    WordView,
    Text,
    TextOwn,
    TextRef,
    tokenize_query,
    tokenize_record,
};
pub use store::{Record, Store, DEFAULT_LIMIT};
pub use search::{SearchResult};


thread_local! {
    static STORES:  RefCell<HashMap<usize, Store>>             = RefCell::new(HashMap::default());
    static RESULTS: RefCell<HashMap<usize, Vec<SearchResult>>> = RefCell::new(HashMap::default());
}


pub fn create_store() -> usize {
    let id = STORES.with(|cell| {
        let stores = &mut *cell.borrow_mut();
        let id = stores.keys().max().unwrap_or(&0) + 1;
        let store = Store::new();
        stores.insert(id, store);
        id
    });
    RESULTS.with(|cell| {
        let buffers = &mut *cell.borrow_mut();
        buffers.insert(id, Vec::with_capacity(DEFAULT_LIMIT));
    });
    id
}



pub fn destroy_store(id: usize) {
    STORES.with(|cell| {
        let stores = &mut *cell.borrow_mut();
        if !stores.contains_key(&id) {
            panic!("Missing store id");
        }
        stores.remove(&id);
    });

    RESULTS.with(|cell| {
        let buffers = &mut *cell.borrow_mut();
        if !buffers.contains_key(&id) {
            panic!("Missing store id");
        }
        buffers.remove(&id);
    });
}


pub fn highlight_with(store_id: usize, separators: (&str, &str)) {
    using_store(store_id, |store| {
        store.highlight_with(separators);
    });
}


pub fn add_record(store_id: usize, record: Record) {
    using_store(store_id, |store| {
        store.add(record);
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


pub fn run_search(store_id: usize, query: &TextRef) -> usize {
    using_store(store_id, |store| {
    using_results(store_id, |buffer| {
        buffer.clear();
        let hits = store.search(&query);
        let len  = hits.len();
        for result in hits {
            buffer.push(result);
        }
        len
    }) })
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
