use wasm_bindgen::prelude::*;
use lucid_suggest_core as core;


#[wasm_bindgen]
pub fn create_store(id: usize) {
    core::create_store(id);
}


#[wasm_bindgen]
pub fn destroy_store(id: usize) {
    core::destroy_store(id);
}


#[wasm_bindgen]
pub fn set_lang(id: usize, lang: &str) {
    core::set_lang(id, lang);
}


#[wasm_bindgen]
pub fn highlight_with(store_id: usize, left: &str, right: &str) {
    core::highlight_with(store_id, (left, right));
}


#[wasm_bindgen]
pub fn set_records(store_id: usize, ids: &[usize], texts: String, ratings: &[usize]) {
    let records = ids.iter()
        .cloned()
        .zip(texts.split('\0'))
        .zip(ratings)
        .map(|((id, text), &rating)| (id, text, rating));
    core::set_records(store_id, records)
}


#[wasm_bindgen]
pub fn run_search(store_id: usize, query: &str) {
    core::run_search(store_id, query)
}


#[wasm_bindgen]
pub fn get_result_ids(store_id: usize) -> Vec<usize> {
    core::using_results(store_id, |results| {
        results.iter().map(|r| r.id).collect()
    })
}


#[wasm_bindgen]
pub fn get_result_titles(store_id: usize) -> String {
    core::using_results(store_id, |results| {
        let bytelen: usize = results.iter()
            .map(|result| result.title.len())
            .sum();
        let mut concat = String::with_capacity(bytelen + results.len());
        for result in results {
            concat.push_str(&result.title);
            concat.push('\0');
        }
        concat
    })
}
