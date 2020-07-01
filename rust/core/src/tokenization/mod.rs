mod word;
mod text;

use crate::lang::{Lang, Chars, CharPattern};
pub use word::Word;
pub use text::Text;


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
