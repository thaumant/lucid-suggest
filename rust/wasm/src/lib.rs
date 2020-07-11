use wasm_bindgen::prelude::*;
use lucid_suggest_core as core;
use lucid_suggest_core::{WordShape, TextOwn, Record};
use lucid_suggest_core::lang::{CharClass, PartOfSpeech};


#[wasm_bindgen]
pub fn create_store() -> usize {
    let id = core::create_store();
    core::highlight_with(id, ("{{", "}}"));
    id
}


#[wasm_bindgen]
pub fn destroy_store(id: usize) {
    core::destroy_store(id);
}


#[wasm_bindgen]
pub fn set_limit(store_id: usize, limit: usize)  {
    core::set_limit(store_id, limit);
}


#[wasm_bindgen]
pub fn add_record(
    store_id:  usize,
    record_id: usize,
    rating:    usize,
    source:    &str,
    chars:     &str,
    classes:   &[usize],
    words:     &[usize],
) {
    let record = record_from_raw(record_id, rating, source, chars, classes, words);
    core::add_record(store_id, record)
}


#[wasm_bindgen]
pub fn run_search(
    store_id:  usize,
    source:    &str,
    chars:     &str,
    classes:   &[usize],
    words:     &[usize],
) -> usize {
    let query = text_from_raw(source, chars, classes, words);
    core::run_search(store_id, &query.to_ref())
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
        concat.pop();
        concat
    })
}


fn record_from_raw(
    id:        usize,
    rating:    usize,
    source:    &str,
    chars:     &str,
    classes:   &[usize],
    words:     &[usize],
) -> Record {
    Record {
        ix: 0,
        id,
        rating,
        title: text_from_raw(source, chars, classes, words),
    }
}


fn text_from_raw(
    source:    &str,
    chars:     &str,
    classes:   &[usize],
    words:     &[usize],
) -> TextOwn {
    if words.len() % 6 != 0 { panic!("Malformed raw words"); }
    TextOwn {
        source:  source.chars().collect(),
        chars:   chars.chars().collect(),
        classes: classes.iter().map(charclass_from_raw).collect(),
        words:   words.chunks_exact(6).map(word_from_raw).collect(),
    }
}


fn charclass_from_raw(raw: &usize) -> CharClass {
    match raw {
        0 => CharClass::Any,
        1 => CharClass::Control,
        2 => CharClass::Whitespace,
        3 => CharClass::Punctuation,
        4 => CharClass::NotAlpha,
        5 => CharClass::NotAlphaNum,
        6 => CharClass::Consonant,
        7 => CharClass::Vowel,
        _ => panic!("Invalid char class"),
    }
}


fn word_from_raw(raw: &[usize]) -> WordShape {
    WordShape {
        offset: raw[0],
        slice:  (raw[1], raw[2]),
        stem:   raw[3],
        pos:    pos_from_raw(raw[4]),
        fin:    raw[5] > 0,
    }
}


fn pos_from_raw(raw: usize) -> Option<PartOfSpeech> {
    match raw {
        0  => None,
        1  => Some(PartOfSpeech::Noun),
        2  => Some(PartOfSpeech::Pronoun),
        3  => Some(PartOfSpeech::Verb),
        4  => Some(PartOfSpeech::Adjective),
        5  => Some(PartOfSpeech::Adverb),
        6  => Some(PartOfSpeech::Preposition),
        7  => Some(PartOfSpeech::Conjunction),
        8  => Some(PartOfSpeech::Particle),
        9  => Some(PartOfSpeech::Intejection),
        10 => Some(PartOfSpeech::Article),
        _  => panic!("Invalid part of speech"),
    }
}
