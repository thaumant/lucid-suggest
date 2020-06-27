#![allow(dead_code)]

use rust_stemmers::{Algorithm, Stemmer};
use crate::tokenization::PartOfSpeech;
use super::Lang;


const ARTICLES: &[&'static str] = &[
    "o",
    "a",
    "os",
    "as",
    "um",
    "uma",
    "uns",
    "umas",
];

const PREPOSITIONS: &[&'static str] = &[
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

const CONJUNCTIONS: &[&'static str] = &[
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

const PARTICLES: &[&'static str] = &[
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