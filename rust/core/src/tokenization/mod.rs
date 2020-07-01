mod word;
mod word_shape;
mod word_split;
mod word_view;
mod text;

use crate::lang::{Lang, CharClass};
pub use word::Word;
pub use word_view::WordView;
pub use word_shape::WordShape;
pub use text::{Text, TextOwn, TextRef};


pub fn tokenize_query(source: &str, lang: &Lang) -> TextOwn {
    Text::from_str(source)
        .normalize(lang)
        .fin(false)
        .split(&[CharClass::Whitespace, CharClass::Control, CharClass::Punctuation], lang)
        .strip(&[CharClass::NotAlphaNum], lang)
        .lower()
        .set_pos(lang)
        .set_char_classes(lang)
        .set_stem(lang)
}


pub fn tokenize_record(source: &str, lang: &Lang) -> TextOwn {
     Text::from_str(source)
        .normalize(lang)
        .split(&[CharClass::Whitespace, CharClass::Control, CharClass::Punctuation], lang)
        .strip(&[CharClass::NotAlphaNum], lang)
        .lower()
        .set_pos(lang)
        .set_char_classes(lang)
        .set_stem(lang)
}
