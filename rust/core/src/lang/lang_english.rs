use std::collections::HashMap;
use rust_stemmers::{Algorithm, Stemmer};
use super::Lang;
use crate::lexis::PartOfSpeech;


const ARTICLES: [&'static str; 3] = [
    "a",
    "an",
    "the",
];

const PREPOSITIONS: [&'static str; 8] = [
    "at",
    "by",
    "for",
    "from",
    "in",
    "of",
    "on",
    "to",
];

const CONJUNCTIONS: [&'static str; 9] = [
    "and",
    "as",
    "but",
    "for",
    "if",
    "nor",
    "or",
    "so",
    "yet",
];

const PARTICLES: [&'static str; 5] = [
    "by",
    "in",
    "not",
    "on",
    "to",
];


pub fn lang_english() -> Lang {
    let stemmer = Stemmer::create(Algorithm::English);

    let mut pos_map = HashMap::new();
    for w in &ARTICLES     { pos_map.insert(w.chars().collect(), PartOfSpeech::Article); }
    for w in &PREPOSITIONS { pos_map.insert(w.chars().collect(), PartOfSpeech::Preposition); }
    for w in &CONJUNCTIONS { pos_map.insert(w.chars().collect(), PartOfSpeech::Conjunction); }
    for w in &PARTICLES    { pos_map.insert(w.chars().collect(), PartOfSpeech::Particle); }

    Lang::new(pos_map, stemmer)
}


#[cfg(test)]
mod tests {
    use crate::lexis::PartOfSpeech;
    use super::lang_english;

    #[test]
    pub fn stem() {
        let lang = lang_english();
        let w1 = "universe".chars().collect::<Vec<_>>();
        let w2 = "the"     .chars().collect::<Vec<_>>();
        assert_eq!(lang.stem(&w1), 7);
        assert_eq!(lang.stem(&w2), 3);
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