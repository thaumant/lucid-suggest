mod utils;
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
use std::collections::HashMap;
use rust_stemmers::Stemmer;
use crate::tokenization::PartOfSpeech;
use normalize::Normalize;


const BUFFER_CAPACITY: usize = 2;


pub struct Lang {
    stem_buffer:  RefCell<String>,
    norm_buffer1: RefCell<Vec<char>>,
    norm_buffer2: RefCell<Vec<char>>,
    pos_map:      HashMap<Vec<char>, PartOfSpeech>,
    compose_map:  HashMap<Vec<char>, Vec<char>>,
    reduce_map:   HashMap<Vec<char>, Vec<char>>,
    stemmer:      Stemmer,
}


impl Lang {
    pub fn new(
        pos_map:     HashMap<Vec<char>, PartOfSpeech>,
        compose_map: HashMap<Vec<char>, Vec<char>>,
        reduce_map:  HashMap<Vec<char>, Vec<char>>,
        stemmer:     Stemmer
    ) -> Self {
        let stem_buffer  = RefCell::new(String::with_capacity(BUFFER_CAPACITY));
        let norm_buffer1 = RefCell::new(Vec::with_capacity(BUFFER_CAPACITY));
        let norm_buffer2 = RefCell::new(Vec::with_capacity(BUFFER_CAPACITY));
        Self { stem_buffer, norm_buffer1, norm_buffer2, pos_map, compose_map, reduce_map, stemmer }
    }

    pub fn stem(&self, word: &[char]) -> usize {
        let buffer = &mut *self.stem_buffer.borrow_mut();
        buffer.clear();
        buffer.extend(word.iter());
        let stem = self.stemmer.stem(&buffer);
        stem.chars().count()
    }

    pub fn get_pos(&self, word: &[char]) -> Option<PartOfSpeech> {
        self.pos_map.get(word).cloned()
    }

    pub fn utf_compose(&self, word: &[char]) -> Option<Vec<char>> {
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

    pub fn utf_reduce(&self, word: &[char]) -> Option<(Vec<char>, Vec<char>)> {
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
    use std::collections::HashMap;
    use rust_stemmers::{Stemmer, Algorithm};
    use insta::assert_debug_snapshot;
    use super::Lang;
    use super::utils::compile_utf_map;

    fn get_lang() -> Lang {
        let stemmer  = Stemmer::create(Algorithm::English);
        let compose_map = compile_utf_map(&[("ó", "ó")]);
        let reduce_map  = compile_utf_map(&[
            ("ó", "o"),
            ("ß", "ss"),
        ]);
        let lang = Lang::new(HashMap::new(), compose_map, reduce_map, stemmer);
        lang
    }

    #[test]
    fn utf_compose_nfc() {
        let input  = "foóbar".chars().collect::<Vec<_>>();
        let output = get_lang().utf_compose(&input[..]);
        assert_debug_snapshot!(output);
    }

    #[test]
    fn utf_compose_nfd() {
        let input  = "foóbar".chars().collect::<Vec<_>>();
        let output = get_lang().utf_compose(&input[..]);
        assert_debug_snapshot!(output);
    }

    #[test]
    fn utf_reduce_reduced() {
        let input  = "foobar".chars().collect::<Vec<_>>();
        let output = get_lang().utf_reduce(&input[..]);
        assert_debug_snapshot!(output);
    }

    #[test]
    fn utf_reduce_nfc() {
        let input  = "foóbar".chars().collect::<Vec<_>>();
        let output = get_lang().utf_reduce(&input[..]);
        assert_debug_snapshot!(output);
    }

    #[test]
    fn utf_reduce_fill0() {
        let input  = "straße".chars().collect::<Vec<_>>();
        let output = get_lang().utf_reduce(&input[..]);
        assert_debug_snapshot!(output);
    }
}