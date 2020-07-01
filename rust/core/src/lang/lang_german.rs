#![allow(dead_code)]

use rust_stemmers::{Algorithm, Stemmer};
use crate::tokenization::PartOfSpeech;
use super::Lang;

use PartOfSpeech::{
    Article,
    Preposition,
    Conjunction,
    Particle,
};


const FUNCTION_WORDS: &[(PartOfSpeech, &'static str)] = &[
    (Article, "das"),
    (Article, "dem"),
    (Article, "den"),
    (Article, "der"),
    (Article, "des"),
    (Article, "die"),
    (Article, "ein"),
    (Article, "eine"),
    (Article, "einem"),
    (Article, "einen"),
    (Article, "einer"),
    (Article, "eines"),

    (Preposition, "an"),
    (Preposition, "auf"),
    (Preposition, "aus"),
    (Preposition, "bei"),
    (Preposition, "bis"),
    (Preposition, "durch"),
    (Preposition, "entlang"),
    (Preposition, "für"),
    (Preposition, "gegen"),
    (Preposition, "hinter"),
    (Preposition, "in"),
    (Preposition, "mit"),
    (Preposition, "nach"),
    (Preposition, "neben"),
    (Preposition, "ohne"),
    (Preposition, "seit"),
    (Preposition, "um"),
    (Preposition, "von"),
    (Preposition, "zu"),

    (Conjunction, "aber"),
    (Conjunction, "als"),
    (Conjunction, "als"),
    (Conjunction, "anstatt"),
    (Conjunction, "auch"),
    (Conjunction, "bevor"),
    (Conjunction, "bis"),
    (Conjunction, "but"),
    (Conjunction, "damit"),
    (Conjunction, "dass"),
    (Conjunction, "denn"),
    (Conjunction, "entweder"),
    (Conjunction, "nachdem"),
    (Conjunction, "noch"),
    (Conjunction, "ob"),
    (Conjunction, "obwohl"),
    (Conjunction, "oder"),
    (Conjunction, "oder"),
    (Conjunction, "seitdem"),
    (Conjunction, "sobald"),
    (Conjunction, "sofern"),
    (Conjunction, "sondern"),
    (Conjunction, "soweit"),
    (Conjunction, "sowie"),
    (Conjunction, "sowohl"),
    (Conjunction, "sowohl"),
    (Conjunction, "the"),
    (Conjunction, "und"),
    (Conjunction, "während"),
    (Conjunction, "weder"),
    (Conjunction, "weil"),
    (Conjunction, "wenn"),
    (Conjunction, "wie"),
    (Conjunction, "wie"),
    (Conjunction, "wo"),
    (Conjunction, "zu"),

    (Particle, "schon"),
    (Particle, "ja"),
    (Particle, "halt"),
    (Particle, "wohl"),
    (Particle, "doch"),
    (Particle, "mal"),
    (Particle, "aber"),
    (Particle, "auch"),
    (Particle, "bloß"),
    (Particle, "denn"),
    (Particle, "eben"),
    (Particle, "etwas"),
    (Particle, "nur"),
    (Particle, "ruhig"),
    (Particle, "shon"),
    (Particle, "zwar"),
    (Particle, "soweiso"),
];

const UTF_COMPOSE_MAP: &[(&'static str, &'static str)] = &[
    ("Ä", "Ä"),
    ("Ö", "Ö"),
    ("Ü", "Ü"),
    ("ä", "ä"),
    ("ö", "ö"),
    ("ü", "ü"),
];

const UTF_REDUCE_MAP: &[(&'static str, &'static str)] = &[
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
    let mut lang = Lang::new();

    lang.set_stemmer(Some(Stemmer::create(Algorithm::German)));

    for (from, to) in UTF_COMPOSE_MAP { lang.add_unicode_composition(from, to); }
    for (from, to) in UTF_REDUCE_MAP  { lang.add_unicode_reduction(from, to); }

    for &(pos, word) in FUNCTION_WORDS { lang.add_pos(word, pos); }

    lang
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
    fn unicode_compose() {
        let lang    = lang_german();

        let source1 = to_vec("singen");
        let norm1   = lang.unicode_compose(&source1);
        assert_eq!(norm1, None);

        let source2 = to_vec("mädchen");
        let norm2   = lang.unicode_compose(&source2).unwrap();
        assert_eq!(to_str(&norm2), "mädchen");
        assert_eq!(norm2.len(), source2.len() - 1);
    }

    #[test]
    fn unicode_reduce() {
        let lang = lang_german();

        let source1 = to_vec("singen");
        let norm1   = lang.unicode_reduce(&source1);
        assert_eq!(norm1, None);

        let source2 = to_vec("mädchen");
        let (padded2, norm2) = lang.unicode_reduce(&source2).unwrap();
        assert_eq!(to_str(&padded2), to_str(&source2));
        assert_eq!(to_str(&norm2), "madchen");
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
            if normal == "ẞ" { continue; }
            if normal == "ß" { continue; }
            assert_eq!(normal .chars().count(), 1, "UTF_REDUCE_MAP['{}'] != 1", normal);
            assert_eq!(reduced.chars().count(), 1, "UTF_REDUCE_MAP['{}'].len() != 1", reduced);
        }
    }
}