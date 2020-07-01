#![allow(dead_code)]

use super::Lang;
use super::constants::CHAR_CLASSES_LATIN;

pub fn lang_basic() -> Lang {
    let mut lang = Lang::new();
    for &(class, ch) in CHAR_CLASSES_LATIN { lang.add_char_class(ch, class); }
    lang
}
