mod normalize;
mod lang_english;
mod lang_german;
mod lang_portuguese;
mod lang_russian;
mod lang_spanish;

pub use lang_german::lang_german;
pub use lang_english::lang_english;
pub use lang_spanish::lang_spanish;
pub use lang_portuguese::lang_portuguese;
pub use lang_russian::lang_russian;

use std::cell::RefCell;
use fnv::{FnvHashMap as HashMap};
use rust_stemmers::Stemmer;
use crate::utils::to_vec;
use crate::tokenization::PartOfSpeech;
use normalize::Normalize;


const BUFFER_CAPACITY: usize = 20;


pub struct Lang {
    stemmer:      Option<Stemmer>,
    pos_map:      HashMap<Vec<char>, PartOfSpeech>,
    compose_map:  HashMap<Vec<char>, Vec<char>>,
    reduce_map:   HashMap<Vec<char>, Vec<char>>,
    stem_buffer:  RefCell<String>,
    norm_buffer1: RefCell<Vec<char>>,
    norm_buffer2: RefCell<Vec<char>>,
}


impl Lang {
    pub fn new() -> Self {
        Self {
            stemmer:      None,
            pos_map:      HashMap::default(),
            compose_map:  HashMap::default(),
            reduce_map:   HashMap::default(),
            stem_buffer:  RefCell::new(String::with_capacity(BUFFER_CAPACITY)),
            norm_buffer1: RefCell::new(Vec::with_capacity(BUFFER_CAPACITY)),
            norm_buffer2: RefCell::new(Vec::with_capacity(BUFFER_CAPACITY)),
        }
    }

    pub fn set_stemmer(&mut self, stemmer: Option<Stemmer>) {
        self.stemmer = stemmer;
    }

    pub fn add_pos(&mut self, word: &str, pos: PartOfSpeech) {
        let source   = to_vec(word);
        let composed = self.unicode_compose(&source).unwrap_or(source);
        let reduced  = self.unicode_reduce(&composed);

        self.pos_map.insert(composed, pos);

        if let Some((_, reduced)) = reduced {
            self.pos_map.insert(reduced, pos);
        }
    }

    pub fn add_unicode_composition(&mut self, from: &str, to: &str) {
        self.compose_map.insert(to_vec(from), to_vec(to));
    }

    pub fn add_unicode_reduction(&mut self, from: &str, to: &str) {
        self.reduce_map.insert(to_vec(from), to_vec(to));
    }

    pub fn stem(&self, word: &[char]) -> usize {
        if let Some(stemmer) = &self.stemmer {
            let buffer = &mut *self.stem_buffer.borrow_mut();
            buffer.clear();
            buffer.extend(word.iter());
            let stem = stemmer.stem(&buffer);
            stem.chars().count()
        } else {
            word.len()
        }
    }

    pub fn get_pos(&self, word: &[char]) -> Option<PartOfSpeech> {
        self.pos_map.get(word).cloned()
    }

    pub fn unicode_compose(&self, word: &[char]) -> Option<Vec<char>> {
        let buffer = &mut *self.norm_buffer1.borrow_mut();
        buffer.clear();

        for (_, norm_chunk) in Normalize::new(word, &self.compose_map) {
            buffer.extend(norm_chunk);
        }

        if &buffer[..] == word {
            None
        } else {
            Some(buffer.clone())
        }
    }

    pub fn unicode_reduce(&self, word: &[char]) -> Option<(Vec<char>, Vec<char>)> {
        let buffer1 = &mut *self.norm_buffer1.borrow_mut();
        let buffer2 = &mut *self.norm_buffer2.borrow_mut();
        buffer1.clear();
        buffer2.clear();

        for (word_chunk, norm_chunk) in Normalize::new(word, &self.reduce_map) {
            buffer1.extend(word_chunk);
            buffer2.extend(norm_chunk);
            for _ in 0 .. norm_chunk.len() - word_chunk.len() {
                buffer1.push('\0');
            }
        }

        if &buffer2[..] == word {
            None
        } else {
            Some((buffer1.clone(), buffer2.clone()))
        }
    }
}


#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use crate::utils::to_vec;
    use crate::tokenization::PartOfSpeech;
    use super::Lang;

    fn get_lang() -> Lang {
        let mut lang = Lang::new();
        lang.add_unicode_composition("ó", "ó");
        lang.add_unicode_reduction("ó", "o");
        lang.add_unicode_reduction("õ", "oo");
        lang.add_pos("fóo", PartOfSpeech::Particle);
        lang
    }

    #[test]
    fn unicode_compose_nfc() {
        let input  = to_vec("foóbar");
        let output = get_lang().unicode_compose(&input[..]);
        assert_debug_snapshot!(output);
    }

    #[test]
    fn unicode_compose_nfd() {
        let input  = to_vec("foóbar");
        let output = get_lang().unicode_compose(&input[..]);
        assert_debug_snapshot!(output);
    }

    #[test]
    fn unicode_reduce_reduced() {
        let input  = to_vec("foobar");
        let output = get_lang().unicode_reduce(&input[..]);
        assert_debug_snapshot!(output);
    }

    #[test]
    fn unicode_reduce_nfc() {
        let input  = to_vec("foóbar");
        let output = get_lang().unicode_reduce(&input[..]);
        assert_debug_snapshot!(output);
    }

    #[test]
    fn unicode_reduce_fill0() {
        let input  = to_vec("fõbar");
        let output = get_lang().unicode_reduce(&input[..]);
        assert_debug_snapshot!(output);
    }

    #[test]
    fn get_pos_source() {
        let input = to_vec("fóo");
        let pos   = get_lang().get_pos(&input);
        assert_eq!(pos, Some(PartOfSpeech::Particle));
    }

    #[test]
    fn get_pos_reduced() {
        let input = to_vec("foo");
        let pos   = get_lang().get_pos(&input);
        assert_eq!(pos, Some(PartOfSpeech::Particle));
    }

    #[test]
    fn get_pos_unknown() {
        let input = to_vec("bar");
        let pos   = get_lang().get_pos(&input);
        assert_eq!(pos, None);
    }
}