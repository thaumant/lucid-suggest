use wasm_bindgen::prelude::*;
use lucid_suggest_core as core;


#[wasm_bindgen]
pub fn create_store(id: usize) {
    core::create_store(id, get_lang());
}


#[cfg(lang = "de")] pub fn get_lang() -> core::Lang { core::lang_german() }
#[cfg(lang = "en")] pub fn get_lang() -> core::Lang { core::lang_english() }
#[cfg(lang = "es")] pub fn get_lang() -> core::Lang { core::lang_spanish() }
#[cfg(lang = "fr")] pub fn get_lang() -> core::Lang { core::lang_french() }
#[cfg(lang = "pt")] pub fn get_lang() -> core::Lang { core::lang_portuguese() }
#[cfg(lang = "ru")] pub fn get_lang() -> core::Lang { core::lang_russian() }
#[cfg(not(any(
    lang = "de",
    lang = "en",
    lang = "es",
    lang = "fr",
    lang = "pt",
    lang = "ru",
)))] pub fn get_lang() -> core::Lang { core::Lang::new() }


#[wasm_bindgen]
pub fn destroy_store(id: usize) {
    core::destroy_store(id);
}


#[wasm_bindgen]
pub fn highlight_with(store_id: usize, left: &str, right: &str) {
    core::highlight_with(store_id, (left, right));
}


#[wasm_bindgen]
pub fn set_limit(id: usize, limit: usize)  {
    core::set_limit(id, limit);
}


#[wasm_bindgen]
pub fn add_record(store_id: usize, record_id: usize, title: &str, rating: usize) {
    core::add_record(store_id, record_id, title, rating);
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
