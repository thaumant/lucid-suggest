mod pos;
mod pattern;
mod word;
mod text;
mod matching;

use crate::lang::Lang;
pub use pos::PartOfSpeech;
pub use word::Word;
pub use text::Text;
pub use matching::{WordMatch, MatchSide, word_match, text_match};
pub use pattern::{Chars, CharPattern};


pub fn tokenize_query(source: &str, lang: &Option<Lang>) -> Text<Vec<char>> {
    let mut text = Text::from_str(source)
        .fin(false)
        .split(&[Chars::Whitespaces, Chars::Control, Chars::Punctuation])
        .strip(&[Chars::NotAlphaNum])
        .lower();

    if let Some(lang) = lang {
        text = text
            .mark_pos(lang)
            .stem(lang);
    }

    text
}


pub fn tokenize_record(source: &str, lang: &Option<Lang>) -> Text<Vec<char>> {
    let mut text = Text::from_str(source)
        .split(&[Chars::Whitespaces, Chars::Control, Chars::Punctuation])
        .strip(&[Chars::NotAlphaNum])
        .lower();

    if let Some(lang) = lang {
        text = text
            .mark_pos(lang)
            .stem(lang);
    }

    text
}
