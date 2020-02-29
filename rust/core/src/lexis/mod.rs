mod word;
mod text;
mod pattern;
mod matching;

pub use word::Word;
pub use text::Text;
pub use matching::{WordMatch, MatchSide, word_match, text_match};
pub use pattern::{Chars, CharPattern};


pub fn tokenize_query(source: &str) -> Text<Vec<char>> {
    Text::from_str(source)
        .fin(false)
        .split(&[Chars::Whitespaces, Chars::Control, Chars::Punctuation])
        .strip(&[Chars::NotAlphaNum])
        .lower()
}


pub fn tokenize_record(source: &str) -> Text<Vec<char>> {
    Text::from_str(source)
        .split(&[Chars::Whitespaces, Chars::Control])
        .strip(&[Chars::NotAlphaNum])
        .lower()
}
