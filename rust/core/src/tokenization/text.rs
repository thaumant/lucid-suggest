use std::fmt;
use crate::utils::to_vec;
use crate::lang::{Lang, CharPattern, CharClass};
use super::word::Word;
use super::word_shape::WordShape;
use super::word_view::WordView;


#[derive(PartialEq)]
pub struct Text<W, T, C> where
    W: AsRef<[WordShape]>,
    T: AsRef<[char]>,
    C: AsRef<[CharClass]>
{
    pub words:   W,
    pub source:  T,
    pub chars:   T,
    pub classes: C,
}

pub type TextOwn     = Text<Vec<WordShape>, Vec<char>, Vec<CharClass>>;
pub type TextRef<'a> = Text<&'a [WordShape], &'a [char], &'a [CharClass]>;


impl TextOwn {
    pub fn from_vec(source: Vec<char>) -> TextOwn {
        let len     = source.len();
        let chars   = source.clone();
        let classes = chars.iter().map(|_| CharClass::Any).collect::<Vec<_>>();
        Self {
            words: vec![WordShape::new(len)],
            source,
            chars,
            classes,
        }
    }

    pub fn from_str(source: &str) -> TextOwn {
        Self::from_vec(to_vec(source))
    }
}


impl TextOwn {
    pub fn to_ref<'a>(&'a self) -> TextRef<'a> {
        TextRef {
            words:   &self.words,
            source:  &self.source,
            chars:   &self.chars,
            classes: &self.classes,
        }
    }
}


impl<'a> TextRef<'a> {
    pub fn to_own(&'a self) -> TextOwn {
        TextOwn {
            words:   self.words.to_vec(),
            source:  self.source.to_vec(),
            chars:   self.chars.to_vec(),
            classes: self.classes.to_vec(),
        }
    }
}


impl<W, T, C> Text<W, T, C> where
    W: AsRef<[WordShape]>,
    T: AsRef<[char]>,
    C: AsRef<[CharClass]>
{
    pub fn is_empty(&self) -> bool {
        self.words.as_ref().is_empty()
    }

    pub fn view<'a>(&'a self, i: usize) -> WordView<'a> {
        self.words.as_ref()[i].to_view(self)
    }
}


impl TextOwn {
    pub fn fin(mut self, fin: bool) -> Self {
        if let Some(word) = self.words.last_mut() {
            word.fin = fin;
        }
        self
    }

    pub fn normalize(mut self, lang: &Lang) -> Self {
        if self.words.len() == 0 {
            return self;
        }
        if self.words.len() > 1 {
            panic!("Normalization should always be the first step");
        }
        if let Some(nfc) = lang.unicode_compose(&self.source) {
            self.source           = nfc.clone();
            self.chars            = nfc;
            self.words[0].slice.1 = self.chars.len();
        }
        if let Some((source, chars)) = lang.unicode_reduce(&self.chars) {
            self.source           = source;
            self.chars            = chars;
            self.words[0].slice.1 = self.chars.len();
        }
        self
    }

    pub fn split<P: CharPattern>(mut self, pattern: &P, lang: &Lang) -> Self {
        let mut words = Vec::with_capacity(self.words.len());
        for word in &self.words {
            for splitted in word.split(&self.chars, pattern, lang) {
                words.push(splitted);
            }
        }
        self.words = words;
        for (offset, word) in self.words.iter_mut().enumerate() {
            word.offset = offset;
        }
        self
    }

    pub fn strip<P: CharPattern>(mut self, pattern: &P, lang: &Lang) -> Self {
        for word in &mut self.words {
            word.strip(&self.chars, pattern, lang);
        }
        self.words.retain(|w| w.len() > 0);
        for (offset, word) in self.words.iter_mut().enumerate() {
            word.offset = offset;
        }
        self
    }

    pub fn set_stem(mut self, lang: &Lang) -> Self {
        for word in &mut self.words {
            word.set_stem(&self.chars, lang);
        }
        self
    }

    pub fn set_pos(mut self, lang: &Lang) -> Self {
        for word in &mut self.words {
            word.set_pos(&self.chars, lang);
        }
        self
    }

    pub fn set_char_classes(mut self, lang: &Lang) -> Self {
        self.classes.resize(self.chars.len(), CharClass::Any);
        for (&ch, class) in &mut self.chars.iter().zip(&mut self.classes) {
            *class = lang
                .get_char_class(ch)
                .or_else(|| {
                    if CharClass::NotAlpha.matches(ch, lang)? {
                        Some(CharClass::NotAlpha)
                    } else {
                        None
                    }
                })
                .unwrap_or(CharClass::Any);
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


impl<W, T, C> fmt::Debug for Text<W, T, C> where
    W: AsRef<[WordShape]>,
    T: AsRef<[char]>,
    C: AsRef<[CharClass]>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Text {{")?;
        for word in self.words.as_ref() {
            write!(f, " \"")?;
            for (i, ch) in word.to_view(self).chars().iter().enumerate() {
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
    use crate::utils::to_vec;
    use crate::lang::{Lang, CharClass, PartOfSpeech, lang_english, lang_portuguese, lang_german};
    use super::{WordShape, Text};

    use CharClass::{
        Whitespace,
        Punctuation,
    };

    #[test]
    fn text_normalize_nfd() {
        let lang = lang_portuguese();
        let text = Text::from_str("Conceição").normalize(&lang);
        assert_debug_snapshot!((&text.source, &text.chars, &text.words[0]));
    }

    #[test]
    fn text_normalize_pad0() {
        let lang = lang_german();
        let text = Text::from_str("straße").normalize(&lang);
        assert_debug_snapshot!((&text.source, &text.chars, &text.words[0]));
    }

    #[test]
    fn text_split() {
        let lang = Lang::new();
        let text = Text::from_str(" Foo Bar, Baz; ").split(&[Whitespace, Punctuation], &lang);
        assert_debug_snapshot!(text);
    }

    #[test]
    fn text_split_empty() {
        let lang = Lang::new();
        let text = Text::from_str(", ").split(&[Whitespace, Punctuation], &lang);
        assert_debug_snapshot!(text);
    }

    #[test]
    fn text_split_unfinished() {
        let lang = Lang::new();
        let text1 = Text::from_str(" Foo Bar, Baz"  ).fin(false).split(&[Whitespace, Punctuation], &lang);
        let text2 = Text::from_str(" Foo Bar, Baz; ").fin(false).split(&[Whitespace, Punctuation], &lang);
        assert_eq!(text1.words.last().unwrap().fin, false);
        assert_eq!(text2.words.last().unwrap().fin, true);
    }

    #[test]
    fn text_strip() {
        let lang = Lang::new();
        let chars = to_vec("-Foo- , Baz; ");
        let text  = Text {
                words:  vec![
                    WordShape { offset: 0, slice: (0,  5), stem: 5, pos: None, fin: true },  // "-Foo-"
                    WordShape { offset: 1, slice: (6,  7), stem: 1, pos: None, fin: true },  // ","
                    WordShape { offset: 2, slice: (8, 13), stem: 5, pos: None, fin: true },  // "Baz; "
                ],
                source:  chars.clone(),
                chars:   chars.clone(),
                classes: chars.iter().map(|_| CharClass::Any).collect(),
            }
            .strip(&[Whitespace, Punctuation], &lang);
        assert_debug_snapshot!(text);
        assert_debug_snapshot!(text.words);
    }

    #[test]
    fn text_strip_unfinished() {
        let lang = Lang::new();
        let chars = to_vec("-Foo- Baz; ");
        let text1 = Text {
                words:  vec![
                    WordShape { offset: 0, slice: (0, 5), stem: 5, pos: None, fin: true },  // "-Foo-"
                    WordShape { offset: 1, slice: (5, 8), stem: 3, pos: None, fin: true },  // "Baz"
                ],
                source:  chars.clone(),
                chars:   chars.clone(),
                classes: chars.iter().map(|_| CharClass::Any).collect(),
            }
            .fin(false)
            .strip(&[Whitespace, Punctuation], &lang);

        let text2 = Text {
                words:  vec![
                    WordShape { offset: 0, slice: (0,  5), stem: 5, pos: None, fin: true },  // "-Foo-"
                    WordShape { offset: 1, slice: (5, 10), stem: 5, pos: None, fin: true },  // "Baz; "
                ],
                source:  chars.clone(),
                chars:   chars.clone(),
                classes: chars.iter().map(|_| CharClass::Any).collect(),
            }
            .fin(false)
            .strip(&[Whitespace, Punctuation], &lang);

        assert_eq!(text1.words.last().unwrap().fin, false);
        assert_eq!(text2.words.last().unwrap().fin, true);
    }

    #[test]
    fn text_lower() {
        let chars = to_vec("Foo, Bar Baz");
        let text  = Text {
                words:  vec![
                    WordShape { offset: 0, slice: (0,  4), stem: 4, pos: None, fin: true }, // "Foo,"
                    WordShape { offset: 1, slice: (5,  8), stem: 3, pos: None, fin: true }, // "Bar"
                    WordShape { offset: 2, slice: (9, 12), stem: 3, pos: None, fin: true }, // "Baz"
                ],
                source:  chars.clone(),
                chars:   chars.clone(),
                classes: chars.iter().map(|_| CharClass::Any).collect(),
            }
            .lower();
        assert_debug_snapshot!(text);
    }

    #[test]
    fn text_stem() {
        let chars = to_vec("hello universe");
        let lang  = lang_english();
        let text  = Text {
                words: vec![
                    WordShape { offset: 0, slice: (0,  5), stem: 5, pos: None, fin: true }, // "hello"
                    WordShape { offset: 1, slice: (6, 14), stem: 8, pos: None, fin: true }, // "universe"
                ],
                source:  chars.clone(),
                chars:   chars.clone(),
                classes: chars.iter().map(|_| CharClass::Any).collect(),
            }.set_stem(&lang);
        assert_eq!(text.words[0].stem, 5);
        assert_eq!(text.words[1].stem, 7);
    }

    #[test]
    fn text_pos() {
        let chars = to_vec("the universe");
        let lang  = lang_english();
        let text  = Text {
                words: vec![
                    WordShape { offset: 0, slice: (0,  3), stem: 3, pos: None, fin: true }, // "hello"
                    WordShape { offset: 1, slice: (4, 12), stem: 8, pos: None, fin: true }, // "universe"
                ],
                source:  chars.clone(),
                chars:   chars.clone(),
                classes: chars.iter().map(|_| CharClass::Any).collect(),
            }.set_pos(&lang);
        assert_eq!(text.words[0].pos, Some(PartOfSpeech::Article));
        assert_eq!(text.words[1].pos, None);
    }

    #[test]
    fn text_mark_char_classes_no_lang() {
        let lang  = Lang::new();
        let text  = Text::from_str("the universe, 123").set_char_classes(&lang);
        assert_debug_snapshot!(text.classes);
    }

    #[test]
    fn text_mark_char_classes_lang_en() {
        let lang  = lang_english();
        let text  = Text::from_str("the universe, 123").set_char_classes(&lang);
        assert_debug_snapshot!(text.classes);
    }
}
