#![allow(dead_code)]

use rust_stemmers::{Algorithm, Stemmer};
use super::PartOfSpeech;
use super::Lang;

use PartOfSpeech::{
    Article,
    Preposition,
    Conjunction,
    Particle,
};


const FUNCTION_WORDS: &[(PartOfSpeech, &'static str)] = &[
    // Articles
    (Article, "a"),
    (Article, "an"),
    (Article, "the"),

    (Preposition, "at"),
    (Preposition, "by"),
    (Preposition, "for"),
    (Preposition, "from"),
    (Preposition, "in"),
    (Preposition, "of"),
    (Preposition, "on"),
    (Preposition, "to"),
    (Preposition, "since"),
    (Preposition, "before"),
    (Preposition, "till"),
    (Preposition, "untill"),
    (Preposition, "beside"),
    (Preposition, "under"),
    (Preposition, "below"),
    (Preposition, "over"),
    (Preposition, "above"),
    (Preposition, "across"),
    (Preposition, "through"),
    (Preposition, "into"),
    (Preposition, "towards"),
    (Preposition, "onto"),
    (Preposition, "off"),
    (Preposition, "out"),
    (Preposition, "about"),

    (Conjunction, "after"),
    (Conjunction, "although"),
    (Conjunction, "as"),
    (Conjunction, "because"),
    (Conjunction, "before"),
    (Conjunction, "but"),
    (Conjunction, "either"),
    (Conjunction, "for"),
    (Conjunction, "how"),
    (Conjunction, "if"),
    (Conjunction, "lest"),
    (Conjunction, "nor"),
    (Conjunction, "once"),
    (Conjunction, "once"),
    (Conjunction, "or"),
    (Conjunction, "since"),
    (Conjunction, "since"),
    (Conjunction, "since"),
    (Conjunction, "so"),
    (Conjunction, "than"),
    (Conjunction, "that"),
    (Conjunction, "that"),
    (Conjunction, "though"),
    (Conjunction, "till"),
    (Conjunction, "unless"),
    (Conjunction, "until"),
    (Conjunction, "until"),
    (Conjunction, "until"),
    (Conjunction, "what"),
    (Conjunction, "whatever"),
    (Conjunction, "when"),
    (Conjunction, "when"),
    (Conjunction, "whenever"),
    (Conjunction, "where"),
    (Conjunction, "whereas"),
    (Conjunction, "whereas"),
    (Conjunction, "wherever"),
    (Conjunction, "whether"),
    (Conjunction, "which"),
    (Conjunction, "whichever"),
    (Conjunction, "while"),
    (Conjunction, "while"),
    (Conjunction, "while"),
    (Conjunction, "whilst"),
    (Conjunction, "who"),
    (Conjunction, "whoever"),
    (Conjunction, "whom"),
    (Conjunction, "whomever"),
    (Conjunction, "whose"),
    (Conjunction, "why"),
    (Conjunction, "yet"),
    (Conjunction, "and"),
    // (Conjunction, "as if"),
    // (Conjunction, "as long as"),
    // (Conjunction, "as much as"),
    // (Conjunction, "as soon as"),
    // (Conjunction, "as though"),
    // (Conjunction, "assuming that"),
    // (Conjunction, "by the time"),
    // (Conjunction, "even if"),
    // (Conjunction, "even though"),
    // (Conjunction, "in case that"),
    // (Conjunction, "in case"),
    // (Conjunction, "in order that"),
    // (Conjunction, "in order"),
    // (Conjunction, "now that"),
    // (Conjunction, "only if"),
    // (Conjunction, "provided that"),
    // (Conjunction, "rather than"),
    // (Conjunction, "so that"),

    (Particle, "by"),
    (Particle, "in"),
    (Particle, "not"),
    (Particle, "on"),
    (Particle, "to"),
    (Particle, "oh"),
];

const UTF_COMPOSE_MAP: &[(&'static str, &'static str)] = &[];

const UTF_REDUCE_MAP: &[(&'static str, &'static str)] = &[];


pub fn lang_english() -> Lang {
    let mut lang = Lang::new();

    lang.set_stemmer(Some(Stemmer::create(Algorithm::English)));

    for (from, to) in UTF_COMPOSE_MAP { lang.add_unicode_composition(from, to); }
    for (from, to) in UTF_REDUCE_MAP  { lang.add_unicode_reduction(from, to); }

    for &(pos, word) in FUNCTION_WORDS { lang.add_pos(word, pos); }

    lang
}


#[cfg(test)]
mod tests {
    use crate::utils::to_vec;
    use super::PartOfSpeech;
    use super::{lang_english, UTF_COMPOSE_MAP, UTF_REDUCE_MAP};

    #[test]
    fn stem() {
        let lang = lang_english();
        let w    = to_vec("universe");
        assert_eq!(lang.stem(&w), 7);
    }

    #[test]
    fn get_pos() {
        let lang = lang_english();
        let w1   = to_vec("universe");
        let w2   = to_vec("the");
        assert_eq!(lang.get_pos(&w1), None);
        assert_eq!(lang.get_pos(&w2), Some(PartOfSpeech::Article));
    }

    #[test]
    fn unicode_compose() {
        let lang   = lang_english();
        let source = to_vec("universe");
        let norm   = lang.unicode_compose(&source);
        assert_eq!(norm, None);
    }

    #[test]
    fn unicode_reduce() {
        let lang   = lang_english();
        let source = to_vec("universe");
        let norm   = lang.unicode_reduce(&source);
        assert_eq!(norm, None);
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