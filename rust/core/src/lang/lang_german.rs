#![allow(dead_code)]

use std::collections::HashMap;
use rust_stemmers::{Algorithm, Stemmer};
use crate::utils::to_vec;
use crate::tokenization::PartOfSpeech;
use super::Lang;
use super::utils::compile_utf_map;


const ARTICLES: [&'static str; 12] = [
    "das",
    "dem",
    "den",
    "der",
    "des",
    "die",
    "ein",
    "eine",
    "einem",
    "einen",
    "einer",
    "eines",
];

const PREPOSITIONS: [&'static str; 19] = [
    "an",
    "auf",
    "aus",
    "bei",
    "bis",
    "durch",
    "entlang",
    "für",
    "gegen",
    "hinter",
    "in",
    "mit",
    "nach",
    "neben",
    "ohne",
    "seit",
    "um",
    "von",
    "zu",
];

const CONJUNCTIONS: [&'static str; 36] = [
    "aber",
    "als",
    "als",
    "anstatt",
    "auch",
    "bevor",
    "bis",
    "but",
    "damit",
    "dass",
    "denn",
    "entweder",
    "nachdem",
    "noch",
    "ob",
    "obwohl",
    "oder",
    "oder",
    "seitdem",
    "sobald",
    "sofern",
    "sondern",
    "soweit",
    "sowie",
    "sowohl",
    "sowohl",
    "the",
    "und",
    "während",
    "weder",
    "weil",
    "wenn",
    "wie",
    "wie",
    "wo",
    "zu",
];

const PARTICLES: [&'static str; 17] = [
    "schon",
    "ja",
    "halt",
    "wohl",
    "doch",
    "mal",
    "aber",
    "auch",
    "bloß",
    "denn",
    "eben",
    "etwas",
    "nur",
    "ruhig",
    "shon",
    "zwar",
    "soweiso",
];

const UTF_COMPOSE_MAP: [(&'static str, &'static str); 6] = [
    ("Ä", "Ä"),
    ("Ö", "Ö"),
    ("Ü", "Ü"),
    ("ä", "ä"),
    ("ö", "ö"),
    ("ü", "ü"),
];

const UTF_REDUCE_MAP: [(&'static str, &'static str); 8] = [
    ("ẞ", "SS"), // eszett
    ("ß", "ss"),
    ("Ä", "A"), // umlauts
    ("Ö", "O"),
    ("Ü", "U"),
    ("ä", "a"),
    ("ö", "o"),
    ("ü", "u"),
];


pub fn lang_german() -> Lang {
    let stemmer = Stemmer::create(Algorithm::German);

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
    use crate::utils::{to_vec, to_str};
    use crate::tokenization::PartOfSpeech;
    use super::{lang_german, UTF_COMPOSE_MAP, UTF_REDUCE_MAP};

    #[test]
    pub fn stem() {
        let lang = lang_german();
        let w    = to_vec("singen");
        assert_eq!(lang.stem(&w), 4);
    }

    #[test]
    pub fn get_pos() {
        let lang = lang_german();
        let w1   = to_vec("singen");
        let w2   = to_vec("das");
        assert_eq!(lang.get_pos(&w1), None);
        assert_eq!(lang.get_pos(&w2), Some(PartOfSpeech::Article));
    }

    #[test]
    fn utf_compose() {
        let lang    = lang_german();

        let source1 = to_vec("singen");
        let norm1   = lang.utf_compose(&source1);
        assert_eq!(norm1, None);

        let source2 = to_vec("mädchen");
        let norm2   = lang.utf_compose(&source2).unwrap();
        assert_eq!(to_str(&norm2), "mädchen");
        assert_eq!(norm2.len(), source2.len() - 1);
    }

    #[test]
    fn utf_reduce() {
        let lang = lang_german();

        let source1 = to_vec("singen");
        let norm1   = lang.utf_reduce(&source1);
        assert_eq!(norm1, None);

        let source2 = to_vec("mädchen");
        let (padded2, norm2) = lang.utf_reduce(&source2).unwrap();
        assert_eq!(to_str(&padded2), to_str(&source2));
        assert_eq!(to_str(&norm2), "madchen");
        assert_eq!(norm2.len(), source2.len());
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
            if normal == "ẞ" { continue; }
            if normal == "ß" { continue; }
            assert_eq!(normal .chars().count(), 1, "UTF_REDUCE_MAP['{}'] != 1", normal);
            assert_eq!(reduced.chars().count(), 1, "UTF_REDUCE_MAP['{}'].len() != 1", reduced);
        }
    }
}