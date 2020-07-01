#![allow(dead_code)]

use rust_stemmers::{Algorithm, Stemmer};
use crate::tokenization::PartOfSpeech;
use super::Lang;

use PartOfSpeech::{
    Article,
    Preposition,
    Conjunction,
};


const FUNCTION_WORDS: &[(PartOfSpeech, &'static str)] = &[
    (Article, "o"),
    (Article, "a"),
    (Article, "os"),
    (Article, "as"),
    (Article, "um"),
    (Article, "uma"),
    (Article, "uns"),
    (Article, "umas"),

    (Preposition, "abaixo"),
    (Preposition, "acima"),
    (Preposition, "além"),
    (Preposition, "antes"),
    (Preposition, "aproximadamente"),
    (Preposition, "aquela"),
    (Preposition, "aquele"),
    (Preposition, "aqueles"),
    (Preposition, "até"),
    (Preposition, "atrás"),
    (Preposition, "com"),
    (Preposition, "como"),
    (Preposition, "conforme"),
    (Preposition, "contra"),
    (Preposition, "de"),
    (Preposition, "depois"),
    (Preposition, "desde"),
    (Preposition, "distante"),
    (Preposition, "durante"),
    (Preposition, "em"),
    (Preposition, "entre"),
    (Preposition, "esta"),
    (Preposition, "estas"),
    (Preposition, "este"),
    (Preposition, "estes"),
    (Preposition, "exceto"),
    (Preposition, "fora"),
    (Preposition, "mais"),
    (Preposition, "mas"),
    (Preposition, "oposto"),
    (Preposition, "para"),
    (Preposition, "perto"),
    (Preposition, "por"),
    (Preposition, "próximo"),
    (Preposition, "que"),
    (Preposition, "sem"),
    (Preposition, "sob"),
    (Preposition, "sobre"),
    (Preposition, "via"),
    // (Preposition, "além de"),
    // (Preposition, "antes de"),
    // (Preposition, "ao lado de"),
    // (Preposition, "apesar de"),
    // (Preposition, "apesar de"),
    // (Preposition, "através de"),
    // (Preposition, "através de"),
    // (Preposition, "bem como"),
    // (Preposition, "dentro de"),
    // (Preposition, "dentro de"),
    // (Preposition, "dentro de"),
    // (Preposition, "devido a"),
    // (Preposition, "diferente de"),
    // (Preposition, "em cima de"),
    // (Preposition, "em direção a"),
    // (Preposition, "em nome de"),
    // (Preposition, "em torno de"),
    // (Preposition, "em vez de"),
    // (Preposition, "fora de"),
    // (Preposition, "fora de"),
    // (Preposition, "longe de"),
    // (Preposition, "na frente de"),
    // (Preposition, "para baixo"),
    // (Preposition, "para cima"),
    // (Preposition, "perto de"),
    // (Preposition, "por causa de"),
    // (Preposition, "próximo a"),
    // (Preposition, "próximo de"),
    // (Preposition, "tanto quanto"),

    (Conjunction, "agora"),
    (Conjunction, "como"),
    (Conjunction, "contudo"),
    (Conjunction, "e"),
    (Conjunction, "enquanto"),
    (Conjunction, "então"),
    (Conjunction, "logo"),
    (Conjunction, "mas"),
    (Conjunction, "nem"),
    (Conjunction, "ou"),
    (Conjunction, "para"),
    (Conjunction, "pois"),
    (Conjunction, "por"),
    (Conjunction, "porém"),
    (Conjunction, "porque"),
    (Conjunction, "portanto"),
    (Conjunction, "quando"),
    (Conjunction, "se"),
    (Conjunction, "todavia"),
    // (Conjunction, "a fim de"),
    // (Conjunction, "ainda assim"),
    // (Conjunction, "ainda que"),
    // (Conjunction, "apesar disso"),
    // (Conjunction, "assim que"),
    // (Conjunction, "como se"),
    // (Conjunction, "enquanto que"),
    // (Conjunction, "não só ... como também"),
    // (Conjunction, "no entanto"),
    // (Conjunction, "ora ... ora"),
    // (Conjunction, "ou ... ou"),
    // (Conjunction, "por conseguinte"),
    // (Conjunction, "por isso"),
    // (Conjunction, "quer ... quer"),
    // (Conjunction, "sempre que"),
    // (Conjunction, "tanto ... como"),
    // (Conjunction, "visto que"),
];

const UTF_COMPOSE_MAP: &[(&'static str, &'static str)] = &[
    ("Ç", "Ç"), // cedilla
    ("ç", "ç"),
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
    ("Â", "Â"), // circumflex accent
    ("Ê", "Ê"),
    ("Ô", "Ô"),
    ("â", "â"),
    ("ê", "ê"),
    ("ô", "ô"),
    ("Ã", "Ã"), // tilde
    ("Õ", "Õ"),
    ("ã", "ã"),
    ("õ", "õ"),
    ("À", "À"), // grave accent
    ("È", "È"),
    ("Ì", "Ì"),
    ("Ò", "Ò"),
    ("Ù", "Ù"),
    ("à", "à"),
    ("è", "è"),
    ("ì", "ì"),
    ("ò", "ò"),
    ("ù", "ù"),
];

const UTF_REDUCE_MAP: &[(&'static str, &'static str)] = &[
    ("Ç", "C"), // cedilla
    ("ç", "c"),
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
    ("Â", "A"), // circumflex accent
    ("Ê", "E"),
    ("Ô", "O"),
    ("â", "a"),
    ("ê", "e"),
    ("ô", "o"),
    ("Ã", "A"), // tilde
    ("Õ", "O"),
    ("ã", "a"),
    ("õ", "o"),
    ("À", "A"), // grave accent
    ("È", "E"),
    ("Ì", "I"),
    ("Ò", "O"),
    ("Ù", "U"),
    ("à", "a"),
    ("è", "e"),
    ("ì", "i"),
    ("ò", "o"),
    ("ù", "u"),
];


pub fn lang_portuguese() -> Lang {
    let mut lang = Lang::new();

    lang.set_stemmer(Some(Stemmer::create(Algorithm::Portuguese)));

    for (from, to) in UTF_COMPOSE_MAP { lang.add_unicode_composition(from, to); }
    for (from, to) in UTF_REDUCE_MAP  { lang.add_unicode_reduction(from, to); }

    for &(pos, word) in FUNCTION_WORDS { lang.add_pos(word, pos); }

    lang
}


#[cfg(test)]
mod tests {
    use crate::utils::{to_vec, to_str};
    use crate::tokenization::PartOfSpeech;
    use super::{lang_portuguese, UTF_COMPOSE_MAP, UTF_REDUCE_MAP};

    #[test]
    pub fn stem() {
        let lang = lang_portuguese();
        let w    = to_vec("quilométricas");
        assert_eq!(lang.stem(&w), 9);
    }

    #[test]
    pub fn get_pos() {
        let lang = lang_portuguese();
        let w1   = to_vec("quilométricas");
        let w2   = to_vec("uma");
        assert_eq!(lang.get_pos(&w1), None);
        assert_eq!(lang.get_pos(&w2), Some(PartOfSpeech::Article));
    }

    #[test]
    fn unicode_compose() {
        let lang = lang_portuguese();

        let source1 = to_vec("conforme");
        let norm1   = lang.unicode_compose(&source1);
        assert_eq!(norm1, None);

        let source2 = to_vec("Conceição");
        let norm2   = lang.unicode_compose(&source2).unwrap();
        assert_eq!(to_str(&norm2), "Conceição");
        assert_eq!(norm2.len(), source2.len() - 2);
    }

    #[test]
    fn unicode_reduce() {
        let lang = lang_portuguese();

        let source1 = to_vec("conforme");
        let norm1   = lang.unicode_reduce(&source1);
        assert_eq!(norm1, None);

        let source2 = to_vec("Conceição");
        let (padded2, norm2) = lang.unicode_reduce(&source2).unwrap();
        assert_eq!(to_str(&padded2), to_str(&source2));
        assert_eq!(to_str(&norm2), "Conceicao");
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