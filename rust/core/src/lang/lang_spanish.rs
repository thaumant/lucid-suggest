#![allow(dead_code)]

use std::collections::HashMap;
use rust_stemmers::{Algorithm, Stemmer};
use crate::tokenization::PartOfSpeech;
use super::Lang;
use super::utils::compile_utf_map;


const ARTICLES: [&'static str; 8] = [
    "el",
    "la",
    "los",
    "las",
    "un",
    "una",
    "unos",
    "unas",
];

const PREPOSITIONS: [&'static str; 34] = [
    "a",
    "abajo",
    "alrededor",
    "antes",
    "aquellos",
    "arriba",
    "bajo",
    "como",
    "con",
    "contra",
    "de",
    "dentro",
    "desde",
    "durante",
    "en",
    "encima",
    "entre",
    "esta",
    "esto",
    "estos",
    "fuera",
    "hacia",
    "hasta",
    "más",
    "opuesto",
    "para",
    "pero",
    "por",
    "próximo",
    "que",
    "salvo",
    "sin",
    "sobre",
    "vía",
    // "a causa de",
    // "a diferencia de",
    // "a pesar de",
    // "a pesar de",
    // "a través de",
    // "a través de",
    // "además de",
    // "al lado de",
    // "al lado de",
    // "alrededor de",
    // "antes de",
    // "así como",
    // "cerca de",
    // "cerca de",
    // "cerca de",
    // "de acuerdo con",
    // "debido a",
    // "delante de",
    // "dentro de",
    // "dentro de",
    // "después de",
    // "detrás de",
    // "en lo que",
    // "en lugar de",
    // "en nombre de",
    // "encima de",
    // "fuera de",
    // "lejos de",
    // "más allá de",
    // "por debajo de",
    // "por encima",
    // "tres palabras",
];

const CONJUNCTIONS: [&'static str; 14] = [
    "aunque",
    "como",
    "e",
    "entonces",
    "excepto",
    "mas",
    "o",
    "pero",
    "porque",
    "pues",
    "si",
    "sino",
    "u",
    "y",
    // "con tal de que",
    // "es decir",
    // "esto es",
    // "ni … ni",
    // "no obstante",
    // "o … o",
    // "o bien … o bien",
    // "por lo demás",
    // "puesto que",
    // "sea … sea",
    // "siempre que",
    // "sin embargo",
    // "ya que",
];

const PARTICLES: [&'static str; 0] = [
];

const UTF_COMPOSE_MAP: [(&'static str, &'static str); 14] = [
    ("Á", "Á"), // acute accent
    ("É", "É"),
    ("Í", "Í"),
    ("Ó", "Ó"),
    ("Ú", "Ú"),
    ("á", "á"),
    ("é", "é"),
    ("í", "í"),
    ("ó", "ó"),
    ("ú", "ú"),
    ("Ñ", "Ñ"), // tilde
    ("ñ", "ñ"),
    ("Ü", "Ü"), // diaeresis
    ("ü", "ü"),
];

const UTF_REDUCE_MAP: [(&'static str, &'static str); 14] = [
    ("Á", "A"), // acute accent
    ("É", "E"),
    ("Í", "I"),
    ("Ó", "O"),
    ("Ú", "U"),
    ("á", "a"),
    ("é", "e"),
    ("í", "i"),
    ("ó", "o"),
    ("ú", "u"),
    ("Ñ", "N"), // tilde
    ("ñ", "n"),
    ("Ü", "U"), // diaeresis
    ("ü", "u"),
];


pub fn lang_spanish() -> Lang {
    let stemmer = Stemmer::create(Algorithm::Spanish);

    let compose_map = compile_utf_map(&UTF_COMPOSE_MAP[..]);
    let reduce_map  = compile_utf_map(&UTF_REDUCE_MAP[..]);

    let mut pos_map = HashMap::new();
    for w in &ARTICLES[..]     { pos_map.insert(w.chars().collect(), PartOfSpeech::Article); }
    for w in &PREPOSITIONS[..] { pos_map.insert(w.chars().collect(), PartOfSpeech::Preposition); }
    for w in &CONJUNCTIONS[..] { pos_map.insert(w.chars().collect(), PartOfSpeech::Conjunction); }
    for w in &PARTICLES[..]    { pos_map.insert(w.chars().collect(), PartOfSpeech::Particle); }

    Lang::new(pos_map, compose_map, reduce_map, stemmer)
}


#[cfg(test)]
mod tests {
    use crate::tokenization::PartOfSpeech;
    use super::{lang_spanish, UTF_COMPOSE_MAP, UTF_REDUCE_MAP};

    #[test]
    pub fn stem() {
        let lang = lang_spanish();
        let w = "torniquete".chars().collect::<Vec<char>>();
        assert_eq!(lang.stem(&w), 9);
    }

    #[test]
    pub fn get_pos() {
        let lang = lang_spanish();
        let w1 = "torniquete".chars().collect::<Vec<char>>();
        let w2 = "una"       .chars().collect::<Vec<char>>();
        assert_eq!(lang.get_pos(&w1), None);
        assert_eq!(lang.get_pos(&w2), Some(PartOfSpeech::Article));
    }

    #[test]
    fn utf_compose() {
        let lang = lang_spanish();

        let source1 = "torniquete";
        let norm1   = lang.utf_compose(&source1.chars().collect::<Vec<_>>());
        assert_eq!(norm1, None);

        let source2 = "piñata";
        let norm2   = lang
            .utf_compose(&source2.chars().collect::<Vec<_>>())
            .unwrap()
            .iter()
            .collect::<String>();
        assert_eq!(norm2, "piñata");
        assert_eq!(norm2.chars().count(), source2.chars().count() - 1);
    }

    #[test]
    fn utf_reduce() {
        let lang = lang_spanish();

        let source1 = "torniquete";
        let norm1   = lang.utf_reduce(&source1.chars().collect::<Vec<_>>());
        assert_eq!(norm1, None);

        let source2 = "piñata";
        let (padded2, norm2) = lang
            .utf_reduce(&source2.chars().collect::<Vec<_>>())
            .unwrap();
        let padded2 = padded2.iter().collect::<String>();
        let norm2   = norm2  .iter().collect::<String>();
        assert_eq!(padded2, source2);
        assert_eq!(norm2, "pinata");
        assert_eq!(norm2.chars().count(), source2.chars().count());
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