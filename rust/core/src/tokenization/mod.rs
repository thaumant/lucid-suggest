mod pos;
mod pattern;
mod word;
mod text;

use crate::lang::Lang;
pub use pos::PartOfSpeech;
pub use word::Word;
pub use text::Text;
pub use pattern::{Chars, CharPattern};


pub fn tokenize_query(source: &str, lang: &Option<Lang>) -> Text<Vec<char>> {
    Text::from_str(source)
        .normalize(lang)
        .fin(false)
        .split(&[Chars::Whitespaces, Chars::Control, Chars::Punctuation])
        .strip(&[Chars::NotAlphaNum])
        .lower()
        .mark_pos(lang)
        .stem(lang)
}


pub fn tokenize_record(source: &str, lang: &Option<Lang>) -> Text<Vec<char>> {
     Text::from_str(source)
        .normalize(lang)
        .split(&[Chars::Whitespaces, Chars::Control, Chars::Punctuation])
        .strip(&[Chars::NotAlphaNum])
        .lower()
        .mark_pos(lang)
        .stem(lang)
}
