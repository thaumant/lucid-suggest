mod lang_english;
mod lang_german;

pub use lang_english::lang_english;
pub use lang_german::lang_german;

use std::cell::RefCell;
use std::collections::HashMap;
use rust_stemmers::Stemmer;
use crate::lexis::PartOfSpeech;


pub struct Lang {
    char_buffer: RefCell<String>,
    stemmer:     Stemmer,
    pos_map:     HashMap<Vec<char>, PartOfSpeech>,
}


impl Lang {
    pub fn new(pos_map: HashMap<Vec<char>, PartOfSpeech>, stemmer: Stemmer) -> Self {
        let char_buffer = RefCell::new(String::with_capacity(20));
        Self { char_buffer, stemmer, pos_map }
    }

    pub fn stem(&self, word: &[char]) -> usize {
        let buffer = &mut *self.char_buffer.borrow_mut();
        buffer.clear();
        buffer.extend(word.iter());
        let stem = self.stemmer.stem(&buffer);
        stem.chars().count()
    }

    pub fn get_pos(&self, word: &[char]) -> Option<PartOfSpeech> {
        self.pos_map.get(word).cloned()
    }
}


