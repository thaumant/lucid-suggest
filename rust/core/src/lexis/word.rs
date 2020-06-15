use std::fmt;
use std::iter::empty;
pub use crate::lang::Lang;
pub use super::{Chars, CharPattern};
pub use super::pos::PartOfSpeech;


#[derive(PartialEq)]
pub struct Word<T: AsRef<[char]>> {
    pub chars: T,
    pub place: (usize, usize),
    pub stem:  usize,
    pub pos:   Option<PartOfSpeech>,
    pub fin:   bool,
}


impl Word<Vec<char>> {
    pub fn from_vec(source: Vec<char>) -> Word<Vec<char>> {
        let len = source.len();
        let chars = source.clone();
        Word {
            chars,
            place: (0, len),
            stem:  len,
            pos:   None,
            fin:   true,
        }
    }

    pub fn from_str(source: &str) -> Word<Vec<char>> {
        let source = source.chars().collect::<Vec<_>>();
        let chars  = source.clone();
        let len    = source.len();
        Word {
            chars,
            place: (0, len),
            stem:  len,
            pos:   None,
            fin:   true,
        }
    }
}


impl Word<Vec<char>> {
    pub fn to_ref<'a>(&'a self) -> Word<&'a [char]> {
        Word {
            chars: &self.chars,
            place: self.place,
            stem:  self.stem,
            pos:   self.pos,
            fin:   self.fin,
        }
    }
}


impl<'a> Word<&'a [char]> {
    pub fn to_own(&self) -> Word<Vec<char>> {
        Word {
            chars: self.chars.to_vec(),
            place: self.place,
            stem:  self.stem,
            pos:   self.pos,
            fin:   self.fin,
        }
    }
}


impl<T: AsRef<[char]>> Word<T> {
    pub fn len(&self) -> usize {
        self.chars.as_ref().len()
    }

    pub fn is_empty(&self) -> bool {
        self.chars.as_ref().is_empty()
    }

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
    pub fn fin(mut self, fin: bool) -> Self {
        self.fin = fin;
        self
    }

    pub fn split<'a, 'b, P: CharPattern>(&'a self, pattern: &'b P) -> WordSplit<'a, 'b, P> {
        WordSplit { word: self, pattern, offset: 0 }
    }

    pub fn strip<P: CharPattern>(&mut self, pattern: &P) -> &mut Self {
        let Self { chars, place, .. } = self;
        let left = chars.iter()
            .take_while(|&&ch| pattern.matches(ch))
            .count();
        let right = chars.iter()
            .rev()
            .take_while(|&&ch| pattern.matches(ch))
            .take(chars.len() - left)
            .count();
        if left  > 0 { chars.splice(..left, empty()); }
        if right > 0 { chars.splice(chars.len() - right .., empty()); }
        place.0 += left;
        place.1 -= right;
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
        let Self { word, offset, pattern } = self;

        if *offset >= word.len() {
            return None;
        }

        *offset += word.chars[*offset ..]
            .iter()
            .take_while(|&&ch| pattern.matches(ch))
            .count();

        let len = word.chars[*offset ..]
            .iter()
            .take_while(|&&ch| !pattern.matches(ch))
            .count();

        if len == 0 {
            return None;
        }

        let splitted = Word {
            chars:  &word.chars[*offset .. *offset + len],
            place:  (word.place.0 + *offset, word.place.0 + *offset + len),
            stem:   len,
            pos:    None,
            fin:    word.fin || *offset + len < word.len(),
        };

        *offset += splitted.len();

        Some(splitted)
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
    fn word_split_place() {
        let mut  w = Word::from_str(" Foo Bar, Baz; ");

        let s = w.split(&[Whitespaces, Punctuation]).collect::<Vec<_>>();
        assert_eq!(s[0].place, (1, 4));
        assert_eq!(s[1].place, (5, 8));
        assert_eq!(s[2].place, (10, 13));

        w.place = (10, 10 + w.len());
        let s = w.split(&[Whitespaces, Punctuation]).collect::<Vec<_>>();
        assert_eq!(s[0].place, (11, 14));
        assert_eq!(s[1].place, (15, 18));
        assert_eq!(s[2].place, (20, 23));
    }

    #[test]
    fn word_split_empty() {
        let w = Word::from_str(" ,;");
        let s = w.split(&[Whitespaces, Punctuation]).collect::<Vec<_>>();
        assert_debug_snapshot!(s);
    }

    #[test]
    fn word_split_unfinished() {
        let w1 = Word::from_str(" Foo Bar, Baz").fin(false);
        let w2 = Word::from_str(" Foo Bar, Baz; ").fin(false);
        let s1 = w1.split(&[Whitespaces, Punctuation]).collect::<Vec<_>>();
        let s2 = w2.split(&[Whitespaces, Punctuation]).collect::<Vec<_>>();
        assert_eq!(s1.last().unwrap().fin, false);
        assert_eq!(s2.last().unwrap().fin, true);
    }

    #[test]
    fn word_strip() {
        let mut w = Word::from_str(" Foo; ");
        w.strip(&[Whitespaces, Punctuation]);
        assert_debug_snapshot!(&w);
    }

    #[test]
    fn word_strip_place() {
        let mut w = Word::from_str(" Foo; ");
        w.strip(&[Whitespaces, Punctuation]);
        assert_eq!(w.place, (1, 4));

        let mut w = Word::from_str(" Foo; ");
        w.place = (10, 16);
        w.strip(&[Whitespaces, Punctuation]);
        assert_eq!(w.place, (11, 14));
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
