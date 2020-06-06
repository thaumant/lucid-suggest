#![allow(dead_code)]

use std::collections::HashMap;
use rust_stemmers::{Algorithm, Stemmer};
use super::Lang;
use crate::lexis::PartOfSpeech;


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


pub fn lang_spanish() -> Lang {
    let stemmer = Stemmer::create(Algorithm::Spanish);

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
    use super::lang_spanish;

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
}