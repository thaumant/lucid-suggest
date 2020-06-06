#![allow(dead_code)]

use std::collections::HashMap;
use rust_stemmers::{Algorithm, Stemmer};
use super::Lang;
use crate::lexis::PartOfSpeech;


const ARTICLES: [&'static str; 3] = [
    "a",
    "an",
    "the",
];

const PREPOSITIONS: [&'static str; 25] = [
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

const CONJUNCTIONS: [&'static str; 52] = [
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

const PARTICLES: [&'static str; 6] = [
    "by",
    "in",
    "not",
    "on",
    "to",
    "oh",
];


pub fn lang_english() -> Lang {
    let stemmer = Stemmer::create(Algorithm::English);

    let mut pos_map = HashMap::new();
    for w in &ARTICLES[..]     { pos_map.insert(w.chars().collect(), PartOfSpeech::Article); }
    for w in &PREPOSITIONS[..] { pos_map.insert(w.chars().collect(), PartOfSpeech::Preposition); }
    for w in &CONJUNCTIONS[..] { pos_map.insert(w.chars().collect(), PartOfSpeech::Conjunction); }
    for w in &PARTICLES[..]    { pos_map.insert(w.chars().collect(), PartOfSpeech::Particle); }

    Lang::new(pos_map, stemmer)
}


#[cfg(test)]
mod tests {
    use crate::lexis::PartOfSpeech;
    use super::lang_english;

    #[test]
    pub fn stem() {
        let lang = lang_english();
        let w = "universe".chars().collect::<Vec<_>>();
        assert_eq!(lang.stem(&w), 7);
    }

    #[test]
    pub fn get_pos() {
        let lang = lang_english();
        let w1 = "universe".chars().collect::<Vec<_>>();
        let w2 = "the"     .chars().collect::<Vec<_>>();
        assert_eq!(lang.get_pos(&w1), None);
        assert_eq!(lang.get_pos(&w2), Some(PartOfSpeech::Article));
    }
}