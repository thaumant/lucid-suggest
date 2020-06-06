#![allow(dead_code)]

use std::collections::HashMap;
use rust_stemmers::{Algorithm, Stemmer};
use super::Lang;
use crate::lexis::PartOfSpeech;


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


pub fn lang_german() -> Lang {
    let stemmer = Stemmer::create(Algorithm::German);

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
    use super::lang_german;

    #[test]
    pub fn stem() {
        let lang = lang_german();
        let w = "singen".chars().collect::<Vec<_>>();
        assert_eq!(lang.stem(&w), 4);
    }

    #[test]
    pub fn get_pos() {
        let lang = lang_german();
        let w1 = "singen".chars().collect::<Vec<_>>();
        let w2 = "das"   .chars().collect::<Vec<_>>();
        assert_eq!(lang.get_pos(&w1), None);
        assert_eq!(lang.get_pos(&w2), Some(PartOfSpeech::Article));
    }
}