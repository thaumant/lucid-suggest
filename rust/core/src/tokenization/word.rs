pub use crate::lang::Lang;
pub use super::{Chars, CharPattern};
pub use super::pos::PartOfSpeech;


#[derive(PartialEq, Debug, Clone)]
pub struct Word {
    pub place: (usize, usize),
    pub stem:  usize,
    pub pos:   Option<PartOfSpeech>,
    pub fin:   bool,
}


impl Word {
    pub fn new(len: usize) -> Self {
        Word {
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

    pub fn is_primary(&self) -> bool {
        match self.pos {
            Some(PartOfSpeech::Article)     => false,
            Some(PartOfSpeech::Preposition) => false,
            Some(PartOfSpeech::Conjunction) => false,
            Some(PartOfSpeech::Particle)    => false,
            _ => true,
        }
    }

    pub fn view<'a>(&self, chars: &'a [char]) -> &'a [char] {
        &chars[self.place.0 .. self.place.1]
    }

    pub fn view_mut<'a>(&self, chars: &'a mut [char]) -> &'a mut [char] {
        &mut chars[self.place.0 .. self.place.1]
    }
}


impl Word {
    pub fn fin(mut self, fin: bool) -> Self {
        self.fin = fin;
        self
    }

    pub fn split<'a, 'b, P: CharPattern>(&'a self, chars: &'a [char], pattern: &'b P) -> WordSplit<'a, 'b, P> {
        WordSplit { word: self, chars, pattern, offset: 0 }
    }

    pub fn strip<P: CharPattern>(&mut self, chars: &[char], pattern: &P) -> &mut Self {
        let chars = self.view(chars);
        let left  = chars.iter()
            .take_while(|&&ch| pattern.matches(ch))
            .count();
        let right = chars.iter()
            .rev()
            .take_while(|&&ch| pattern.matches(ch))
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
pub struct WordSplit<'b, 'c, P: CharPattern> {
    word: &'b Word,
    chars: &'b [char],
    offset: usize,
    pattern: &'c P,
}


impl<'b, 'c, P: CharPattern> Iterator for WordSplit<'b, 'c, P> {
    type Item = Word;

    fn next(&mut self) -> Option<Self::Item> {
        let Self { word, offset, pattern, .. } = self;
        let chars = word.view(self.chars);

        if *offset >= word.len() {
            return None;
        }

        *offset += chars[*offset ..]
            .iter()
            .take_while(|&&ch| pattern.matches(ch))
            .count();

        let len = chars[*offset ..]
            .iter()
            .take_while(|&&ch| !pattern.matches(ch))
            .count();

        if len == 0 {
            return None;
        }

        let splitted = Word {
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
        let chars = " Foo Bar, Baz; ".chars().collect::<Vec<_>>();
        let word  = Word::new(chars.len());
        let split = word.split(&chars[..], &[Whitespaces, Punctuation]).collect::<Vec<_>>();
        assert_debug_snapshot!(split);
    }

    #[test]
    fn word_split_empty() {
        let chars = " ,;".chars().collect::<Vec<_>>();
        let word  = Word::new(chars.len());
        let split = word.split(&chars[..], &[Whitespaces, Punctuation]).collect::<Vec<_>>();
        assert_debug_snapshot!(split);
    }

    #[test]
    fn word_split_unfinished() {
        let chars1 = " Foo Bar, Baz"  .chars().collect::<Vec<_>>();
        let chars2 = " Foo Bar, Baz; ".chars().collect::<Vec<_>>();
        let word1  = Word::new(chars1.len()).fin(false);
        let word2  = Word::new(chars2.len()).fin(false);
        let split1 = word1.split(&chars1[..], &[Whitespaces, Punctuation]).collect::<Vec<_>>();
        let split2 = word2.split(&chars2[..], &[Whitespaces, Punctuation]).collect::<Vec<_>>();
        assert_eq!(split1.last().unwrap().fin, false);
        assert_eq!(split2.last().unwrap().fin, true);
    }

    #[test]
    fn word_strip() {
        let chars = " Foo; ".chars().collect::<Vec<_>>();
        let mut word = Word::new(chars.len());
        word.strip(&chars[..], &[Whitespaces, Punctuation]);
        assert_debug_snapshot!(&word);
    }

    #[test]
    fn word_strip_empty() {
        let chars = " ,;".chars().collect::<Vec<_>>();
        let mut word = Word::new(chars.len());
        word.strip(&chars[..], &[Whitespaces, Punctuation]);
        assert_debug_snapshot!(word);
    }

    #[test]
    fn word_strip_unfinished() {
        let chars1 = " Foo Bar, Baz"  .chars().collect::<Vec<_>>();
        let chars2 = " Foo Bar, Baz; ".chars().collect::<Vec<_>>();
        let mut word1 = Word::new(chars1.len()).fin(false);
        let mut word2 = Word::new(chars2.len()).fin(false);
        word1.strip(&chars1[..], &[Whitespaces, Punctuation]);
        word2.strip(&chars2[..], &[Whitespaces, Punctuation]);
        assert_eq!(word1.fin, false);
        assert_eq!(word2.fin, true);
    }

    #[test]
    fn word_stem() {
        let chars = "university".chars().collect::<Vec<_>>();
        let lang  = lang_english();
        let mut word = Word::new(chars.len());
        word.stem(&chars[..], &lang);
        assert_eq!(word.stem, 7);
    }

    #[test]
    fn word_pos() {
        let lang   = lang_english();
        let chars1 = "university".chars().collect::<Vec<_>>();
        let chars2 = "the"       .chars().collect::<Vec<_>>();
        let mut word1 = Word::new(chars1.len());
        let mut word2 = Word::new(chars2.len());
        word1.mark_pos(&chars1[..], &lang);
        word2.mark_pos(&chars2[..], &lang);
        assert_eq!(word1.pos, None);
        assert_eq!(word2.pos, Some(PartOfSpeech::Article));
    }

    #[test]
    fn word_lower() {
        let mut chars = " Foo Bar, Baz; ".chars().collect::<Vec<_>>();
        let mut word  = Word::new(chars.len());
        word.lower(&mut chars[..]);
        assert_debug_snapshot!(word);
    }
}