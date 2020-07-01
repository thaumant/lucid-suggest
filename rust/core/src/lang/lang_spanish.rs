#![allow(dead_code)]

use rust_stemmers::{Algorithm, Stemmer};
use super::{CharClass, PartOfSpeech};
use super::Lang;
use super::constants::CHAR_CLASSES_LATIN;

use PartOfSpeech::{
    Article,
    Preposition,
    Conjunction,
};


const FUNCTION_WORDS: &[(PartOfSpeech, &'static str)] = &[
    (Article, "el"),
    (Article, "la"),
    (Article, "los"),
    (Article, "las"),
    (Article, "un"),
    (Article, "una"),
    (Article, "unos"),
    (Article, "unas"),

    (Preposition, "a"),
    (Preposition, "abajo"),
    (Preposition, "alrededor"),
    (Preposition, "antes"),
    (Preposition, "aquellos"),
    (Preposition, "arriba"),
    (Preposition, "bajo"),
    (Preposition, "como"),
    (Preposition, "con"),
    (Preposition, "contra"),
    (Preposition, "de"),
    (Preposition, "dentro"),
    (Preposition, "desde"),
    (Preposition, "durante"),
    (Preposition, "en"),
    (Preposition, "encima"),
    (Preposition, "entre"),
    (Preposition, "esta"),
    (Preposition, "esto"),
    (Preposition, "estos"),
    (Preposition, "fuera"),
    (Preposition, "hacia"),
    (Preposition, "hasta"),
    (Preposition, "más"),
    (Preposition, "opuesto"),
    (Preposition, "para"),
    (Preposition, "pero"),
    (Preposition, "por"),
    (Preposition, "próximo"),
    (Preposition, "que"),
    (Preposition, "salvo"),
    (Preposition, "sin"),
    (Preposition, "sobre"),
    (Preposition, "vía"),
    // (Preposition, "a causa de"),
    // (Preposition, "a diferencia de"),
    // (Preposition, "a pesar de"),
    // (Preposition, "a pesar de"),
    // (Preposition, "a través de"),
    // (Preposition, "a través de"),
    // (Preposition, "además de"),
    // (Preposition, "al lado de"),
    // (Preposition, "al lado de"),
    // (Preposition, "alrededor de"),
    // (Preposition, "antes de"),
    // (Preposition, "así como"),
    // (Preposition, "cerca de"),
    // (Preposition, "cerca de"),
    // (Preposition, "cerca de"),
    // (Preposition, "de acuerdo con"),
    // (Preposition, "debido a"),
    // (Preposition, "delante de"),
    // (Preposition, "dentro de"),
    // (Preposition, "dentro de"),
    // (Preposition, "después de"),
    // (Preposition, "detrás de"),
    // (Preposition, "en lo que"),
    // (Preposition, "en lugar de"),
    // (Preposition, "en nombre de"),
    // (Preposition, "encima de"),
    // (Preposition, "fuera de"),
    // (Preposition, "lejos de"),
    // (Preposition, "más allá de"),
    // (Preposition, "por debajo de"),
    // (Preposition, "por encima"),
    // (Preposition, "tres palabras"),

    (Conjunction, "aunque"),
    (Conjunction, "como"),
    (Conjunction, "e"),
    (Conjunction, "entonces"),
    (Conjunction, "excepto"),
    (Conjunction, "mas"),
    (Conjunction, "o"),
    (Conjunction, "pero"),
    (Conjunction, "porque"),
    (Conjunction, "pues"),
    (Conjunction, "si"),
    (Conjunction, "sino"),
    (Conjunction, "u"),
    (Conjunction, "y"),
    // (Conjunction, "con tal de que"),
    // (Conjunction, "es decir"),
    // (Conjunction, "esto es"),
    // (Conjunction, "ni … ni"),
    // (Conjunction, "no obstante"),
    // (Conjunction, "o … o"),
    // (Conjunction, "o bien … o bien"),
    // (Conjunction, "por lo demás"),
    // (Conjunction, "puesto que"),
    // (Conjunction, "sea … sea"),
    // (Conjunction, "siempre que"),
    // (Conjunction, "sin embargo"),
    // (Conjunction, "ya que"),
];


const CHAR_CLASSES: &[(CharClass, char)] = &[];


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

    for &(pos, word) in FUNCTION_WORDS { lang.add_pos(word, pos); }

    for &(class, ch) in CHAR_CLASSES_LATIN { lang.add_char_class(ch, class); }
    for &(class, ch) in CHAR_CLASSES       { lang.add_char_class(ch, class); }

    lang
}


#[cfg(test)]
mod tests {
    use crate::utils::{to_vec, to_str};
    use super::{PartOfSpeech, CharClass};
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

    #[test]
    fn get_char_class() {
        let lang = lang_spanish();
        assert_eq!(lang.get_char_class('a'), Some(CharClass::Vowel));
        assert_eq!(lang.get_char_class('n'), Some(CharClass::Consonant));
        assert_eq!(lang.get_char_class('%'), None);
    }
}