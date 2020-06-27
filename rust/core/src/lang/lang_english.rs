#![allow(dead_code)]

use rust_stemmers::{Algorithm, Stemmer};
use crate::tokenization::PartOfSpeech;
use super::Lang;


const ARTICLES: &[&'static str] = &[
    "a",
    "an",
    "the",
];

const PREPOSITIONS: &[&'static str] = &[
    "at",
    "by",
    "for",
    "from",
    "in",
    "of",
    "on",
    "to",
    "since",
    "before",
    "till",
    "untill",
    "beside",
    "under",
    "below",
    "over",
    "above",
    "across",
    "through",
    "into",
    "towards",
    "onto",
    "off",
    "out",
    "about",
];

const CONJUNCTIONS: &[&'static str] = &[
    "after",
    "although",
    "as",
    "because",
    "before",
    "but",
    "either",
    "for",
    "how",
    "if",
    "lest",
    "nor",
    "once",
    "once",
    "or",
    "since",
    "since",
    "since",
    "so",
    "than",
    "that",
    "that",
    "though",
    "till",
    "unless",
    "until",
    "until",
    "until",
    "what",
    "whatever",
    "when",
    "when",
    "whenever",
    "where",
    "whereas",
    "whereas",
    "wherever",
    "whether",
    "which",
    "whichever",
    "while",
    "while",
    "while",
    "whilst",
    "who",
    "whoever",
    "whom",
    "whomever",
    "whose",
    "why",
    "yet",
    "and",
    // "as if",
    // "as long as",
    // "as much as",
    // "as soon as",
    // "as though",
    // "assuming that",
    // "by the time",
    // "even if",
    // "even though",
    // "in case that",
    // "in case",
    // "in order that",
    // "in order",
    // "now that",
    // "only if",
    // "provided that",
    // "rather than",
    // "so that",
];

const PARTICLES: &[&'static str] = &[
    "by",
    "in",
    "not",
    "on",
    "to",
    "oh",
];

const UTF_COMPOSE_MAP: &[(&'static str, &'static str)] = &[];

const UTF_REDUCE_MAP: &[(&'static str, &'static str)] = &[];


pub fn lang_english() -> Lang {
    let mut lang = Lang::new();

    lang.set_stemmer(Some(Stemmer::create(Algorithm::English)));

    for (from, to) in UTF_COMPOSE_MAP { lang.add_unicode_composition(from, to); }
    for (from, to) in UTF_REDUCE_MAP  { lang.add_unicode_reduction(from, to); }

    for word in ARTICLES     { lang.add_pos(word, PartOfSpeech::Article); }
    for word in PREPOSITIONS { lang.add_pos(word, PartOfSpeech::Preposition); }
    for word in CONJUNCTIONS { lang.add_pos(word, PartOfSpeech::Conjunction); }
    for word in PARTICLES    { lang.add_pos(word, PartOfSpeech::Particle); }

    lang
}


#[cfg(test)]
mod tests {
    use crate::utils::to_vec;
    use crate::tokenization::PartOfSpeech;
    use super::{lang_english, UTF_COMPOSE_MAP, UTF_REDUCE_MAP};

    #[test]
    fn stem() {
        let lang = lang_english();
        let w    = to_vec("universe");
        assert_eq!(lang.stem(&w), 7);
    }

    #[test]
    fn get_pos() {
        let lang = lang_english();
        let w1   = to_vec("universe");
        let w2   = to_vec("the");
        assert_eq!(lang.get_pos(&w1), None);
        assert_eq!(lang.get_pos(&w2), Some(PartOfSpeech::Article));
    }

    #[test]
    fn unicode_compose() {
        let lang   = lang_english();
        let source = to_vec("universe");
        let norm   = lang.unicode_compose(&source);
        assert_eq!(norm, None);
    }

    #[test]
    fn unicode_reduce() {
        let lang   = lang_english();
        let source = to_vec("universe");
        let norm   = lang.unicode_reduce(&source);
        assert_eq!(norm, None);
    }

    #[test]
    fn unicode_compose_map_dimenstions() {
        for &(nfd, nfc) in UTF_COMPOSE_MAP {
            assert_eq!(nfd.chars().count(), 2);
            assert_eq!(nfc.chars().count(), 1);
        }
    }

    #[test]
    fn unicode_reduce_map_dimenstions() {
        for &(normal, reduced) in UTF_REDUCE_MAP {
            assert_eq!(normal .chars().count(), 1, "UTF_REDUCE_MAP['{}'] != 1", normal);
            assert_eq!(reduced.chars().count(), 1, "UTF_REDUCE_MAP['{}'].len() != 1", reduced);
        }
    }
}