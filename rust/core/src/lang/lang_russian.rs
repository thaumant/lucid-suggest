#![allow(dead_code)]

use rust_stemmers::{Algorithm, Stemmer};
use super::PartOfSpeech;
use super::Lang;

use PartOfSpeech::{
    Preposition,
    Conjunction,
    Particle,
};


const FUNCTION_WORDS: &[(PartOfSpeech, &'static str)] = &[
    (Preposition, "c"),
    (Preposition, "без"),
    (Preposition, "благодаря"),
    (Preposition, "в"),
    (Preposition, "ввиду"),
    (Preposition, "вдоль"),
    (Preposition, "вместо"),
    (Preposition, "вне"),
    (Preposition, "внутри"),
    (Preposition, "внутрь"),
    (Preposition, "во"),
    (Preposition, "возле"),
    (Preposition, "вокруг"),
    (Preposition, "вопреки"),
    (Preposition, "впереди"),
    (Preposition, "вследствие"),
    (Preposition, "для"),
    (Preposition, "до"),
    (Preposition, "за"),
    (Preposition, "из-за"),
    (Preposition, "из-под"),
    (Preposition, "из"),
    (Preposition, "к"),
    (Preposition, "ко"),
    (Preposition, "кроме"),
    (Preposition, "между"),
    (Preposition, "мимо"),
    (Preposition, "на"),
    (Preposition, "над"),
    (Preposition, "надо"),
    (Preposition, "напротив"),
    (Preposition, "насчет"),
    (Preposition, "о"),
    (Preposition, "об"),
    (Preposition, "около"),
    (Preposition, "от"),
    (Preposition, "ото"),
    (Preposition, "перед"),
    (Preposition, "передо"),
    (Preposition, "по"),
    (Preposition, "под"),
    (Preposition, "подле"),
    (Preposition, "подо"),
    (Preposition, "позади"),
    (Preposition, "помимо"),
    (Preposition, "после"),
    (Preposition, "посреди"),
    (Preposition, "посредством"),
    (Preposition, "при"),
    (Preposition, "про"),
    (Preposition, "против"),
    (Preposition, "путём"),
    (Preposition, "ради"),
    (Preposition, "с"),
    (Preposition, "сверх"),
    (Preposition, "свыше"),
    (Preposition, "сквозь"),
    (Preposition, "со"),
    (Preposition, "среди"),
    (Preposition, "у"),
    (Preposition, "через"),

    (Conjunction, "а"),
    (Conjunction, "а"),
    (Conjunction, "будто"),
    (Conjunction, "впрочем"),
    (Conjunction, "где"),
    (Conjunction, "да"),
    (Conjunction, "едва"),
    (Conjunction, "ежели"),
    (Conjunction, "если"),
    (Conjunction, "же"),
    (Conjunction, "зато"),
    (Conjunction, "и"),
    (Conjunction, "ибо"),
    (Conjunction, "или"),
    (Conjunction, "именно"),
    (Conjunction, "как"),
    (Conjunction, "как"),
    (Conjunction, "когда"),
    (Conjunction, "которая"),
    (Conjunction, "которого"),
    (Conjunction, "которое"),
    (Conjunction, "котором"),
    (Conjunction, "которую"),
    (Conjunction, "которые"),
    (Conjunction, "который"),
    (Conjunction, "которых"),
    (Conjunction, "ли"),
    (Conjunction, "либо"),
    (Conjunction, "лишь"),
    (Conjunction, "настолько"),
    (Conjunction, "но"),
    (Conjunction, "однако"),
    (Conjunction, "пока"),
    (Conjunction, "покамест"),
    (Conjunction, "покуда"),
    (Conjunction, "пускай"),
    (Conjunction, "пусть"),
    (Conjunction, "раз"),
    (Conjunction, "словно"),
    (Conjunction, "также"),
    (Conjunction, "то"),
    (Conjunction, "тоже"),
    (Conjunction, "точно"),
    (Conjunction, "хоть"),
    (Conjunction, "хотя"),
    (Conjunction, "чем"),
    (Conjunction, "что"),
    (Conjunction, "что"),
    (Conjunction, "чтобы"),
    (Conjunction, "чтобы"),
    (Conjunction, "чуть"),
    // (Conjunction, "а именно"),
    // (Conjunction, "а то"),
    // (Conjunction, "благодаря тому что"),
    // (Conjunction, "будто бы"),
    // (Conjunction, "в то время как"),
    // (Conjunction, "ввиду того что"),
    // (Conjunction, "всё же"),
    // (Conjunction, "да нет"),
    // (Conjunction, "для того чтобы"),
    // (Conjunction, "до того как"),
    // (Conjunction, "до того"),
    // (Conjunction, "ещё не ... как"),
    // (Conjunction, "и ... и"),
    // (Conjunction, "или ... или"),
    // (Conjunction, "как ... ни"),
    // (Conjunction, "как будто бы"),
    // (Conjunction, "как будто"),
    // (Conjunction, "как бы"),
    // (Conjunction, "как ни"),
    // (Conjunction, "как только"),
    // (Conjunction, "либо ... либо"),
    // (Conjunction, "между тем как"),
    // (Conjunction, "настолько ... насколько"),
    // (Conjunction, "не ... как"),
    // (Conjunction, "не настолько ... насколько"),
    // (Conjunction, "не так ... как"),
    // (Conjunction, "не то ... не то"),
    // (Conjunction, "не то что"),
    // (Conjunction, "не то чтобы"),
    // (Conjunction, "не только ... но и"),
    // (Conjunction, "несмотря на то что"),
    // (Conjunction, "ни ... ни"),
    // (Conjunction, "оттого что"),
    // (Conjunction, "перед тем как"),
    // (Conjunction, "пока .. не"),
    // (Conjunction, "пока не"),
    // (Conjunction, "после того как"),
    // (Conjunction, "потому что"),
    // (Conjunction, "правда ... но"),
    // (Conjunction, "прежде чем"),
    // (Conjunction, "разве только"),
    // (Conjunction, "разве что"),
    // (Conjunction, "с тех пор как"),
    // (Conjunction, "так как"),
    // (Conjunction, "так что"),
    // (Conjunction, "то ... то"),
    // (Conjunction, "то есть"),
    // (Conjunction, "чем ... тем"),
    // (Conjunction, "чтобы не"),

    (Particle, "будто"),
    (Particle, "бы"),
    (Particle, "ведь"),
    (Particle, "ведь"),
    (Particle, "вон"),
    (Particle, "вот"),
    (Particle, "да"),
    (Particle, "даже"),
    (Particle, "есть"),
    (Particle, "же"),
    (Particle, "здесь"),
    (Particle, "именно"),
    (Particle, "лишь"),
    (Particle, "не"),
    (Particle, "нет"),
    (Particle, "неужели"),
    (Particle, "ни"),
    (Particle, "ну"),
    (Particle, "ну"),
    (Particle, "пожалуй"),
    (Particle, "почти"),
    (Particle, "просто"),
    (Particle, "разве"),
    (Particle, "словно"),
    (Particle, "там"),
    (Particle, "только"),
    (Particle, "угодно"),
    (Particle, "уж"),
    (Particle, "хоть"),
    (Particle, "хотя"),
    (Particle, "чуть"),
    (Particle, "это"),
    // (Particle, "вряд ли"),
    // (Particle, "да ну"),
    // (Particle, "даже и"),
    // (Particle, "едва ли"),
    // (Particle, "ещё бы"),
    // (Particle, "как будто"),
    // (Particle, "как раз"),
    // (Particle, "лишь только"),
    // (Particle, "ни ... ни"),
    // (Particle, "ну да"),
    // (Particle, "ну и"),
    // (Particle, "только лишь"),
    // (Particle, "хоть бы"),
    // (Particle, "хотя бы"),
    // (Particle, "чуть ли не"),
    // (Particle, "чуть ли"),
    // (Particle, "чуть не"),
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

    for &(pos, word) in FUNCTION_WORDS { lang.add_pos(word, pos); }

    lang
}


#[cfg(test)]
mod tests {
    use crate::utils::{to_vec, to_str};
    use super::PartOfSpeech;
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