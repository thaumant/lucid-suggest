use std::fmt;


pub trait CharPattern: fmt::Debug {
    fn matches(&self, ch: char) -> Option<bool>;
}


#[derive(Clone, Copy, Debug)]
pub enum CharClass {
    Control,
    Whitespace,
    Punctuation,
    NotAlphaNum,
    Consonant,
    Vowel,
}


impl CharPattern for CharClass {
    fn matches(&self, ch: char) -> Option<bool> {
        match self {
            CharClass::Whitespace  => Some(ch.is_whitespace()),
            CharClass::Punctuation => Some(ch.is_ascii_punctuation()),
            CharClass::Control     => Some(ch.is_control()),
            CharClass::NotAlphaNum => Some(!ch.is_alphanumeric()),
            CharClass::Consonant   => None,
            CharClass::Vowel       => None,
        }
    }
}


impl<P: CharPattern> CharPattern for [P] {
    fn matches(&self, ch: char) -> Option<bool> {
        let mut met_none = false;
        for pattern in self {
            match pattern.matches(ch) {
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


impl<P: CharPattern> CharPattern for [P; 1] { fn matches(&self, ch: char) -> Option<bool> { self[..].matches(ch) } }
impl<P: CharPattern> CharPattern for [P; 2] { fn matches(&self, ch: char) -> Option<bool> { self[..].matches(ch) } }
impl<P: CharPattern> CharPattern for [P; 3] { fn matches(&self, ch: char) -> Option<bool> { self[..].matches(ch) } }
impl<P: CharPattern> CharPattern for [P; 4] { fn matches(&self, ch: char) -> Option<bool> { self[..].matches(ch) } }
impl<P: CharPattern> CharPattern for [P; 5] { fn matches(&self, ch: char) -> Option<bool> { self[..].matches(ch) } }


#[cfg(test)]
mod tests {
    use super::{CharClass, CharPattern};

    use CharClass::{
        Whitespace,
        Punctuation,
        Control,
        NotAlphaNum,
    };

    #[test]
    fn control() {
        assert_eq!(Control.matches('\0'), Some(true));
        assert_eq!(Control.matches('2'),  Some(false));
        assert_eq!(Control.matches('f'),  Some(false));
        assert_eq!(Control.matches(';'),  Some(false));
        assert_eq!(Control.matches(' '),  Some(false));
    }

    #[test]
    fn whitespaces() {
        assert_eq!(Whitespace.matches(' '),  Some(true));
        assert_eq!(Whitespace.matches('\t'), Some(true));
        assert_eq!(Whitespace.matches('\n'), Some(true));
        assert_eq!(Whitespace.matches('2'),  Some(false));
        assert_eq!(Whitespace.matches('f'),  Some(false));
        assert_eq!(Whitespace.matches(';'),  Some(false));
        assert_eq!(Whitespace.matches('\0'), Some(false));
    }

    #[test]
    fn punctuation() {
        assert_eq!(Punctuation.matches(';'),  Some(true));
        assert_eq!(Punctuation.matches('.'),  Some(true));
        assert_eq!(Punctuation.matches(','),  Some(true));
        assert_eq!(Punctuation.matches('2'),  Some(false));
        assert_eq!(Punctuation.matches('f'),  Some(false));
        assert_eq!(Punctuation.matches(' '),  Some(false));
        assert_eq!(Punctuation.matches('\0'), Some(false));
    }

    #[test]
    fn non_alphanum() {
        assert_eq!(NotAlphaNum.matches('2'),  Some(false));
        assert_eq!(NotAlphaNum.matches('f'),  Some(false));
        assert_eq!(NotAlphaNum.matches('й'),  Some(false));
        assert_eq!(NotAlphaNum.matches('も'), Some(false));
        assert_eq!(NotAlphaNum.matches(';'),  Some(true));
        assert_eq!(NotAlphaNum.matches('.'),  Some(true));
        assert_eq!(NotAlphaNum.matches(','),  Some(true));
        assert_eq!(NotAlphaNum.matches(' '),  Some(true));
        assert_eq!(NotAlphaNum.matches('\0'), Some(true));
    }

    #[test]
    fn slice() {
        let pattern = &[Whitespace, Punctuation][..];

        assert_eq!(pattern.matches(' '),  Some(true));
        assert_eq!(pattern.matches(';'),  Some(true));
        assert_eq!(pattern.matches('f'),  Some(false));
        assert_eq!(pattern.matches('2'),  Some(false));
        assert_eq!(pattern.matches('\0'), Some(false));
    }

    #[test]
    fn array_2() {
        let pattern = [Whitespace, Punctuation];

        assert_eq!(pattern.matches(' '),  Some(true));
        assert_eq!(pattern.matches(';'),  Some(true));
        assert_eq!(pattern.matches('f'),  Some(false));
        assert_eq!(pattern.matches('2'),  Some(false));
        assert_eq!(pattern.matches('\0'), Some(false));
    }
}
