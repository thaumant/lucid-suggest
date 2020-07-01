mod word;
mod text;

use crate::lang::{Lang, CharClass, CharPattern};
pub use word::Word;
pub use text::Text;


pub fn tokenize_query(source: &str, lang: &Option<Lang>) -> Text<Vec<char>> {
    Text::from_str(source)
        .normalize(lang)
        .fin(false)
        .split(&[CharClass::Whitespace, CharClass::Control, CharClass::Punctuation])
        .strip(&[CharClass::NotAlphaNum])
        .lower()
        .mark_pos(lang)
        .stem(lang)
}


pub fn tokenize_record(source: &str, lang: &Option<Lang>) -> Text<Vec<char>> {
     Text::from_str(source)
        .normalize(lang)
        .split(&[CharClass::Whitespace, CharClass::Control, CharClass::Punctuation])
        .strip(&[CharClass::NotAlphaNum])
        .lower()
        .mark_pos(lang)
        .stem(lang)
}
