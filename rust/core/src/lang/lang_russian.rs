#![allow(dead_code)]

use rust_stemmers::{Algorithm, Stemmer};
use crate::tokenization::PartOfSpeech;
use super::Lang;


const ARTICLES: &[&'static str] = &[

];

const PREPOSITIONS: &[&'static str] = &[
    "c",
    "без",
    "благодаря",
    "в",
    "ввиду",
    "вдоль",
    "вместо",
    "вне",
    "внутри",
    "внутрь",
    "во",
    "возле",
    "вокруг",
    "вопреки",
    "впереди",
    "вследствие",
    "для",
    "до",
    "за",
    "из-за",
    "из-под",
    "из",
    "к",
    "ко",
    "кроме",
    "между",
    "мимо",
    "на",
    "над",
    "надо",
    "напротив",
    "насчет",
    "о",
    "об",
    "около",
    "от",
    "ото",
    "перед",
    "передо",
    "по",
    "под",
    "подле",
    "подо",
    "позади",
    "помимо",
    "после",
    "посреди",
    "посредством",
    "при",
    "про",
    "против",
    "путём",
    "ради",
    "с",
    "сверх",
    "свыше",
    "сквозь",
    "со",
    "среди",
    "у",
    "через",
];

const CONJUNCTIONS: &[&'static str] = &[
    "а",
    "а",
    "будто",
    "впрочем",
    "где",
    "да",
    "едва",
    "ежели",
    "если",
    "же",
    "зато",
    "и",
    "ибо",
    "или",
    "именно",
    "как",
    "как",
    "когда",
    "которая",
    "которого",
    "которое",
    "котором",
    "которую",
    "которые",
    "который",
    "которых",
    "ли",
    "либо",
    "лишь",
    "настолько",
    "но",
    "однако",
    "пока",
    "покамест",
    "покуда",
    "пускай",
    "пусть",
    "раз",
    "словно",
    "также",
    "то",
    "тоже",
    "точно",
    "хоть",
    "хотя",
    "чем",
    "что",
    "что",
    "чтобы",
    "чтобы",
    "чуть",
    // "а именно",
    // "а то",
    // "благодаря тому что",
    // "будто бы",
    // "в то время как",
    // "ввиду того что",
    // "всё же",
    // "да нет",
    // "для того чтобы",
    // "до того как",
    // "до того",
    // "ещё не ... как",
    // "и ... и",
    // "или ... или",
    // "как ... ни",
    // "как будто бы",
    // "как будто",
    // "как бы",
    // "как ни",
    // "как только",
    // "либо ... либо",
    // "между тем как",
    // "настолько ... насколько",
    // "не ... как",
    // "не настолько ... насколько",
    // "не так ... как",
    // "не то ... не то",
    // "не то что",
    // "не то чтобы",
    // "не только ... но и",
    // "несмотря на то что",
    // "ни ... ни",
    // "оттого что",
    // "перед тем как",
    // "пока .. не",
    // "пока не",
    // "после того как",
    // "потому что",
    // "правда ... но",
    // "прежде чем",
    // "разве только",
    // "разве что",
    // "с тех пор как",
    // "так как",
    // "так что",
    // "то ... то",
    // "то есть",
    // "чем ... тем",
    // "чтобы не",
];

const PARTICLES: &[&'static str] = &[
    "будто",
    "бы",
    "ведь",
    "ведь",
    "вон",
    "вот",
    "да",
    "даже",
    "есть",
    "же",
    "здесь",
    "именно",
    "лишь",
    "не",
    "нет",
    "неужели",
    "ни",
    "ну",
    "ну",
    "пожалуй",
    "почти",
    "просто",
    "разве",
    "словно",
    "там",
    "только",
    "угодно",
    "уж",
    "хоть",
    "хотя",
    "чуть",
    "это",
    // "вряд ли",
    // "да ну",
    // "даже и",
    // "едва ли",
    // "ещё бы",
    // "как будто",
    // "как раз",
    // "лишь только",
    // "ни ... ни",
    // "ну да",
    // "ну и",
    // "только лишь",
    // "хоть бы",
    // "хотя бы",
    // "чуть ли не",
    // "чуть ли",
    // "чуть не",
];

const UTF_COMPOSE_MAP: &[(&'static str, &'static str)] = &[
    ("Ё", "Ё"),
    ("ё", "ё"),
];

const UTF_REDUCE_MAP: &[(&'static str, &'static str)] = &[
    ("Ё", "Е"),
    ("ё", "е"),
];


pub fn lang_russian() -> Lang {
    let mut lang = Lang::new();

    lang.set_stemmer(Some(Stemmer::create(Algorithm::Russian)));

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
    use super::{lang_russian, UTF_COMPOSE_MAP, UTF_REDUCE_MAP};

    #[test]
    pub fn stem() {
        let lang = lang_russian();
        let w    = to_vec("важный");
        assert_eq!(lang.stem(&w), 4);
    }

    #[test]
    pub fn get_pos() {
        let lang = lang_russian();
        let w1   = to_vec("важный");
        let w2   = to_vec("ведь");
        assert_eq!(lang.get_pos(&w1), None);
        assert_eq!(lang.get_pos(&w2), Some(PartOfSpeech::Particle));
    }

    #[test]
    fn unicode_compose() {
        let lang = lang_russian();

        let source1 = to_vec("важный");
        let norm1   = lang.unicode_compose(&source1);
        assert_eq!(norm1, None);

        let source2 = to_vec("Ёлка");
        let norm2   = lang.unicode_compose(&source2).unwrap();
        assert_eq!(to_str(&norm2), "Ёлка");
        assert_eq!(norm2.len(), source2.len() - 1);
    }

    #[test]
    fn unicode_reduce() {
        let lang = lang_russian();

        let source1 = to_vec("важный");
        let norm1   = lang.unicode_reduce(&source1);
        assert_eq!(norm1, None);

        let source2 = to_vec("Ёлка");
        let (padded2, norm2) = lang.unicode_reduce(&source2).unwrap();
        assert_eq!(to_str(&padded2), to_str(&source2));
        assert_eq!(to_str(&norm2), "Елка");
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