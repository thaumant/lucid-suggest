#![allow(dead_code)]

use rust_stemmers::{Algorithm, Stemmer};
use super::Lang;
use super::constants::CHAR_CLASSES_LATIN;

pub fn lang_empty() -> Lang {
    let mut lang = Lang::new();

    lang.set_stemmer(Some(Stemmer::create(Algorithm::English)));
    for &(class, ch) in CHAR_CLASSES_LATIN { lang.add_char_class(ch, class); }

    lang
}
