mod utils;

mod damlev;
mod lexis;
mod search;

use std::cell::RefCell;
use std::collections::HashMap;
pub use lexis::{Word, Text};
pub use search::{Record, SearchResult, Store};


thread_local! {
    static STORES: RefCell<HashMap<usize, Store>> = RefCell::new(HashMap::new());
}


pub fn create_store() -> usize {
    STORES.with(|cell| {
        let stores   = &mut *cell.borrow_mut();
        let store    = Store::new();
        let store_id = *stores.keys().max().unwrap_or(&0) + 1;
        stores.insert(store_id, store);
        store_id
    })
}


pub fn highlight_using(store_id: usize, separators: (&str, &str)) {
    using_store(store_id, |store| {
        store.highlight_using(separators);
    });
}


pub fn add_records<'a, I>(store_id: usize, records: I) where I: IntoIterator<Item=(usize, &'a str)> {
    using_store(store_id, |store| {
        for (id, text) in records {
            store.add(Record::new(id, text));
        }
    });
}


pub fn search(store_id: usize, query: &str) {
    using_store(store_id, |store| {
        store.search(query);
    });
}


pub fn using_store<T, F>(store_id: usize, f: F) -> T where F: (FnOnce(&mut Store) -> T) {
    STORES.with(|cell| {
        let stores = &mut *cell.borrow_mut();
        let store  = stores.get_mut(&store_id).unwrap();
        f(store)
    })
}


pub fn using_results<T, F>(store_id: usize, f: F) -> T where F: (FnOnce(&[SearchResult]) -> T) {
    STORES.with(|cell| {
        let stores = &mut *cell.borrow_mut();
        let store  = stores.get_mut(&store_id).unwrap();
        f(store.results())
    })
}
