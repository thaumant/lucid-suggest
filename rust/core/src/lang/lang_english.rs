#![allow(dead_code)]

use std::collections::HashMap;
use rust_stemmers::{Algorithm, Stemmer};
use crate::utils::to_vec;
use crate::tokenization::PartOfSpeech;
use super::Lang;
use super::utils::compile_utf_map;


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

const UTF_COMPOSE_MAP: [(&'static str, &'static str); 0] = [];

const UTF_REDUCE_MAP: [(&'static str, &'static str); 0] = [];


pub fn lang_english() -> Lang {
    let stemmer = Stemmer::create(Algorithm::English);

    let compose_map = compile_utf_map(&UTF_COMPOSE_MAP[..]);
    let reduce_map  = compile_utf_map(&UTF_REDUCE_MAP[..]);

    let mut pos_map = HashMap::new();
    for w in &ARTICLES[..]     { pos_map.insert(to_vec(w), PartOfSpeech::Article); }
    for w in &PREPOSITIONS[..] { pos_map.insert(to_vec(w), PartOfSpeech::Preposition); }
    for w in &CONJUNCTIONS[..] { pos_map.insert(to_vec(w), PartOfSpeech::Conjunction); }
    for w in &PARTICLES[..]    { pos_map.insert(to_vec(w), PartOfSpeech::Particle); }

    Lang::new(pos_map, compose_map, reduce_map, stemmer)
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
    fn utf_compose() {
        let lang   = lang_english();
        let source = to_vec("universe");
        let norm   = lang.utf_compose(&source);
        assert_eq!(norm, None);
    }

    #[test]
    fn utf_reduce() {
        let lang   = lang_english();
        let source = to_vec("universe");
        let norm   = lang.utf_reduce(&source);
        assert_eq!(norm, None);
    }

    #[test]
    fn utf_compose_map_dimenstions() {
        for &(nfd, nfc) in &UTF_COMPOSE_MAP {
            assert_eq!(nfd.chars().count(), 2);
            assert_eq!(nfc.chars().count(), 1);
        }
    }

    #[test]
    fn utf_reduce_map_dimenstions() {
        for &(normal, reduced) in &UTF_REDUCE_MAP {
            assert_eq!(normal .chars().count(), 1, "UTF_REDUCE_MAP['{}'] != 1", normal);
            assert_eq!(reduced.chars().count(), 1, "UTF_REDUCE_MAP['{}'].len() != 1", reduced);
        }
    }
}