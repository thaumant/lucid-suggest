#![allow(dead_code)]

use std::collections::HashMap;
use rust_stemmers::{Algorithm, Stemmer};
use super::Lang;
use crate::lexis::PartOfSpeech;


const ARTICLES: [&'static str; 8] = [
    "o",
    "a",
    "os",
    "as",
    "um",
    "uma",
    "uns",
    "umas",
];

const PREPOSITIONS: [&'static str; 39] = [
    "abaixo",
    "acima",
    "além",
    "antes",
    "aproximadamente",
    "aquela",
    "aquele",
    "aqueles",
    "até",
    "atrás",
    "com",
    "como",
    "conforme",
    "contra",
    "de",
    "depois",
    "desde",
    "distante",
    "durante",
    "em",
    "entre",
    "esta",
    "estas",
    "este",
    "estes",
    "exceto",
    "fora",
    "mais",
    "mas",
    "oposto",
    "para",
    "perto",
    "por",
    "próximo",
    "que",
    "sem",
    "sob",
    "sobre",
    "via",
    // "além de",
    // "antes de",
    // "ao lado de",
    // "apesar de",
    // "apesar de",
    // "através de",
    // "através de",
    // "bem como",
    // "dentro de",
    // "dentro de",
    // "dentro de",
    // "devido a",
    // "diferente de",
    // "em cima de",
    // "em direção a",
    // "em nome de",
    // "em torno de",
    // "em vez de",
    // "fora de",
    // "fora de",
    // "longe de",
    // "na frente de",
    // "para baixo",
    // "para cima",
    // "perto de",
    // "por causa de",
    // "próximo a",
    // "próximo de",
    // "tanto quanto",
];

const CONJUNCTIONS: [&'static str; 19] = [
    "agora",
    "como",
    "contudo",
    "e",
    "enquanto",
    "então",
    "logo",
    "mas",
    "nem",
    "ou",
    "para",
    "pois",
    "por",
    "porém",
    "porque",
    "portanto",
    "quando",
    "se",
    "todavia",
    // "a fim de",
    // "ainda assim",
    // "ainda que",
    // "apesar disso",
    // "assim que",
    // "como se",
    // "enquanto que",
    // "não só ... como também",
    // "no entanto",
    // "ora ... ora",
    // "ou ... ou",
    // "por conseguinte",
    // "por isso",
    // "quer ... quer",
    // "sempre que",
    // "tanto ... como",
    // "visto que",
];

const PARTICLES: [&'static str; 0] = [
];


pub fn lang_portuguese() -> Lang {
    let stemmer = Stemmer::create(Algorithm::Portuguese);

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
    use super::lang_portuguese;

    #[test]
    pub fn stem() {
        let lang = lang_portuguese();
        let w = "quilométricas".chars().collect::<Vec<_>>();
        assert_eq!(lang.stem(&w), 9);
    }

    #[test]
    pub fn get_pos() {
        let lang = lang_portuguese();
        let w1 = "quilométricas".chars().collect::<Vec<_>>();
        let w2 = "uma"          .chars().collect::<Vec<_>>();
        assert_eq!(lang.get_pos(&w1), None);
        assert_eq!(lang.get_pos(&w2), Some(PartOfSpeech::Article));
    }
}