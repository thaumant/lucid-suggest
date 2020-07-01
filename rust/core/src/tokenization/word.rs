pub use crate::lang::{Lang, Chars, CharPattern, PartOfSpeech};


#[derive(PartialEq, Debug, Clone)]
pub struct Word {
    pub ix:    usize,
    pub place: (usize, usize),
    pub stem:  usize,
    pub pos:   Option<PartOfSpeech>,
    pub fin:   bool,
}


impl Word {
    pub fn new(len: usize) -> Self {
        Word {
            ix:    0,
            place: (0, len),
            stem:  len,
            pos:   None,
            fin:   true,
        }
    }

    pub fn len(&self) -> usize {
        self.place.1 - self.place.0
    }

    pub fn is_empty(&self) -> bool {
        self.place.1 == self.place.0
    }

    pub fn is_function(&self) -> bool {
        match self.pos {
            Some(PartOfSpeech::Article)     => true,
            Some(PartOfSpeech::Preposition) => true,
            Some(PartOfSpeech::Conjunction) => true,
            Some(PartOfSpeech::Particle)    => true,
            _ => false,
        }
    }

    pub fn view<'a>(&self, chars: &'a [char]) -> &'a [char] {
        &chars[self.place.0 .. self.place.1]
    }

    pub fn view_mut<'a>(&self, chars: &'a mut [char]) -> &'a mut [char] {
        &mut chars[self.place.0 .. self.place.1]
    }

    pub fn dist(&self, other: &Self) -> usize {
        let (start1, end1) = self.place;
        let (start2, end2) = other.place;
        if start1 >= end2 { return start1 - end2; }
        if start2 >= end1 { return start2 - end1; }
        panic!("Malformed words: ({}, {}), ({}, {})", start1, end1, start2, end2);
    }

    pub fn join(&self, other: &Self) -> Self {
        Word {
            ix:    self.ix,
            place: (self.place.0, other.place.1),
            stem:  other.place.0 - self.place.0 + other.stem,
            pos:   None,
            fin:   other.fin,
        }
    }
}


impl Word {
    pub fn fin(mut self, fin: bool) -> Self {
        self.fin = fin;
        self
    }

    pub fn split<'a, 'b, P: CharPattern>(&'a self, chars: &'a [char], pattern: &'b P) -> WordSplit<'a, 'b, P> {
        WordSplit { word: self, chars, pattern, ix: self.ix, offset: 0 }
    }

    pub fn strip<P: CharPattern>(&mut self, chars: &[char], pattern: &P) -> &mut Self {
        let chars = self.view(chars);
        let left  = chars.iter()
            .take_while(|&&ch| pattern.matches(ch).unwrap_or(false))
            .count();
        let right = chars.iter()
            .rev()
            .take_while(|&&ch| pattern.matches(ch).unwrap_or(false))
            .take(chars.len() - left)
            .count();
        self.place.0 += left;
        self.place.1 -= right;
        self.fin = self.fin || right != 0;
        self
    }

    pub fn stem(&mut self, chars: &[char], lang: &Lang) -> &mut Self {
        self.stem = lang.stem(self.view(chars));
        self
    }

    pub fn mark_pos(&mut self, chars: &[char], lang: &Lang) -> &mut Self {
        self.pos = lang.get_pos(self.view(chars));
        self
    }

    pub fn lower(&mut self, chars: &mut [char]) -> &mut Self {
        let chars = self.view_mut(chars);
        if chars.iter().any(|ch| ch.is_uppercase()) {
            for ch in chars {
                *ch = ch.to_lowercase().next().unwrap_or(*ch);
            }
        }
        self
    }
}


#[derive(Debug)]
pub struct WordSplit<'a, 'b, P: CharPattern> {
    word:    &'a Word,
    chars:   &'a [char],
    pattern: &'b P,
    ix:      usize,
    offset:  usize,
}


impl<'a, 'b, P: CharPattern> Iterator for WordSplit<'a, 'b, P> {
    type Item = Word;

    fn next(&mut self) -> Option<Self::Item> {
        let Self { word, ix, offset, pattern, .. } = self;
        let chars = word.view(self.chars);

        if *offset >= word.len() {
            return None;
        }

        *offset += chars[*offset ..]
            .iter()
            .take_while(|&&ch| pattern.matches(ch).unwrap_or(false))
            .count();

        let len = chars[*offset ..]
            .iter()
            .take_while(|&&ch| !pattern.matches(ch).unwrap_or(false))
            .count();

        if len == 0 {
            return None;
        }

        let splitted = Word {
            ix:     *ix,
            place:  (word.place.0 + *offset, word.place.0 + *offset + len),
            stem:   len,
            pos:    None,
            fin:    word.fin || *offset + len < word.len(),
        };

        *offset += splitted.len();
        *ix     += 1;

        Some(splitted)
    }
}


#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use crate::utils::to_vec;
    use crate::lang::lang_english;
    use super::{Word, Chars, PartOfSpeech};

    use Chars::{
        Whitespaces,
        Punctuation,
    };

    #[test]
    fn word_dist_basic() {
        let mut w1 = Word::new(7);
        let mut w2 = Word::new(5);
        w1.place = (0, w1.len());
        w2.place = (w1.place.1 + 2, w1.place.1 + 2 + w2.len());
        assert_eq!(w1.dist(&w2), 2);
        assert_eq!(w2.dist(&w1), 2);
    }

    #[test]
    fn word_dist_fused() {
        let mut w1 = Word::new(7);
        let mut w2 = Word::new(5);
        w1.place = (0, w1.len());
        w2.place = (w1.place.1, w1.place.1 + w2.len());
        assert_eq!(w1.dist(&w2), 0);
        assert_eq!(w2.dist(&w1), 0);
    }

    #[test]
    #[should_panic]
    fn word_dist_malformed() {
        let mut w1 = Word::new(7);
        let mut w2 = Word::new(5);
        w1.place = (0, w1.len());
        w2.place = (w1.place.1 - 2, w1.place.1 - 2 + w2.len());
        w1.dist(&w2);
    }

    #[test]
    fn word_join_basic() {
        let mut w1 = Word::new(7);
        let mut w2 = Word::new(5);
        w1.place = (0, w1.len());
        w2.place = (w1.place.1 + 1, w1.place.1 + 1 + w2.len());
        assert_debug_snapshot!(w1.join(&w2));
    }

    #[test]
    fn word_join_offset() {
        let mut w1 = Word::new(7);
        let mut w2 = Word::new(5);
        w1.place = (3, 3 + w1.len());
        w2.place = (w1.place.1 + 1, w1.place.1 + 1 + w2.len());
        assert_debug_snapshot!(w1.join(&w2));
    }

    #[test]
    fn word_join_unfinished_first() {
        let mut w1 = Word::new(7);
        let mut w2 = Word::new(5);
        w1.place = (0, w1.len());
        w2.place = (w1.place.1 + 1, w1.place.1 + 1 + w2.len());
        w1.fin   = false;
        w2.fin   = true;
        assert_debug_snapshot!(w1.join(&w2));
    }

    #[test]
    fn word_join_unfinished_last() {
        let mut w1 = Word::new(7);
        let mut w2 = Word::new(5);
        w1.place = (0, w1.len());
        w2.place = (w1.place.1 + 1, w1.place.1 + 1 + w2.len());
        w1.fin   = true;
        w2.fin   = false;
        assert_debug_snapshot!(w1.join(&w2));
    }

    #[test]
    fn word_join_pos_first() {
        let mut w1 = Word::new(7);
        let mut w2 = Word::new(5);
        w1.place = (0, w1.len());
        w2.place = (w1.place.1 + 1, w1.place.1 + 1 + w2.len());
        w1.pos   = Some(PartOfSpeech::Article);
        w2.pos   = None;
        assert_debug_snapshot!(w1.join(&w2));
    }

    #[test]
    fn word_join_pos_last() {
        let mut w1 = Word::new(7);
        let mut w2 = Word::new(5);
        w1.place = (0, w1.len());
        w2.place = (w1.place.1 + 1, w1.place.1 + 1 + w2.len());
        w1.pos   = None;
        w2.pos   = Some(PartOfSpeech::Article);
        assert_debug_snapshot!(w1.join(&w2));
    }

    #[test]
    fn word_split() {
        let chars = to_vec(" Foo Bar, Baz; ");
        let word  = Word::new(chars.len());
        let split = word.split(&chars[..], &[Whitespaces, Punctuation]).collect::<Vec<_>>();
        assert_debug_snapshot!(split);
    }

    #[test]
    fn word_split_empty() {
        let chars = to_vec(" ,;");
        let word  = Word::new(chars.len());
        let split = word.split(&chars[..], &[Whitespaces, Punctuation]).collect::<Vec<_>>();
        assert_debug_snapshot!(split);
    }

    #[test]
    fn word_split_unfinished() {
        let chars1 = to_vec(" Foo Bar, Baz");
        let chars2 = to_vec(" Foo Bar, Baz; ");
        let word1  = Word::new(chars1.len()).fin(false);
        let word2  = Word::new(chars2.len()).fin(false);
        let split1 = word1.split(&chars1[..], &[Whitespaces, Punctuation]).collect::<Vec<_>>();
        let split2 = word2.split(&chars2[..], &[Whitespaces, Punctuation]).collect::<Vec<_>>();
        assert_eq!(split1.last().unwrap().fin, false);
        assert_eq!(split2.last().unwrap().fin, true);
    }

    #[test]
    fn word_strip() {
        let chars = to_vec(" Foo; ");
        let mut word = Word::new(chars.len());
        word.strip(&chars[..], &[Whitespaces, Punctuation]);
        assert_debug_snapshot!(&word);
    }

    #[test]
    fn word_strip_empty() {
        let chars = to_vec(" ,;");
        let mut word = Word::new(chars.len());
        word.strip(&chars[..], &[Whitespaces, Punctuation]);
        assert_debug_snapshot!(word);
    }

    #[test]
    fn word_strip_unfinished() {
        let chars1 = to_vec(" Foo Bar, Baz");
        let chars2 = to_vec(" Foo Bar, Baz; ");
        let mut word1 = Word::new(chars1.len()).fin(false);
        let mut word2 = Word::new(chars2.len()).fin(false);
        word1.strip(&chars1[..], &[Whitespaces, Punctuation]);
        word2.strip(&chars2[..], &[Whitespaces, Punctuation]);
        assert_eq!(word1.fin, false);
        assert_eq!(word2.fin, true);
    }

    #[test]
    fn word_stem() {
        let chars = to_vec("university");
        let lang  = lang_english();
        let mut word = Word::new(chars.len());
        word.stem(&chars[..], &lang);
        assert_eq!(word.stem, 7);
    }

    #[test]
    fn word_pos() {
        let lang   = lang_english();
        let chars1 = to_vec("university");
        let chars2 = to_vec("the");
        let mut word1 = Word::new(chars1.len());
        let mut word2 = Word::new(chars2.len());
        word1.mark_pos(&chars1[..], &lang);
        word2.mark_pos(&chars2[..], &lang);
        assert_eq!(word1.pos, None);
        assert_eq!(word2.pos, Some(PartOfSpeech::Article));
    }

    #[test]
    fn word_lower() {
        let mut chars = to_vec(" Foo Bar, Baz; ");
        let mut word  = Word::new(chars.len());
        word.lower(&mut chars[..]);
        assert_debug_snapshot!(word);
    }
}
