use std::fmt;
use super::Lang;


pub trait CharPattern: fmt::Debug {
    fn matches(&self, ch: char, lang: &Lang) -> Option<bool>;
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CharClass {
    Any,
    Control,
    Whitespace,
    Punctuation,
    NotAlpha,
    NotAlphaNum,
    Consonant,
    Vowel,
}

use CharClass::{
    Any,
    Control,
    Whitespace,
    Punctuation,
    NotAlpha,
    NotAlphaNum,
    Consonant,
    Vowel,
};


impl CharPattern for CharClass {
    fn matches(&self, ch: char, lang: &Lang) -> Option<bool> {
        match self {
            Any         => Some(true),
            Control     => Some(ch.is_control()),
            Whitespace  => Some(ch.is_whitespace()),
            Punctuation => Some(ch.is_ascii_punctuation()),
            NotAlpha    => Some(!ch.is_alphabetic()),
            NotAlphaNum => Some(!ch.is_alphanumeric()),
            Consonant   => Some(lang.get_char_class(ch)? == Consonant),
            Vowel       => Some(lang.get_char_class(ch)? == Vowel),
        }
    }
}


impl<P: CharPattern> CharPattern for [P] {
    fn matches(&self, ch: char, lang: &Lang) -> Option<bool> {
        let mut met_none = false;
        for pattern in self {
            match pattern.matches(ch, lang) {
                Some(true)  => return Some(true),
                Some(false) => continue,
                None => {
                    met_none = true;
                    continue;
                },
            }
        }
        if met_none { None } else { Some(false) }
    }
}


impl<P: CharPattern> CharPattern for [P; 1] { fn matches(&self, ch: char, lang: &Lang) -> Option<bool> { self[..].matches(ch, lang) } }
impl<P: CharPattern> CharPattern for [P; 2] { fn matches(&self, ch: char, lang: &Lang) -> Option<bool> { self[..].matches(ch, lang) } }
impl<P: CharPattern> CharPattern for [P; 3] { fn matches(&self, ch: char, lang: &Lang) -> Option<bool> { self[..].matches(ch, lang) } }
impl<P: CharPattern> CharPattern for [P; 4] { fn matches(&self, ch: char, lang: &Lang) -> Option<bool> { self[..].matches(ch, lang) } }
impl<P: CharPattern> CharPattern for [P; 5] { fn matches(&self, ch: char, lang: &Lang) -> Option<bool> { self[..].matches(ch, lang) } }


#[cfg(test)]
mod tests {
    use super::{CharClass, CharPattern, Lang};

    use CharClass::{
        Any,
        Control,
        Whitespace,
        Punctuation,
        NotAlpha,
        NotAlphaNum,
        Consonant,
        Vowel,
    };

    #[test]
    fn pattern_matches_any() {
        assert_eq!(Any.matches('\0', &Lang::new()), Some(true));
        assert_eq!(Any.matches('2',  &Lang::new()), Some(true));
        assert_eq!(Any.matches('f',  &Lang::new()), Some(true));
        assert_eq!(Any.matches(';',  &Lang::new()), Some(true));
        assert_eq!(Any.matches(' ',  &Lang::new()), Some(true));
    }

    #[test]
    fn pattern_matches_control() {
        assert_eq!(Control.matches('\0', &Lang::new()), Some(true));
        assert_eq!(Control.matches('2',  &Lang::new()), Some(false));
        assert_eq!(Control.matches('f',  &Lang::new()), Some(false));
        assert_eq!(Control.matches(';',  &Lang::new()), Some(false));
        assert_eq!(Control.matches(' ',  &Lang::new()), Some(false));
    }

    #[test]
    fn pattern_matches_whitespaces() {
        assert_eq!(Whitespace.matches(' ',  &Lang::new()), Some(true));
        assert_eq!(Whitespace.matches('\t', &Lang::new()), Some(true));
        assert_eq!(Whitespace.matches('\n', &Lang::new()), Some(true));
        assert_eq!(Whitespace.matches('2',  &Lang::new()), Some(false));
        assert_eq!(Whitespace.matches('f',  &Lang::new()), Some(false));
        assert_eq!(Whitespace.matches(';',  &Lang::new()), Some(false));
        assert_eq!(Whitespace.matches('\0', &Lang::new()), Some(false));
    }

    #[test]
    fn pattern_matches_punctuation() {
        assert_eq!(Punctuation.matches(';',  &Lang::new()), Some(true));
        assert_eq!(Punctuation.matches('.',  &Lang::new()), Some(true));
        assert_eq!(Punctuation.matches(',',  &Lang::new()), Some(true));
        assert_eq!(Punctuation.matches('2',  &Lang::new()), Some(false));
        assert_eq!(Punctuation.matches('f',  &Lang::new()), Some(false));
        assert_eq!(Punctuation.matches(' ',  &Lang::new()), Some(false));
        assert_eq!(Punctuation.matches('\0', &Lang::new()), Some(false));
    }

    #[test]
    fn pattern_matches_non_alpha() {
        assert_eq!(NotAlpha.matches('2',  &Lang::new()), Some(true));
        assert_eq!(NotAlpha.matches('f',  &Lang::new()), Some(false));
        assert_eq!(NotAlpha.matches('ы',  &Lang::new()), Some(false));
        assert_eq!(NotAlpha.matches('も', &Lang::new()), Some(false));
        assert_eq!(NotAlpha.matches(';',  &Lang::new()), Some(true));
        assert_eq!(NotAlpha.matches('.',  &Lang::new()), Some(true));
        assert_eq!(NotAlpha.matches(',',  &Lang::new()), Some(true));
        assert_eq!(NotAlpha.matches(' ',  &Lang::new()), Some(true));
        assert_eq!(NotAlpha.matches('\0', &Lang::new()), Some(true));
    }

    #[test]
    fn pattern_matches_non_alphanum() {
        assert_eq!(NotAlphaNum.matches('2',  &Lang::new()), Some(false));
        assert_eq!(NotAlphaNum.matches('f',  &Lang::new()), Some(false));
        assert_eq!(NotAlphaNum.matches('ы',  &Lang::new()), Some(false));
        assert_eq!(NotAlphaNum.matches('も', &Lang::new()), Some(false));
        assert_eq!(NotAlphaNum.matches(';',  &Lang::new()), Some(true));
        assert_eq!(NotAlphaNum.matches('.',  &Lang::new()), Some(true));
        assert_eq!(NotAlphaNum.matches(',',  &Lang::new()), Some(true));
        assert_eq!(NotAlphaNum.matches(' ',  &Lang::new()), Some(true));
        assert_eq!(NotAlphaNum.matches('\0', &Lang::new()), Some(true));
    }

    #[test]
    fn pattern_matches_slice() {
        let pattern = &[Whitespace, Punctuation][..];

        assert_eq!(pattern.matches(' ',  &Lang::new()), Some(true));
        assert_eq!(pattern.matches(';',  &Lang::new()), Some(true));
        assert_eq!(pattern.matches('f',  &Lang::new()), Some(false));
        assert_eq!(pattern.matches('2',  &Lang::new()), Some(false));
        assert_eq!(pattern.matches('\0', &Lang::new()), Some(false));
    }

    #[test]
    fn pattern_matches_array_2() {
        let pattern = [Whitespace, Punctuation];

        assert_eq!(pattern.matches(' ',  &Lang::new()), Some(true));
        assert_eq!(pattern.matches(';',  &Lang::new()), Some(true));
        assert_eq!(pattern.matches('f',  &Lang::new()), Some(false));
        assert_eq!(pattern.matches('2',  &Lang::new()), Some(false));
        assert_eq!(pattern.matches('\0', &Lang::new()), Some(false));
    }

    #[test]
    fn pattern_matches_vowel_no_lang() {
        assert_eq!(Vowel.matches('2',  &Lang::new()), None);
        assert_eq!(Vowel.matches('f',  &Lang::new()), None);
        assert_eq!(Vowel.matches('ы',  &Lang::new()), None);
        assert_eq!(Vowel.matches('も', &Lang::new()), None);
        assert_eq!(Vowel.matches(';',  &Lang::new()), None);
        assert_eq!(Vowel.matches('.',  &Lang::new()), None);
        assert_eq!(Vowel.matches(',',  &Lang::new()), None);
        assert_eq!(Vowel.matches(' ',  &Lang::new()), None);
        assert_eq!(Vowel.matches('\0', &Lang::new()), None);
    }

    #[test]
    fn pattern_matches_vowel_with_lang() {
        let mut lang = Lang::new();
        lang.add_char_class('f', Consonant);
        lang.add_char_class('ы', Vowel);

        assert_eq!(Vowel.matches('2',  &lang), None);
        assert_eq!(Vowel.matches('f',  &lang), Some(false));
        assert_eq!(Vowel.matches('ы',  &lang), Some(true));
        assert_eq!(Vowel.matches('も', &lang), None);
        assert_eq!(Vowel.matches(';',  &lang), None);
        assert_eq!(Vowel.matches('.',  &lang), None);
        assert_eq!(Vowel.matches(',',  &lang), None);
        assert_eq!(Vowel.matches(' ',  &lang), None);
        assert_eq!(Vowel.matches('\0', &lang), None);
    }

    #[test]
    fn pattern_matches_consonant_no_lang() {
        assert_eq!(Consonant.matches('2',  &Lang::new()), None);
        assert_eq!(Consonant.matches('f',  &Lang::new()), None);
        assert_eq!(Consonant.matches('ы',  &Lang::new()), None);
        assert_eq!(Consonant.matches('も', &Lang::new()), None);
        assert_eq!(Consonant.matches(';',  &Lang::new()), None);
        assert_eq!(Consonant.matches('.',  &Lang::new()), None);
        assert_eq!(Consonant.matches(',',  &Lang::new()), None);
        assert_eq!(Consonant.matches(' ',  &Lang::new()), None);
        assert_eq!(Consonant.matches('\0', &Lang::new()), None);
    }

    #[test]
    fn pattern_matches_consonant_with_lang() {
        let mut lang = Lang::new();
        lang.add_char_class('f', Consonant);
        lang.add_char_class('ы', Vowel);

        assert_eq!(Consonant.matches('2',  &lang), None);
        assert_eq!(Consonant.matches('f',  &lang), Some(true));
        assert_eq!(Consonant.matches('ы',  &lang), Some(false));
        assert_eq!(Consonant.matches('も', &lang), None);
        assert_eq!(Consonant.matches(';',  &lang), None);
        assert_eq!(Consonant.matches('.',  &lang), None);
        assert_eq!(Consonant.matches(',',  &lang), None);
        assert_eq!(Consonant.matches(' ',  &lang), None);
        assert_eq!(Consonant.matches('\0', &lang), None);
    }
}
