use crate::lang::{Lang, CharPattern};
use super::word::Word;
use super::word_shape::WordShape;


pub struct WordSplit<'a, 'b, P: CharPattern> {
    word:        &'a WordShape,
    lang:        &'a Lang,
    chars:       &'a [char],
    pattern:     &'b P,
    word_offset: usize,
    char_offset: usize,
}


impl<'a, 'b, P: CharPattern> WordSplit<'a, 'b, P> {
    pub fn new(
        word:    &'a WordShape,
        chars:   &'a [char],
        pattern: &'b P,
        lang:    &'a Lang,
    ) -> Self {
        Self { lang, word, chars, pattern, word_offset: word.offset, char_offset: 0 }
    }
}


impl<'a, 'b, P: CharPattern> Iterator for WordSplit<'a, 'b, P> {
    type Item = WordShape;

    fn next(&mut self) -> Option<Self::Item> {
        let Self { word, word_offset, char_offset, pattern, lang, .. } = self;
        let chars = &self.chars[word.slice.0 .. word.slice.1];

        if *char_offset >= word.len() {
            return None;
        }

        *char_offset += chars[*char_offset ..]
            .iter()
            .take_while(|&&ch| pattern.matches(ch, lang).unwrap_or(false))
            .count();

        let len = chars[*char_offset ..]
            .iter()
            .take_while(|&&ch| !pattern.matches(ch, lang).unwrap_or(false))
            .count();

        if len == 0 {
            return None;
        }

        let splitted = WordShape {
            offset: *word_offset,
            slice:  (word.slice.0 + *char_offset, word.slice.0 + *char_offset + len),
            stem:   len,
            pos:    None,
            fin:    word.fin || *char_offset + len < word.len(),
        };

        *char_offset += splitted.len();
        *word_offset += 1;

        Some(splitted)
    }
}


#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use crate::utils::to_vec;
    use crate::lang::{Lang, CharClass};
    use super::super::word_shape::WordShape;

    use CharClass::{
        Whitespace,
        Punctuation,
    };

    #[test]
    fn word_split() {
        let lang  = Lang::new();
        let chars = to_vec(" Foo Bar, Baz; ");
        let word  = WordShape::new(chars.len());
        let split = word.split(&chars[..], &[Whitespace, Punctuation], &lang).collect::<Vec<_>>();
        assert_debug_snapshot!(split);
    }

    #[test]
    fn word_split_empty() {
        let lang  = Lang::new();
        let chars = to_vec(" ,;");
        let word  = WordShape::new(chars.len());
        let split = word.split(&chars[..], &[Whitespace, Punctuation], &lang).collect::<Vec<_>>();
        assert_debug_snapshot!(split);
    }

    #[test]
    fn word_split_unfinished() {
        let lang   = Lang::new();
        let chars1 = to_vec(" Foo Bar, Baz");
        let chars2 = to_vec(" Foo Bar, Baz; ");
        let word1  = WordShape::new(chars1.len()).set_fin(false);
        let word2  = WordShape::new(chars2.len()).set_fin(false);
        let split1 = word1.split(&chars1[..], &[Whitespace, Punctuation], &lang).collect::<Vec<_>>();
        let split2 = word2.split(&chars2[..], &[Whitespace, Punctuation], &lang).collect::<Vec<_>>();
        assert_eq!(split1.last().unwrap().fin, false);
        assert_eq!(split2.last().unwrap().fin, true);
    }
}
