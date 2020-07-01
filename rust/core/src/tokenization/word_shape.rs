use crate::lang::{Lang, CharClass, CharPattern, PartOfSpeech};
use super::word::Word;
use super::word_view::WordView;
use super::word_split::WordSplit;
use super::text::Text;


#[derive(PartialEq, Debug, Clone)]
pub struct WordShape {
    pub ix:    usize,
    pub place: (usize, usize),
    pub stem:  usize,
    pub pos:   Option<PartOfSpeech>,
    pub fin:   bool,
}


impl Word for WordShape {
    #[inline] fn ix(&self)    -> usize                { self.ix }
    #[inline] fn place(&self) -> (usize, usize)       { self.place }
    #[inline] fn stem(&self)  -> usize                { self.stem }
    #[inline] fn pos(&self)   -> Option<PartOfSpeech> { self.pos }
    #[inline] fn fin(&self)   -> bool                 { self.fin }
}


impl WordShape {
    pub fn new(len: usize) -> Self {
        WordShape {
            ix:    0,
            place: (0, len),
            stem:  len,
            pos:   None,
            fin:   true,
        }
    }

    pub fn to_view<'a, W, T, C>(&'a self, text: &'a Text<W, T, C>) -> WordView<'a> where
        W: AsRef<[WordShape]>,
        T: AsRef<[char]>,
        C: AsRef<[CharClass]>
    {
        WordView::new(self, text)
    }

    pub fn join(&self, other: &Self) -> Self {
        Self {
            ix:    self.ix,
            place: (self.place.0, other.place.1),
            stem:  other.place.0 - self.place.0 + other.stem,
            pos:   None,
            fin:   other.fin,
        }
    }

    pub fn set_fin(mut self, fin: bool) -> Self {
        self.fin = fin;
        self
    }

    pub fn split<'a, 'b, P: CharPattern>(&'a self, chars: &'a [char], pattern: &'b P, lang: &'a Lang) -> WordSplit<'a, 'b, P> {
        WordSplit::new(self, chars, pattern, lang)
    }

    pub fn strip<P: CharPattern>(&mut self, chars: &[char], pattern: &P, lang: &Lang) -> &mut Self {
        let chars = &chars[self.place.0 .. self.place.1];
        let left  = chars.iter()
            .take_while(|&&ch| pattern.matches(ch, lang).unwrap_or(false))
            .count();
        let right = chars.iter()
            .rev()
            .take_while(|&&ch| pattern.matches(ch, lang).unwrap_or(false))
            .take(chars.len() - left)
            .count();
        self.place.0 += left;
        self.place.1 -= right;
        self.fin = self.fin || right != 0;
        self
    }

    pub fn set_stem(&mut self, chars: &[char], lang: &Lang) -> &mut Self {
        let chars = &chars[self.place.0 .. self.place.1];
        self.stem = lang.stem(chars);
        self
    }

    pub fn set_pos(&mut self, chars: &[char], lang: &Lang) -> &mut Self {
        let chars = &chars[self.place.0 .. self.place.1];
        self.pos = lang.get_pos(chars);
        self
    }

    pub fn lower(&mut self, chars: &mut [char]) -> &mut Self {
        let chars = &mut chars[self.place.0 .. self.place.1];
        if chars.iter().any(|ch| ch.is_uppercase()) {
            for ch in chars {
                *ch = ch.to_lowercase().next().unwrap_or(*ch);
            }
        }
        self
    }
}


#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use crate::utils::to_vec;
    use crate::lang::{Lang, CharClass, PartOfSpeech, lang_english};
    use super::{Word, WordShape};

    use CharClass::{
        Whitespace,
        Punctuation,
    };

    #[test]
    fn word_dist_basic() {
        let mut w1 = WordShape::new(7);
        let mut w2 = WordShape::new(5);
        w1.place = (0, w1.len());
        w2.place = (w1.place.1 + 2, w1.place.1 + 2 + w2.len());
        assert_eq!(w1.dist(&w2), 2);
        assert_eq!(w2.dist(&w1), 2);
    }

    #[test]
    fn word_dist_fused() {
        let mut w1 = WordShape::new(7);
        let mut w2 = WordShape::new(5);
        w1.place = (0, w1.len());
        w2.place = (w1.place.1, w1.place.1 + w2.len());
        assert_eq!(w1.dist(&w2), 0);
        assert_eq!(w2.dist(&w1), 0);
    }

    #[test]
    #[should_panic]
    fn word_dist_malformed() {
        let mut w1 = WordShape::new(7);
        let mut w2 = WordShape::new(5);
        w1.place = (0, w1.len());
        w2.place = (w1.place.1 - 2, w1.place.1 - 2 + w2.len());
        w1.dist(&w2);
    }

    #[test]
    fn word_join_basic() {
        let mut w1 = WordShape::new(7);
        let mut w2 = WordShape::new(5);
        w1.place = (0, w1.len());
        w2.place = (w1.place.1 + 1, w1.place.1 + 1 + w2.len());
        assert_debug_snapshot!(w1.join(&w2));
    }

    #[test]
    fn word_join_offset() {
        let mut w1 = WordShape::new(7);
        let mut w2 = WordShape::new(5);
        w1.place = (3, 3 + w1.len());
        w2.place = (w1.place.1 + 1, w1.place.1 + 1 + w2.len());
        assert_debug_snapshot!(w1.join(&w2));
    }

    #[test]
    fn word_join_unfinished_first() {
        let mut w1 = WordShape::new(7);
        let mut w2 = WordShape::new(5);
        w1.place = (0, w1.len());
        w2.place = (w1.place.1 + 1, w1.place.1 + 1 + w2.len());
        w1.fin   = false;
        w2.fin   = true;
        assert_debug_snapshot!(w1.join(&w2));
    }

    #[test]
    fn word_join_unfinished_last() {
        let mut w1 = WordShape::new(7);
        let mut w2 = WordShape::new(5);
        w1.place = (0, w1.len());
        w2.place = (w1.place.1 + 1, w1.place.1 + 1 + w2.len());
        w1.fin   = true;
        w2.fin   = false;
        assert_debug_snapshot!(w1.join(&w2));
    }

    #[test]
    fn word_join_pos_first() {
        let mut w1 = WordShape::new(7);
        let mut w2 = WordShape::new(5);
        w1.place = (0, w1.len());
        w2.place = (w1.place.1 + 1, w1.place.1 + 1 + w2.len());
        w1.pos   = Some(PartOfSpeech::Article);
        w2.pos   = None;
        assert_debug_snapshot!(w1.join(&w2));
    }

    #[test]
    fn word_join_pos_last() {
        let mut w1 = WordShape::new(7);
        let mut w2 = WordShape::new(5);
        w1.place = (0, w1.len());
        w2.place = (w1.place.1 + 1, w1.place.1 + 1 + w2.len());
        w1.pos   = None;
        w2.pos   = Some(PartOfSpeech::Article);
        assert_debug_snapshot!(w1.join(&w2));
    }

    #[test]
    fn word_strip() {
        let lang   = Lang::new();
        let chars = to_vec(" Foo; ");
        let mut word = WordShape::new(chars.len());
        word.strip(&chars[..], &[Whitespace, Punctuation], &lang);
        assert_debug_snapshot!(&word);
    }

    #[test]
    fn word_strip_empty() {
        let lang   = Lang::new();
        let chars = to_vec(" ,;");
        let mut word = WordShape::new(chars.len());
        word.strip(&chars[..], &[Whitespace, Punctuation], &lang);
        assert_debug_snapshot!(word);
    }

    #[test]
    fn word_strip_unfinished() {
        let lang   = Lang::new();
        let chars1 = to_vec(" Foo Bar, Baz");
        let chars2 = to_vec(" Foo Bar, Baz; ");
        let mut word1 = WordShape::new(chars1.len()).set_fin(false);
        let mut word2 = WordShape::new(chars2.len()).set_fin(false);
        word1.strip(&chars1[..], &[Whitespace, Punctuation], &lang);
        word2.strip(&chars2[..], &[Whitespace, Punctuation], &lang);
        assert_eq!(word1.fin, false);
        assert_eq!(word2.fin, true);
    }

    #[test]
    fn word_stem() {
        let chars = to_vec("university");
        let lang  = lang_english();
        let mut word = WordShape::new(chars.len());
        word.set_stem(&chars[..], &lang);
        assert_eq!(word.stem, 7);
    }

    #[test]
    fn word_pos() {
        let lang   = lang_english();
        let chars1 = to_vec("university");
        let chars2 = to_vec("the");
        let mut word1 = WordShape::new(chars1.len());
        let mut word2 = WordShape::new(chars2.len());
        word1.set_pos(&chars1[..], &lang);
        word2.set_pos(&chars2[..], &lang);
        assert_eq!(word1.pos, None);
        assert_eq!(word2.pos, Some(PartOfSpeech::Article));
    }

    #[test]
    fn word_lower() {
        let mut chars = to_vec(" Foo Bar, Baz; ");
        let mut word  = WordShape::new(chars.len());
        word.lower(&mut chars[..]);
        assert_debug_snapshot!(word);
    }
}
