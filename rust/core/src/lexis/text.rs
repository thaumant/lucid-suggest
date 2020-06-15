use std::fmt;
use crate::lang::Lang;
use super::CharPattern;
use super::Word;


#[derive(PartialEq)]
pub struct Text<T: AsRef<[char]>> {
    pub source: T,
    pub words: Vec<Word<T>>,
}


impl Text<Vec<char>> {
    pub fn from_vec(source: Vec<char>) -> Text<Vec<char>> {
        let cloned = source.clone();
        Text {
            source: source,
            words: vec![Word::from_vec(cloned)],
        }
    }

    pub fn from_str(source: &str) -> Text<Vec<char>> {
        let source = source.chars().collect::<Vec<_>>();
        let word   = source.clone();
        Text {
            source,
            words: vec![Word::from_vec(word)],
        }
    }
}


impl Text<Vec<char>> {
    pub fn to_ref<'a>(&'a self) -> Text<&'a [char]> {
        Text {
            source: &self.source,
            words: self.words.iter().map(|w| w.to_ref()).collect()
        }
    }
}


impl<'a> Text<&'a [char]> {
    pub fn to_own(&self) -> Text<Vec<char>> {
        Text {
            source: self.source.to_vec(),
            words:  self.words.iter().map(|w| w.to_own()).collect(),
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
}


impl Text<Vec<char>> {
    pub fn split<P: CharPattern>(mut self, pattern: &P) -> Self {
        let mut words = Vec::with_capacity(self.words.len());
        for word in &self.words {
            for splitted in word.split(pattern) {
                words.push(splitted.to_own());
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

    pub fn stem(mut self, lang: &Lang) -> Self {
        for word in &mut self.words {
            word.stem(lang);
        }
        self
    }

    pub fn mark_pos(mut self, lang: &Lang) -> Self {
        for word in &mut self.words {
            word.mark_pos(lang);
        }
        self
    }

    pub fn lower(mut self) -> Self {
        for word in &mut self.words {
            word.lower();
        }
        self
    }
}


impl<T: AsRef<[char]>> fmt::Debug for Text<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Text {{")?;
        for word in &self.words {
            let len  = word.chars.as_ref().len();
            let stem = word.stem;
            write!(f, " \"")?;
            for (i, ch) in word.chars.as_ref().iter().enumerate() {
                write!(f, "{}", ch)?;
                if i == stem - 1 && i != len - 1 {
                    write!(f, "|")?;
                }
            }
            write!(f, "\"")?;
            if !word.fin { write!(f, "..")?; }
        }
        write!(f, " }}")?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use crate::lang::lang_english;
    use super::{Word, Text};
    use super::super::{Chars, PartOfSpeech};

    use Chars::{
        Whitespaces,
        Punctuation,
    };

    #[test]
    fn text_split() {
        let t = Text::from_str(" Foo Bar, Baz; ").split(&[Whitespaces, Punctuation]);
        assert_debug_snapshot!(t);
    }

    #[test]
    fn text_split_empty() {
        let t = Text::from_str(", ").split(&[Whitespaces, Punctuation]);
        assert_debug_snapshot!(t);
    }

    #[test]
    fn text_split_unfinished() {
        let t1 = Text::from_str(" Foo Bar, Baz"  ).fin(false).split(&[Whitespaces, Punctuation]);
        let t2 = Text::from_str(" Foo Bar, Baz; ").fin(false).split(&[Whitespaces, Punctuation]);
        assert_eq!(t1.words.last().unwrap().fin, false);
        assert_eq!(t2.words.last().unwrap().fin, true);
    }

    #[test]
    fn text_strip() {
        let t = Text {
                source: vec![],
                words: vec![
                    Word::from_str("-Foo-"),
                    Word::from_str(","),
                    Word::from_str("Baz; "),
                ],
            }
            .strip(&[Whitespaces, Punctuation]);
        assert_debug_snapshot!(t);
    }

    #[test]
    fn text_strip_unfinished() {
        let t1 = Text {
                source: vec![],
                words: vec![
                    Word::from_str("-Foo-"),
                    Word::from_str("Baz"),
                ],
            }
            .fin(false)
            .strip(&[Whitespaces, Punctuation]);
        let t2 = Text {
                source: vec![],
                words: vec![
                    Word::from_str("-Foo-"),
                    Word::from_str("Baz; "),
                ],
            }
            .fin(false)
            .strip(&[Whitespaces, Punctuation]);
        assert_eq!(t1.words.last().unwrap().fin, false);
        assert_eq!(t2.words.last().unwrap().fin, true);
    }

    #[test]
    fn text_lower() {
        let t  = Text {
                source: vec![],
                words: vec![
                    Word::from_str("Foo,"),
                    Word::from_str("Bar"),
                    Word::from_str("Baz"),
                ],
            }.lower();
        assert_debug_snapshot!(t);
    }

    #[test]
    fn text_stem() {
        let lang = lang_english();
        let t  = Text {
                source: vec![],
                words: vec![
                    Word::from_str("hello"),
                    Word::from_str("universe"),
                ],
            }.stem(&lang);
        assert_eq!(t.words[0].stem, 5);
        assert_eq!(t.words[1].stem, 7);
    }

    #[test]
    fn text_pos() {
        let lang = lang_english();
        let t  = Text {
                source: vec![],
                words: vec![
                    Word::from_str("the"),
                    Word::from_str("universe"),
                ],
            }.mark_pos(&lang);
        assert_eq!(t.words[0].pos, Some(PartOfSpeech::Article));
        assert_eq!(t.words[1].pos, None);
    }
}
