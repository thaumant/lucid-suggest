use std::fmt;
use std::iter::empty;
pub use crate::lang::Lang;
pub use super::{Chars, CharPattern};
pub use super::pos::PartOfSpeech;


#[derive(PartialEq)]
pub struct Word<T: AsRef<[char]>> {
    pub source: T,
    pub slice:  (usize, usize),
    pub chars:  T,
    pub stem:   usize,
    pub pos:    Option<PartOfSpeech>,
    pub fin:    bool,
}


impl<T: AsRef<[char]>> Word<T> {
    pub fn is_primary(&self) -> bool {
        match self.pos {
            Some(PartOfSpeech::Article)     => false,
            Some(PartOfSpeech::Preposition) => false,
            Some(PartOfSpeech::Conjunction) => false,
            Some(PartOfSpeech::Particle)    => false,
            _ => true,
        }
    }
}


impl Word<Vec<char>> {
    pub fn from_vec(source: Vec<char>) -> Word<Vec<char>> {
        let len = source.len();
        let chars = source.clone();
        Word {
            source,
            slice: (0, len),
            chars,
            stem: len,
            pos: None,
            fin: true,
         }
    }

    pub fn from_str(source: &str) -> Word<Vec<char>> {
        let source = source.chars().collect::<Vec<_>>();
        let chars  = source.clone();
        let len    = source.len();
        Word {
            source,
            slice: (0, len),
            chars,
            stem: len,
            pos:  None,
            fin:  true,
         }
    }

    pub fn to_ref<'a>(&'a self) -> Word<&'a [char]> {
        Word {
            source: &self.source,
            slice:  self.slice,
            chars:  &self.chars,
            stem:   self.stem,
            pos:    self.pos,
            fin:    self.fin,
        }
    }

    pub fn split<'a, 'b, P: CharPattern>(&'a self, pattern: &'b P) -> WordSplit<'a, 'b, P> {
        WordSplit { word: self, offset: 0, pattern }
    }

    pub fn strip<P: CharPattern>(&mut self, pattern: &P) -> &mut Self {
        let left = self.chars.iter()
            .take_while(|&&ch| pattern.matches(ch))
            .count();
        let right = self.chars.iter()
            .rev()
            .take_while(|&&ch| pattern.matches(ch))
            .take(self.chars.len() - left)
            .count();

        self.chars.splice(self.chars.len() - right .., empty());
        self.chars.splice(0 .. left, empty());

        self.slice = (self.slice.0 + left, self.slice.1 - right);
        self.fin = self.fin || right != 0;
        self
    }

    pub fn stem(&mut self, lang: &Lang) -> &mut Self {
        self.stem = lang.stem(&self.chars);
        self
    }

    pub fn mark_pos(&mut self, lang: &Lang) -> &mut Self {
        self.pos = lang.get_pos(&self.chars);
        self
    }

    pub fn lower(&mut self) -> &mut Self {
        if self.chars.iter().any(|ch| ch.is_uppercase()) {
            for ch in &mut self.chars {
                *ch = ch.to_lowercase().next().unwrap();
            }
        }
        self
    }
}


impl<'a> Word<&'a [char]> {
    pub fn to_own(&self) -> Word<Vec<char>> {
        Word {
            source: self.source.to_vec(),
            slice:  self.slice,
            chars:  self.chars.to_vec(),
            stem:   self.stem,
            pos:    None,
            fin:    self.fin,
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
}


impl<T: AsRef<[char]>> fmt::Debug for Word<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Word {{ \"")?;
        let len  = self.chars.as_ref().len();
        let stem = self.stem;
        for (i, ch) in self.chars.as_ref().iter().enumerate() {
            write!(f, "{}", ch)?;
            if i == stem - 1 && i != len - 1 {
                write!(f, "|")?;
            }
        }
        write!(f, "\"")?;
        if !self.fin { write!(f, "..")?; }
        write!(f, " }}")?;
        Ok(())
    }
}


#[derive(Debug)]
pub struct WordSplit<'b, 'c, P: CharPattern> {
    word: &'b Word<Vec<char>>,
    offset: usize,
    pattern: &'c P,
}


impl<'b, 'c, P: CharPattern> Iterator for WordSplit<'b, 'c, P> {
    type Item = Word<&'b [char]>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.word.len() {
            return None;
        }

        self.offset += self.word.chars[self.offset ..]
            .iter()
            .take_while(|&&ch| self.pattern.matches(ch))
            .count();

        let len = self.word.chars[self.offset ..]
            .iter()
            .take_while(|&&ch| !self.pattern.matches(ch))
            .count();

        if len == 0 {
            return None;
        }

        let word = Word {
            source: &self.word.chars[self.offset .. self.offset + len],
            chars:  &self.word.source[self.offset .. self.offset + len],
            slice:  (0, len),
            stem:   len,
            pos:    None,
            fin:    self.word.fin || self.offset + len < self.word.len(),
        };

        self.offset += word.len();

        Some(word)
    }
}


#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use crate::lang::lang_english;
    use super::{Word, Chars, PartOfSpeech};

    use Chars::{
        Whitespaces,
        Punctuation,
    };

    #[test]
    fn word_split() {
        let w = Word::from_str(" Foo Bar, Baz; ");
        let s = w.split(&[Whitespaces, Punctuation]).collect::<Vec<_>>();
        assert_debug_snapshot!(s);
    }

    #[test]
    fn word_split_empty() {
        let w = Word::from_str(" ,;");
        let s = w.split(&[Whitespaces, Punctuation]).collect::<Vec<_>>();
        assert_debug_snapshot!(s);
    }

    #[test]
    fn word_split_unfinished() {
        let mut w1 = Word::from_str(" Foo Bar, Baz");
        let mut w2 = Word::from_str(" Foo Bar, Baz; ");
        w1.fin = false;
        w2.fin = false;
        let s1 = w1.split(&[Whitespaces, Punctuation]).collect::<Vec<_>>();
        let s2 = w2.split(&[Whitespaces, Punctuation]).collect::<Vec<_>>();
        assert_eq!(s1.last().unwrap().fin, false);
        assert_eq!(s2.last().unwrap().fin, true);
    }

    #[test]
    fn word_strip() {
        let mut w = Word::from_str(" Foo Bar, Baz; ");
        w.strip(&[Whitespaces, Punctuation]);
        assert_debug_snapshot!(&w);
        assert_debug_snapshot!(&w.slice);
    }

    #[test]
    fn word_strip_empty() {
        let mut w = Word::from_str(" ,;");
        w.strip(&[Whitespaces, Punctuation]);
        assert_debug_snapshot!(w);
    }

    #[test]
    fn word_strip_unfinished() {
        let mut w1 = Word::from_str(" Foo Bar, Baz");
        let mut w2 = Word::from_str(" Foo Bar, Baz; ");
        w1.fin = false;
        w2.fin = false;
        w1.strip(&[Whitespaces, Punctuation]);
        w2.strip(&[Whitespaces, Punctuation]);
        assert_eq!(w1.fin, false);
        assert_eq!(w2.fin, true);
    }

    #[test]
    fn word_stem() {
        let lang = lang_english();
        let mut w = Word::from_str("university");
        w.stem(&lang);
        assert_eq!(w.stem, 7);
    }

    #[test]
    fn word_pos() {
        let lang = lang_english();
        let mut w1 = Word::from_str("university");
        let mut w2 = Word::from_str("the");
        w1.mark_pos(&lang);
        w2.mark_pos(&lang);
        assert_eq!(w1.pos, None);
        assert_eq!(w2.pos, Some(PartOfSpeech::Article));
    }

    #[test]
    fn word_lower() {
        let mut w = Word::from_str(" Foo Bar, Baz; ");
        w.lower();
        assert_debug_snapshot!(w);
    }
}
