#![allow(dead_code)]

use rust_stemmers::{Algorithm, Stemmer};
use crate::tokenization::PartOfSpeech;
use super::Lang;


const ARTICLES: &[&'static str] = &[
    "el",
    "la",
    "los",
    "las",
    "un",
    "una",
    "unos",
    "unas",
];

const PREPOSITIONS: &[&'static str] = &[
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

const CONJUNCTIONS: &[&'static str] = &[
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

const PARTICLES: &[&'static str] = &[
];

const UTF_COMPOSE_MAP: &[(&'static str, &'static str)] = &[
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

const UTF_REDUCE_MAP: &[(&'static str, &'static str)] = &[
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
    let mut lang = Lang::new();

    lang.set_stemmer(Some(Stemmer::create(Algorithm::Spanish)));

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
    use crate::utils::{to_vec, to_str};
    use crate::tokenization::PartOfSpeech;
    use super::{lang_spanish, UTF_COMPOSE_MAP, UTF_REDUCE_MAP};

    #[test]
    pub fn stem() {
        let lang = lang_spanish();
        let w = to_vec("torniquete");
        assert_eq!(lang.stem(&w), 9);
    }

    #[test]
    pub fn get_pos() {
        let lang = lang_spanish();
        let w1 = to_vec("torniquete");
        let w2 = to_vec("una");
        assert_eq!(lang.get_pos(&w1), None);
        assert_eq!(lang.get_pos(&w2), Some(PartOfSpeech::Article));
    }

    #[test]
    fn unicode_compose() {
        let lang = lang_spanish();

        let source1 = to_vec("torniquete");
        let norm1   = lang.unicode_compose(&source1);
        assert_eq!(norm1, None);

        let source2 = to_vec("piñata");
        let norm2   = lang.unicode_compose(&source2).unwrap();
        assert_eq!(to_str(&norm2), "piñata");
        assert_eq!(norm2.len(), source2.len() - 1);
    }

    #[test]
    fn unicode_reduce() {
        let lang = lang_spanish();

        let source1 = to_vec("torniquete");
        let norm1   = lang.unicode_reduce(&source1);
        assert_eq!(norm1, None);

        let source2 = to_vec("piñata");
        let (padded2, norm2) = lang.unicode_reduce(&source2).unwrap();
        assert_eq!(to_str(&padded2), to_str(&source2));
        assert_eq!(to_str(&norm2), "pinata");
        assert_eq!(norm2.len(), source2.len());
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