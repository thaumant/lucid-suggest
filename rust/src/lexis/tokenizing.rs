use std::borrow::Cow;
use std::iter::empty;
use super::{Word, Text, CharPattern};


impl<'a> Text<'a> {
    pub fn fin(mut self, fin: bool) -> Self {
        if let Some(word) = self.words.last_mut() {
            word.fin = fin;
        }
        self
    }

    pub fn split<P: CharPattern>(mut self, pattern: &P) -> Self {
        let mut words = Vec::with_capacity(self.words.len());
        for word in &self.words {
            for splitted in word.split(pattern) {
                words.push(splitted);
            }
        }
        self.words = words;
        self
    }

    pub fn strip<P: CharPattern>(mut self, pattern: &P) -> Self {
        for word in &mut self.words {
            word.strip(pattern);
        }
        self.words.retain(|w| w.len() > 0);
        self
    }

    pub fn lower(mut self) -> Self {
        for word in &mut self.words {
            word.lower();
        }
        self
    }
}


impl<'a> Word<'a> {
    pub fn split<'b, 'c, P: CharPattern>(&'b self, pattern: &'c P) -> WordSplit<'a, 'b, 'c, P> {
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
        match &mut self.chars {
            Cow::Borrowed(c) => {
                self.chars = Cow::Borrowed(&c[left .. c.len() - right]);
            },
            Cow::Owned(c) => {
                c.splice(c.len() - right .., empty());
                c.splice(0 .. left, empty());
            },
        }
        self.slice = (self.slice.0 + right, self.slice.1 - left);
        self.fin = self.fin || right != 0;
        self
    }

    pub fn lower(&mut self) -> &mut Self {
        if self.chars.iter().any(|ch| ch.is_uppercase()) {
            for ch in self.chars.to_mut() {
                *ch = ch.to_lowercase().next().unwrap();
            }
        }
        self
    }
}


#[derive(Debug)]
pub struct WordSplit<'a, 'b, 'c, P: CharPattern> {
    word: &'b Word<'a>,
    offset: usize,
    pattern: &'c P,
}


impl<'a, 'b, 'c, P: CharPattern> Iterator for WordSplit<'a, 'b, 'c, P> {
    type Item = Word<'a>;

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

        let chars = match &self.word.chars {
            Cow::Borrowed(c) => Cow::Borrowed(&c[self.offset .. self.offset + len]),
            Cow::Owned(c) => Cow::Owned(c[self.offset .. self.offset + len].to_vec()),
        };

        let word = Word {
            fin:    self.word.fin || self.offset + len < self.word.len(),
            source: &self.word.source[self.offset .. self.offset + len],
            slice:  (0, len),
            chars,
        };

        self.offset += word.len();

        Some(word)
    }
}


#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use super::{Word, Text};
    use super::super::Chars;

    use Chars::{
        Whitespaces,
        Punctuation,
    };


    fn chars(s: &str) -> Vec<char> {
        s.chars().collect()
    }

    #[test]
    fn word_split() {
        let p = &[Whitespaces, Punctuation];
        let c = chars(" Foo Bar, Baz; ");
        let w = Word::new(&c);
        let s: Vec<Word> = w.split(p).collect();
        assert_debug_snapshot!(s);
    }

    #[test]
    fn word_split_empty() {
        let p = &[Whitespaces, Punctuation];
        let c = chars(" ,;");
        let w = Word::new(&c);
        let s: Vec<Word> = w.split(p).collect();
        assert_debug_snapshot!(s);
    }

    #[test]
    fn word_split_unfinished() {
        let p = &[Whitespaces, Punctuation];
        let c1 = chars(" Foo Bar, Baz");
        let c2 = chars(" Foo Bar, Baz; ");
        let mut w1 = Word::new(&c1);
        let mut w2 = Word::new(&c2);
        w1.fin = false;
        w2.fin = false;
        let s1: Vec<Word> = w1.split(p).collect();
        let s2: Vec<Word> = w2.split(p).collect();
        assert_eq!(s1.last().unwrap().fin, false);
        assert_eq!(s2.last().unwrap().fin, true);
    }

    #[test]
    fn word_strip() {
        let p = &[Whitespaces, Punctuation];
        let c = chars(" Foo Bar, Baz; ");
        let mut w = Word::new(&c);
        w.strip(p);
        assert_debug_snapshot!(w);
    }

    #[test]
    fn word_strip_empty() {
        let p = &[Whitespaces, Punctuation];
        let c = chars(" ,;");
        let mut w = Word::new(&c);
        w.strip(p);
        assert_debug_snapshot!(w);
    }

    #[test]
    fn word_strip_unfinished() {
        let p = &[Whitespaces, Punctuation];
        let c1 = chars(" Foo Bar, Baz");
        let c2 = chars(" Foo Bar, Baz; ");
        let mut w1 = Word::new(&c1);
        let mut w2 = Word::new(&c2);
        w1.fin = false;
        w2.fin = false;
        w1.strip(p);
        w2.strip(p);
        assert_eq!(w1.fin, false);
        assert_eq!(w2.fin, true);
    }

    #[test]
    fn word_lower() {
        let c = chars(" Foo Bar, Baz; ");
        let mut w = Word::new(&c);
        w.lower();
        assert_debug_snapshot!(w);
    }

    #[test]
    fn text_split() {
        let p = &[Whitespaces, Punctuation];
        let c = chars(" Foo Bar, Baz; ");
        let t = Text::new(&c).split(p);
        assert_debug_snapshot!(t);
    }

    #[test]
    fn text_split_empty() {
        let p = &[Whitespaces, Punctuation];
        let c = chars(", ");
        let t = Text::new(&c).split(p);
        assert_debug_snapshot!(t);
    }

    #[test]
    fn text_split_unfinished() {
        let p = &[Whitespaces, Punctuation];
        let c1 = chars(" Foo Bar, Baz");
        let c2 = chars(" Foo Bar, Baz; ");
        let t1 = Text::new(&c1).fin(false).split(p);
        let t2 = Text::new(&c2).fin(false).split(p);
        assert_eq!(t1.words.last().unwrap().fin, false);
        assert_eq!(t2.words.last().unwrap().fin, true);
    }

    #[test]
    fn text_strip() {
        let p = &[Whitespaces, Punctuation];
        let c1 = chars("-Foo-");
        let c2 = chars(",");
        let c3 = chars("Baz; ");
        let t = Text { 
                source: &[],
                words: vec![
                    Word::new(&c1),
                    Word::new(&c2),
                    Word::new(&c3),
                ],
            }.strip(p);
        assert_debug_snapshot!(t);
    }

    #[test]
    fn text_strip_unfinished() {
        let p = &[Whitespaces, Punctuation];
        let c1 = chars("-Foo-");
        let c2 = chars("Baz");
        let c3 = chars("Baz; ");
        let t1 = Text { source: &[], words: vec![Word::new(&c1), Word::new(&c2)] }.fin(false).strip(p);
        let t2 = Text { source: &[], words: vec![Word::new(&c1), Word::new(&c3)] }.fin(false).strip(p);
        assert_eq!(t1.words.last().unwrap().fin, false);
        assert_eq!(t2.words.last().unwrap().fin, true);
    }

    #[test]
    fn text_lower() {
        let c1 = chars("Foo,");
        let c2 = chars("Bar");
        let c3 = chars("Baz");
        let t  = Text {
                source: &[],
                words: vec![
                    Word::new(&c1),
                    Word::new(&c2),
                    Word::new(&c3),
                ],
            }.lower();
        assert_debug_snapshot!(t);
    }
}
