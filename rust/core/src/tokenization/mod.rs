mod word;
mod text;

use crate::lang::{Lang, CharClass};
pub use word::Word;
pub use text::{Text, TextOwn, TextRef};


pub fn tokenize_query(source: &str, lang: &Option<Lang>) -> TextOwn {
    Text::from_str(source)
        .normalize(lang)
        .fin(false)
        .split(&[CharClass::Whitespace, CharClass::Control, CharClass::Punctuation], lang)
        .strip(&[CharClass::NotAlphaNum], lang)
        .lower()
        .mark_pos(lang)
        .stem(lang)
}


pub fn tokenize_record(source: &str, lang: &Option<Lang>) -> TextOwn {
     Text::from_str(source)
        .normalize(lang)
        .split(&[CharClass::Whitespace, CharClass::Control, CharClass::Punctuation], lang)
        .strip(&[CharClass::NotAlphaNum], lang)
        .lower()
        .mark_pos(lang)
        .stem(lang)
}
