#![allow(dead_code)]

use rust_stemmers::{Algorithm, Stemmer};
use super::{CharClass, PartOfSpeech};
use super::Lang;
use super::constants::CHAR_CLASSES_LATIN;

use PartOfSpeech::{
    Article,
    Preposition,
    Conjunction,
    Particle,
};

use CharClass::{
    Consonant,
    Vowel,
};

const FUNCTION_WORDS: &[(PartOfSpeech, &'static str)] = &[
    (Article, "le"),
    (Article, "la"),
    (Article, "les"),
    (Article, "l"),
    (Article, "un"),
    (Article, "une"),
    (Article, "des"),
    (Article, "du"),
    (Article, "de"),

    (Preposition, "à"),
    (Preposition, "après"),
    (Preposition, "au-delà"),
    (Preposition, "au-dessus"),
    (Preposition, "avant"),
    (Preposition, "avec"),
    (Preposition, "ces"),
    (Preposition, "cette"),
    (Preposition, "ceux"),
    (Preposition, "comme"),
    (Preposition, "contre"),
    (Preposition, "dans"),
    (Preposition, "de"),
    (Preposition, "depuis"),
    (Preposition, "derrière"),
    (Preposition, "entre"),
    (Preposition, "jusqu'à"),
    (Preposition, "mais"),
    (Preposition, "malgré"),
    (Preposition, "opposé"),
    (Preposition, "par"),
    (Preposition, "plus"),
    (Preposition, "pour"),
    (Preposition, "prochain"),
    (Preposition, "que"),
    (Preposition, "qui"),
    (Preposition, "sans"),
    (Preposition, "sauf"),
    (Preposition, "selon"),
    (Preposition, "sous"),
    (Preposition, "sous"),
    (Preposition, "sur"),
    (Preposition, "tour"),
    (Preposition, "vers"),
    (Preposition, "via"),
    // (Preposition, "à cause de"),
    // (Preposition, "à côté de"),
    // (Preposition, "à côté de"),
    // (Preposition, "à l'exception de"),
    // (Preposition, "à l'extérieur"),
    // (Preposition, "à l'intérieur"),
    // (Preposition, "à partir de"),
    // (Preposition, "à travers"),
    // (Preposition, "au cours de"),
    // (Preposition, "au lieu de"),
    // (Preposition, "au nom de"),
    // (Preposition, "au sommet de"),
    // (Preposition, "aussi bien que"),
    // (Preposition, "autant que"),
    // (Preposition, "autour de"),
    // (Preposition, "contrairement à"),
    // (Preposition, "en dehors des"),
    // (Preposition, "en dépit de"),
    // (Preposition, "en dessous"),
    // (Preposition, "en face de"),
    // (Preposition, "en plus de"),
    // (Preposition, "en raison de"),
    // (Preposition, "jusqu'à ce que"),
    // (Preposition, "l'intérieur de"),
    // (Preposition, "loin d'être"),
    // (Preposition, "près de"),
    // (Preposition, "près de"),
    // (Preposition, "près de"),
    // (Preposition, "vers le bas"),

    (Conjunction, "car"),
    (Conjunction, "comme"),
    (Conjunction, "donc"),
    (Conjunction, "ensuite"),
    (Conjunction, "et"),
    (Conjunction, "lorsque"),
    (Conjunction, "mais"),
    (Conjunction, "ni"),
    (Conjunction, "or"),
    (Conjunction, "ou"),
    (Conjunction, "plus"),
    (Conjunction, "pourquoi"),
    (Conjunction, "puis"),
    (Conjunction, "puisque"),
    (Conjunction, "quand"),
    (Conjunction, "que"),
    (Conjunction, "quoique"),
    (Conjunction, "si"),
    // (Conjunction, "à condition que"),
    // (Conjunction, "à mesure que"),
    // (Conjunction, "à moins que"),
    // (Conjunction, "à supposer que"),
    // (Conjunction, "afin que"),
    // (Conjunction, "ainsi que"),
    // (Conjunction, "alors que"),
    // (Conjunction, "après que"),
    // (Conjunction, "au cas où"),
    // (Conjunction, "aussitôt que"),
    // (Conjunction, "avant que"),
    // (Conjunction, "bien que"),
    // (Conjunction, "c’est-à-dire que"),
    // (Conjunction, "comme si"),
    // (Conjunction, "dans l’hypothèse où"),
    // (Conjunction, "dans la mesure où"),
    // (Conjunction, "de crainte que"),
    // (Conjunction, "de façon que"),
    // (Conjunction, "de manière que"),
    // (Conjunction, "de même que"),
    // (Conjunction, "de peur que"),
    // (Conjunction, "de sorte que"),
    // (Conjunction, "depuis que"),
    // (Conjunction, "dès que"),
    // (Conjunction, "en admettant que"),
    // (Conjunction, "en attendant que"),
    // (Conjunction, "encore que"),
    // (Conjunction, "jusqu’à ce que"),
    // (Conjunction, "même si"),
    // (Conjunction, "ou bien"),
    // (Conjunction, "parce que"),
    // (Conjunction, "pendant que"),
    // (Conjunction, "pour que"),
    // (Conjunction, "pourvu que"),
    // (Conjunction, "quand bien même"),
    // (Conjunction, "quoi que"),
    // (Conjunction, "sans que"),
    // (Conjunction, "selon que"),
    // (Conjunction, "sitôt que"),
    // (Conjunction, "suivant que"),
    // (Conjunction, "supposé que"),
    // (Conjunction, "tandis que"),
    // (Conjunction, "tant que"),
    // (Conjunction, "une fois que"),
    // (Conjunction, "vu que"),
    // (Conjunction, "et ... et"),
    // (Conjunction, "ni ... ni"),
    // (Conjunction, "ou ... ou"),
    // (Conjunction, "ou bien ... ou bien"),
    // (Conjunction, "plus ... plus"),
    // (Conjunction, "soit ... soit"),

    (Particle, "ne"),
    (Particle, "ô"),
];


const CHAR_CLASSES: &[(CharClass, char)] = &[
    // acute accent
    (Vowel, 'É'),
    (Vowel, 'é'),
    // grave accent
    (Vowel, 'À'),
    (Vowel, 'È'),
    (Vowel, 'Ù'),
    (Vowel, 'à'),
    (Vowel, 'è'),
    (Vowel, 'ù'),
    // circumflex
    (Vowel, 'Â'),
    (Vowel, 'Ê'),
    (Vowel, 'Î'),
    (Vowel, 'Ô'),
    (Vowel, 'Û'),
    (Vowel, 'â'),
    (Vowel, 'ê'),
    (Vowel, 'î'),
    (Vowel, 'ô'),
    (Vowel, 'û'),
    // diaeresis
    (Vowel, 'Ë'),
    (Vowel, 'Ï'),
    (Vowel, 'Ü'),
    (Vowel, 'Ÿ'),
    (Vowel, 'ë'),
    (Vowel, 'ï'),
    (Vowel, 'ü'),
    (Vowel, 'ÿ'),
    // cedilla
    (Consonant, 'Ç'),
    (Consonant, 'ç'),
    // tilde
    (Consonant, 'Ñ'),
    (Consonant, 'ñ'),
    // ligatures
    (Vowel, 'Æ'),
    (Vowel, 'æ'),
    (Vowel, 'Œ'),
    (Vowel, 'œ'),
    (Vowel, 'Ø'),
    (Vowel, 'ø'),
];


const UTF_COMPOSE_MAP: &[(&'static str, &'static str)] = &[
    // acute accent
    ("É", "É"),
    ("é", "é"),
    // grave accent
    ("À", "À"),
    ("È", "È"),
    ("Ù", "Ù"),
    ("à", "à"),
    ("è", "è"),
    ("ù", "ù"),
    // circumflex
    ("Â", "Â"),
    ("Ê", "Ê"),
    ("Î", "Î"),
    ("Ô", "Ô"),
    ("Û", "Û"),
    ("â", "â"),
    ("ê", "ê"),
    ("î", "î"),
    ("ô", "ô"),
    ("û", "û"),
    // diaeresis
    ("Ë", "Ë"),
    ("Ï", "Ï"),
    ("Ü", "Ü"),
    ("Ÿ", "Ÿ"),
    ("ë", "ë"),
    ("ï", "ï"),
    ("ü", "ü"),
    ("ÿ", "ÿ"),
    // cedilla
    ("Ç", "Ç"),
    ("ç", "ç"),
    // tilde
    ("Ñ", "Ñ"),
    ("ñ", "ñ"),
];


const UTF_REDUCE_MAP: &[(&'static str, &'static str)] = &[
    // acute accent
    ("É", "E"),
    ("é", "e"),
    // grave accent
    ("À", "A"),
    ("È", "E"),
    ("Ù", "U"),
    ("à", "a"),
    ("è", "e"),
    ("ù", "u"),
    // circumflex
    ("Â", "A"),
    ("Ê", "E"),
    ("Î", "I"),
    ("Ô", "O"),
    ("Û", "U"),
    ("â", "a"),
    ("ê", "e"),
    ("î", "i"),
    ("ô", "o"),
    ("û", "u"),
    // diaeresis
    ("Ë", "E"),
    ("Ï", "I"),
    ("Ü", "U"),
    ("Ÿ", "Y"),
    ("ë", "e"),
    ("ï", "i"),
    ("ü", "u"),
    ("ÿ", "y"),
    // cedilla
    ("Ç", "C"),
    ("ç", "c"),
    // tilde
    ("Ñ", "N"),
    ("ñ", "n"),
    // ligatures
    ("Æ", "AE"),
    ("æ", "ae"),
    ("Œ", "OE"),
    ("œ", "oe"),
    ("Ø", "OE"),
    ("ø", "oe"),
];


pub fn lang_french() -> Lang {
    let mut lang = Lang::new();

    lang.set_stemmer(Some(Stemmer::create(Algorithm::French)));

    for (from, to)   in UTF_COMPOSE_MAP    { lang.add_unicode_composition(from, to); }
    for (from, to)   in UTF_REDUCE_MAP     { lang.add_unicode_reduction(from, to); }

    for &(pos, word) in FUNCTION_WORDS     { lang.add_pos(word, pos); }

    for &(class, ch) in CHAR_CLASSES_LATIN { lang.add_char_class(ch, class); }
    for &(class, ch) in CHAR_CLASSES       { lang.add_char_class(ch, class); }

    lang
}


#[cfg(test)]
mod tests {
    use crate::utils::{to_vec, to_str};
    use super::{PartOfSpeech, CharClass};
    use super::{lang_french, UTF_COMPOSE_MAP, UTF_REDUCE_MAP};

    #[test]
    pub fn stem() {
        let lang = lang_french();
        let w    = to_vec("université");
        assert_eq!(lang.stem(&w), 7);
    }

    #[test]
    pub fn get_pos() {
        let lang = lang_french();
        let w1   = to_vec("université");
        let w2   = to_vec("les");
        assert_eq!(lang.get_pos(&w1), None);
        assert_eq!(lang.get_pos(&w2), Some(PartOfSpeech::Article));
    }

    #[test]
    fn unicode_compose() {
        let lang    = lang_french();

        let source1 = to_vec("université");
        let norm1   = lang.unicode_compose(&source1);
        assert_eq!(norm1, None);

        let source2 = to_vec("château");
        let norm2   = lang.unicode_compose(&source2).unwrap();
        assert_eq!(to_str(&norm2), "château");
        assert_eq!(norm2.len(), source2.len() - 1);
    }

    #[test]
    fn unicode_reduce() {
        let lang = lang_french();

        let source1 = to_vec("univers");
        let norm1   = lang.unicode_reduce(&source1);
        assert_eq!(norm1, None);

        let source2 = to_vec("château");
        let (padded2, norm2) = lang.unicode_reduce(&source2).unwrap();
        assert_eq!(to_str(&padded2), to_str(&source2));
        assert_eq!(to_str(&norm2), "chateau");
        assert_eq!(norm2.len(), source2.len());

        let source3 = to_vec("sœur");
        let (padded3, norm3) = lang.unicode_reduce(&source3).unwrap();
        assert_eq!(to_str(&padded3), "sœ\0ur");
        assert_eq!(to_str(&norm3), "soeur");
        assert_eq!(norm3.len(), source3.len() + 1);
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
            let ligature = ["Æ", "æ", "Œ", "œ", "Ø", "ø"].contains(&normal);
            let len2 = if ligature { 2 } else { 1 };
            assert_eq!(normal .chars().count(), 1, "UTF_REDUCE_MAP['{}'] != {}", normal, 1);
            assert_eq!(reduced.chars().count(), len2, "UTF_REDUCE_MAP['{}'].len() != {}", reduced, len2);
        }
    }

    #[test]
    fn get_char_class() {
        let lang = lang_french();
        assert_eq!(lang.get_char_class('n'), Some(CharClass::Consonant));
        assert_eq!(lang.get_char_class('a'), Some(CharClass::Vowel));
        assert_eq!(lang.get_char_class('ô'), Some(CharClass::Vowel));
        assert_eq!(lang.get_char_class('œ'), Some(CharClass::Vowel));
        assert_eq!(lang.get_char_class('%'), None);
    }
}