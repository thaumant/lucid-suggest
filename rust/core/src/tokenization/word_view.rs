use crate::lang::{CharClass, PartOfSpeech};
use super::text::Text;
use super::word::Word;
use super::word_shape::WordShape;


#[derive(PartialEq, Debug, Clone)]
pub struct WordView<'a> {
    pub offset:  usize,
    pub slice:   (usize, usize),
    pub stem:    usize,
    pub pos:     Option<PartOfSpeech>,
    pub fin:     bool,
    source:      &'a [char],
    chars:       &'a [char],
    classes:     &'a [CharClass],
}


impl<'a> Word for WordView<'a> {
    #[inline] fn offset(&self) -> usize                { self.offset }
    #[inline] fn slice(&self)  -> (usize, usize)       { self.slice }
    #[inline] fn stem(&self)   -> usize                { self.stem }
    #[inline] fn pos(&self)    -> Option<PartOfSpeech> { self.pos }
    #[inline] fn fin(&self)    -> bool                 { self.fin }
}


impl<'a> WordView<'a> {
    pub fn new<W, T, C>(word: &'a WordShape, text: &'a Text<W, T, C>) -> Self where
        W: AsRef<[WordShape]>,
        T: AsRef<[char]>,
        C: AsRef<[CharClass]>
    {
        Self {
            offset:  word.offset,
            slice:   word.slice,
            stem:    word.stem,
            pos:     word.pos,
            fin:     word.fin,
            source:  &text.source.as_ref(),
            chars:   &text.chars.as_ref(),
            classes: &text.classes.as_ref(),
        }
    }

    pub fn to_shape(&'a self) -> WordShape {
        WordShape {
            offset: self.offset,
            slice:  self.slice,
            stem:   self.stem,
            pos:    self.pos,
            fin:    self.fin,
        }
    }

    pub fn source(&'a self) -> &'a [char] {
        &self.source[self.slice.0 .. self.slice.1]
    }

    pub fn chars(&'a self) -> &'a [char] {
        &self.chars[self.slice.0 .. self.slice.1]
    }

    pub fn classes(&'a self) -> &'a [CharClass] {
        &self.classes[self.slice.0 .. self.slice.1]
    }

    pub fn join(&self, other: &Self) -> Self {
        Self {
            offset:  self.offset,
            slice:   (self.slice.0, other.slice.1),
            stem:    other.slice.0 - self.slice.0 + other.stem,
            pos:     None,
            fin:     other.fin,
            source:  &self.source,
            chars:   &self.chars,
            classes: &self.classes,
        }
    }
}
