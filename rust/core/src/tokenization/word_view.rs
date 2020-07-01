use crate::lang::{CharClass, PartOfSpeech};
use super::text::Text;
use super::word::Word;
use super::word_shape::WordShape;


#[derive(PartialEq, Debug, Clone)]
pub struct WordView<'a> {
    pub ix:      usize,
    pub place:   (usize, usize),
    pub stem:    usize,
    pub pos:     Option<PartOfSpeech>,
    pub fin:     bool,
    source:  &'a [char],
    chars:   &'a [char],
    classes: &'a [CharClass],
}


impl<'a> Word for WordView<'a> {
    #[inline] fn ix(&self)    -> usize                { self.ix }
    #[inline] fn place(&self) -> (usize, usize)       { self.place }
    #[inline] fn stem(&self)  -> usize                { self.stem }
    #[inline] fn pos(&self)   -> Option<PartOfSpeech> { self.pos }
    #[inline] fn fin(&self)   -> bool                 { self.fin }
}


impl<'a> WordView<'a> {
    pub fn new<W, T, C>(word: &'a WordShape, text: &'a Text<W, T, C>) -> Self where
        W: AsRef<[WordShape]>,
        T: AsRef<[char]>,
        C: AsRef<[CharClass]>
    {
        Self {
            ix:      word.ix,
            place:   word.place,
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
            ix:      self.ix,
            place:   self.place,
            stem:    self.stem,
            pos:     self.pos,
            fin:     self.fin,
        }
    }

    pub fn source(&'a self) -> &'a [char] {
        &self.source[self.place.0 .. self.place.1]
    }

    pub fn chars(&'a self) -> &'a [char] {
        &self.chars[self.place.0 .. self.place.1]
    }

    pub fn classes(&'a self) -> &'a [CharClass] {
        &self.classes[self.place.0 .. self.place.1]
    }

    pub fn join(&self, other: &Self) -> Self {
        Self {
            ix:      self.ix,
            place:   (self.place.0, other.place.1),
            stem:    other.place.0 - self.place.0 + other.stem,
            pos:     None,
            fin:     other.fin,
            source:  &self.source,
            chars:   &self.chars,
            classes: &self.classes,
        }
    }
}
