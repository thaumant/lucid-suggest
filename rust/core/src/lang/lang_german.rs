#![allow(dead_code)]

use rust_stemmers::{Algorithm, Stemmer};
use crate::tokenization::PartOfSpeech;
use super::Lang;


const ARTICLES: &[&'static str] = &[
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

const PREPOSITIONS: &[&'static str] = &[
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

const CONJUNCTIONS: &[&'static str] = &[
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

const PARTICLES: &[&'static str] = &[
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