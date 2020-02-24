use wasm_bindgen::prelude::*;
use lucid_suggest_core as core;


#[wasm_bindgen]
pub fn get_highlights() -> String {
    core::get_highlights()
}


#[wasm_bindgen]
pub fn highlight_using(l: &str, r: &str) {
    core::highlight_using(l, r)
}


#[wasm_bindgen]
pub fn set_records(ids: &[usize], sources: String) {
    core::set_records(ids, sources)
}


#[wasm_bindgen]
pub fn run_search(query: String) -> Vec<usize> {
    core::run_search(query)
}
