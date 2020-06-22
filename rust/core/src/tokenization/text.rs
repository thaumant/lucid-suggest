use std::fmt;
use crate::lang::Lang;
use super::CharPattern;
use super::Word;


#[derive(PartialEq)]
pub struct Text<T: AsRef<[char]>> {
    pub source: T,
    pub chars: T,
    pub words: Vec<Word>,
}


impl Text<Vec<char>> {
    pub fn from_vec(source: Vec<char>) -> Text<Vec<char>> {
        let len   = source.len();
        let chars = source.clone();
        Text {
            source,
            chars,
            words: vec![Word::new(len)],
        }
    }

    pub fn from_str(source: &str) -> Text<Vec<char>> {
        Text::from_vec(source.chars().collect())
    }
}


impl Text<Vec<char>> {
    pub fn to_ref<'a>(&'a self) -> Text<&'a [char]> {
        Text {
            source: &self.source,
            chars:  &self.chars,
            words:  self.words.clone(),
        }
    }
}


impl<'a> Text<&'a [char]> {
    pub fn to_own(&self) -> Text<Vec<char>> {
        Text {
            source: self.source.to_vec(),
            chars:  self.chars.to_vec(),
            words:  self.words.clone(),
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
            for splitted in word.split(&self.chars, pattern) {
                words.push(splitted);
            }
        }
        self.words = words;
        for (ix, word) in self.words.iter_mut().enumerate() {
            word.ix = ix;
        }
        self
    }

    pub fn strip<P: CharPattern>(mut self, pattern: &P) -> Self {
        for word in &mut self.words {
            word.strip(&self.chars, pattern);
        }
        self.words.retain(|w| w.len() > 0);
        for (ix, word) in self.words.iter_mut().enumerate() {
            word.ix = ix;
        }
        self
    }

    pub fn stem(mut self, lang: &Lang) -> Self {
        for word in &mut self.words {
            word.stem(&self.chars, lang);
        }
        self
    }

    pub fn mark_pos(mut self, lang: &Lang) -> Self {
        for word in &mut self.words {
            word.mark_pos(&self.chars, lang);
        }
        self
    }

    pub fn lower(mut self) -> Self {
        if self.chars.iter().any(|ch| ch.is_uppercase()) {
            for ch in &mut self.chars {
                *ch = ch.to_lowercase().next().unwrap_or(*ch);
            }
        }
        self
    }
}


impl<T: AsRef<[char]>> fmt::Debug for Text<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Text {{")?;
        for word in &self.words {
            let chars = word.view(self.chars.as_ref());
            write!(f, " \"")?;
            for (i, ch) in chars.iter().enumerate() {
                write!(f, "{}", ch)?;
                if i == word.stem - 1 && i != word.len() - 1 {
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
        let text = Text::from_str(" Foo Bar, Baz; ").split(&[Whitespaces, Punctuation]);
        assert_debug_snapshot!(text);
    }

    #[test]
    fn text_split_empty() {
        let text = Text::from_str(", ").split(&[Whitespaces, Punctuation]);
        assert_debug_snapshot!(text);
    }

    #[test]
    fn text_split_unfinished() {
        let text1 = Text::from_str(" Foo Bar, Baz"  ).fin(false).split(&[Whitespaces, Punctuation]);
        let text2 = Text::from_str(" Foo Bar, Baz; ").fin(false).split(&[Whitespaces, Punctuation]);
        assert_eq!(text1.words.last().unwrap().fin, false);
        assert_eq!(text2.words.last().unwrap().fin, true);
    }

    #[test]
    fn text_strip() {
        let chars = "-Foo- , Baz; ".chars().collect::<Vec<_>>();
        let text  = Text {
                source: chars.clone(),
                chars:  chars.clone(),
                words:  vec![
                    Word { ix: 0, place: (0,  5), stem: 5, pos: None, fin: true },  // "-Foo-"
                    Word { ix: 1, place: (6,  7), stem: 1, pos: None, fin: true },  // ","
                    Word { ix: 2, place: (8, 13), stem: 5, pos: None, fin: true },  // "Baz; "
                ],
            }
            .strip(&[Whitespaces, Punctuation]);
        assert_debug_snapshot!(text);
        assert_debug_snapshot!(text.words);
    }

    #[test]
    fn text_strip_unfinished() {
        let chars = "-Foo- Baz; ".chars().collect::<Vec<_>>();
        let text1 = Text {
                source: chars.clone(),
                chars:  chars.clone(),
                words:  vec![
                    Word { ix: 0, place: (0, 5), stem: 5, pos: None, fin: true },  // "-Foo-"
                    Word { ix: 1, place: (5, 8), stem: 3, pos: None, fin: true },  // "Baz"
                ],
            }
            .fin(false)
            .strip(&[Whitespaces, Punctuation]);

        let text2 = Text {
                source: chars.clone(),
                chars:  chars.clone(),
                words:  vec![
                    Word { ix: 0, place: (0,  5), stem: 5, pos: None, fin: true },  // "-Foo-"
                    Word { ix: 1, place: (5, 10), stem: 5, pos: None, fin: true },  // "Baz; "
                ],
            }
            .fin(false)
            .strip(&[Whitespaces, Punctuation]);

        assert_eq!(text1.words.last().unwrap().fin, false);
        assert_eq!(text2.words.last().unwrap().fin, true);
    }

    #[test]
    fn text_lower() {
        let chars = "Foo, Bar Baz".chars().collect::<Vec<_>>();
        let text  = Text {
                source: chars.clone(),
                chars:  chars.clone(),
                words:  vec![
                    Word { ix: 0, place: (0,  4), stem: 4, pos: None, fin: true }, // "Foo,"
                    Word { ix: 1, place: (5,  8), stem: 3, pos: None, fin: true }, // "Bar"
                    Word { ix: 2, place: (9, 12), stem: 3, pos: None, fin: true }, // "Baz"
                ],
            }
            .lower();
        assert_debug_snapshot!(text);
    }

    #[test]
    fn text_stem() {
        let chars = "hello universe".chars().collect::<Vec<_>>();
        let lang  = lang_english();
        let text  = Text {
                source: chars.clone(),
                chars:  chars.clone(),
                words: vec![
                    Word { ix: 0, place: (0,  5), stem: 5, pos: None, fin: true }, // "hello"
                    Word { ix: 1, place: (6, 14), stem: 8, pos: None, fin: true }, // "universe"
                ],
            }.stem(&lang);
        assert_eq!(text.words[0].stem, 5);
        assert_eq!(text.words[1].stem, 7);
    }

    #[test]
    fn text_pos() {
        let chars = "the universe".chars().collect::<Vec<_>>();
        let lang  = lang_english();
        let text  = Text {
                source: chars.clone(),
                chars:  chars.clone(),
                words: vec![
                    Word { ix: 0, place: (0,  3), stem: 3, pos: None, fin: true }, // "hello"
                    Word { ix: 1, place: (4, 12), stem: 8, pos: None, fin: true }, // "universe"
                ],
            }.mark_pos(&lang);
        assert_eq!(text.words[0].pos, Some(PartOfSpeech::Article));
        assert_eq!(text.words[1].pos, None);
    }
}
