mod pattern;
mod matching;
mod tokenizing;

use std::fmt;
use std::borrow::Cow;
pub use matching::WordMatch;
pub use pattern::{Chars, CharPattern};


#[derive(Clone, PartialEq)]
pub struct Text<'a> {
    pub source: &'a [char],
    pub words: Vec<Word<'a>>,
}


impl<'a> Text<'a> {
    pub fn new(source: &'a [char]) -> Text<'a> {
        Text { 
            source,
            words: vec![Word::new(source)] 
        }
    }

    pub fn is_empty(&self) -> bool {
        self.words.is_empty()
    }
}


impl<'a> fmt::Debug for Text<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Text {{")?;
        for word in &self.words {
            write!(f, " \"")?;
            for ch in word.chars.iter() {
                write!(f, "{}", ch)?;
            }
            write!(f, "\"")?;
            if !word.fin { write!(f, "..")?; }
        }
        write!(f, " }}")?;
        Ok(())
    }
}


#[derive(Clone, PartialEq)]
pub struct Word<'a> {
    pub source: &'a [char],
    pub slice:  (usize, usize),
    pub chars:  Cow<'a, [char]>,
    pub fin:    bool,
}


impl<'a> fmt::Debug for Word<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Word {{ \"")?;
        for ch in self.chars.iter() {
            write!(f, "{}", ch)?;
        }
        write!(f, "\"")?;
        if !self.fin { write!(f, "..")?; }
        write!(f, " }}")?;
        Ok(())
    }
}


impl<'a> Word<'a> {
    pub fn new(source: &'a [char]) -> Word<'a> {
        Word { 
            source,
            slice: (0, source.len()),
            chars: Cow::Borrowed(source),
            fin: true,
         }
    }

    pub fn len(&self) -> usize {
        self.chars.len()
    }

    pub fn is_empty(&self) -> bool {
        self.chars.is_empty()
    }
}
