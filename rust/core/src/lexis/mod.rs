mod pattern;
mod matching;
mod tokenizing;

use std::fmt;
use std::borrow::Cow;
pub use matching::{WordMatch, MatchSide};
pub use pattern::{Chars, CharPattern};


pub fn tokenize_query<'a>(source: &'a [char]) -> Text<Cow<'a, [char]>> {
    Text::new_cow(Cow::Borrowed(source))
        .fin(false)
        .split(&[Chars::Whitespaces, Chars::Control, Chars::Punctuation])
        .strip(&[Chars::NotAlphaNum])
        .lower()
}


pub fn tokenize_record<'a>(source: &'a [char]) -> Text<Cow<'a, [char]>> {
    Text::new_cow(Cow::Borrowed(source))
        .split(&[Chars::Whitespaces, Chars::Control])
        .strip(&[Chars::NotAlphaNum])
        .lower()
}


#[derive(Clone, PartialEq)]
pub struct Text<T: AsRef<[char]>> {
    pub source: T,
    pub words: Vec<Word<T>>,
}


impl<'a> Text<Cow<'a, [char]>> {
    pub fn new_cow(source: Cow<'a, [char]>) -> Text<Cow<'a, [char]>> {
        let cloned = match source {
            Cow::Borrowed(s) => Cow::Borrowed(&s[..]),
            Cow::Owned(ref s) => Cow::Owned(s[..].to_vec()),
        };
        Text {
            source,
            words: vec![Word::new_cow(cloned)],
        }
    }
}


impl Text<Vec<char>> {
    pub fn new_owned(source: Vec<char>) -> Text<Vec<char>> {
        let cloned = source.clone();
        Text {
            source: source,
            words: vec![Word::new_owned(cloned)],
        }
    }
}


impl<T: AsRef<[char]>> Text<T> {
    pub fn fin(mut self, fin: bool) -> Self {
        if let Some(word) = self.words.last_mut() {
            word.fin = fin;
        }
        self
    }

    pub fn is_empty(&self) -> bool {
        self.words.is_empty()
    }

    pub fn to_owned(&self) -> Text<Vec<char>> {
        Text {
            source: self.source.as_ref().to_vec(),
            words:  self.words.iter().map(|w| w.to_owned()).collect(),
        }
    }

    pub fn to_cow<'a>(&'a self) -> Text<Cow<'a, [char]>> {
        Text {
            source: Cow::Borrowed(self.source.as_ref()),
            words:  self.words.iter().map(|w| w.to_cow()).collect(),
        }
    }
}


impl<T: AsRef<[char]>> fmt::Debug for Text<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Text {{")?;
        for word in &self.words {
            write!(f, " \"")?;
            for ch in word.chars.as_ref().iter() {
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
pub struct Word<T: AsRef<[char]>> {
    pub source: T,
    pub slice:  (usize, usize),
    pub chars:  T,
    pub fin:    bool,
}


impl Word<Vec<char>> {
    pub fn new_owned(source: Vec<char>) -> Word<Vec<char>> {
        let len = source.len();
        let chars = source.clone();
        Word {
            source,
            slice: (0, len),
            chars,
            fin: true,
         }
    }
}


impl<'a> Word<Cow<'a, [char]>> {
    pub fn new_cow(source: Cow<'a, [char]>) -> Word<Cow<'a, [char]>> {
        let len = source.len();
        let chars = match source {
            Cow::Borrowed(s) => Cow::Borrowed(&s[..]),
            Cow::Owned(ref s) => Cow::Owned(s[..].to_vec()),
        };
        Word {
            source,
            slice: (0, len),
            chars,
            fin: true,
         }
    }
}


impl<T: AsRef<[char]>> Word<T> {
    pub fn fin(mut self, fin: bool) -> Self {
        self.fin = fin;
        self
    }

    pub fn len(&self) -> usize {
        self.chars.as_ref().len()
    }

    pub fn is_empty(&self) -> bool {
        self.chars.as_ref().is_empty()
    }

    pub fn to_owned(&self) -> Word<Vec<char>> {
        Word {
            source: self.source.as_ref().to_vec(),
            slice:  self.slice,
            chars:  self.chars.as_ref().to_vec(),
            fin:    self.fin,
        }
    }

    pub fn to_cow<'a>(&'a self) -> Word<Cow<'a, [char]>> {
        Word {
            source: Cow::Borrowed(self.source.as_ref()),
            slice:  self.slice,
            chars:  Cow::Borrowed(self.chars.as_ref()),
            fin:    self.fin,
        }
    }
}


impl<T: AsRef<[char]>> fmt::Debug for Word<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Word {{ \"")?;
        for ch in self.chars.as_ref().iter() {
            write!(f, "{}", ch)?;
        }
        write!(f, "\"")?;
        if !self.fin { write!(f, "..")?; }
        write!(f, " }}")?;
        Ok(())
    }
}
