use std::fmt;


pub trait CharPattern: fmt::Debug {
    fn matches(&self, ch: char) -> bool;
}


#[derive(Clone, Copy, Debug)]
pub enum Chars {
    Control,
    Whitespaces,
    Punctuation,
    NotAlphaNum,
}


impl CharPattern for Chars {
    fn matches(&self, ch: char) -> bool {
        match self {
            Chars::Whitespaces  => ch.is_whitespace(),
            Chars::Punctuation  => ch.is_ascii_punctuation(),
            Chars::Control      => ch.is_control(),
            Chars::NotAlphaNum  => !ch.is_alphanumeric(),
        }
    }
}


impl<P: CharPattern> CharPattern for [P] {
    fn matches(&self, ch: char) -> bool {
        self.iter().any(|p| p.matches(ch))
    }
}


impl<P: CharPattern> CharPattern for [P; 1] { fn matches(&self, ch: char) -> bool { self[..].matches(ch) } }
impl<P: CharPattern> CharPattern for [P; 2] { fn matches(&self, ch: char) -> bool { self[..].matches(ch) } }
impl<P: CharPattern> CharPattern for [P; 3] { fn matches(&self, ch: char) -> bool { self[..].matches(ch) } }
impl<P: CharPattern> CharPattern for [P; 4] { fn matches(&self, ch: char) -> bool { self[..].matches(ch) } }
impl<P: CharPattern> CharPattern for [P; 5] { fn matches(&self, ch: char) -> bool { self[..].matches(ch) } }


#[cfg(test)]
mod tests {
    use super::{Chars, CharPattern};
    
    use Chars::{
        Whitespaces,
        Punctuation,
        Control,
        NotAlphaNum,
    };

    #[test]
    fn control() {
        assert_eq!(Control.matches('\0'), true);
        assert_eq!(Control.matches('2'), false);
        assert_eq!(Control.matches('f'), false);
        assert_eq!(Control.matches(';'), false);
        assert_eq!(Control.matches(' '), false);
    }

    #[test]
    fn whitespaces() {
        assert_eq!(Whitespaces.matches(' '), true);
        assert_eq!(Whitespaces.matches('\t'), true);
        assert_eq!(Whitespaces.matches('\n'), true);
        assert_eq!(Whitespaces.matches('2'), false);
        assert_eq!(Whitespaces.matches('f'), false);
        assert_eq!(Whitespaces.matches(';'), false);
        assert_eq!(Whitespaces.matches('\0'), false);
    }

    #[test]
    fn punctuation() {
        assert_eq!(Punctuation.matches(';'), true);
        assert_eq!(Punctuation.matches('.'), true);
        assert_eq!(Punctuation.matches(','), true);
        assert_eq!(Punctuation.matches('2'), false);
        assert_eq!(Punctuation.matches('f'), false);
        assert_eq!(Punctuation.matches(' '), false);
        assert_eq!(Punctuation.matches('\0'), false);
    }

    #[test]
    fn non_alphanum() {
        assert_eq!(NotAlphaNum.matches('2'), false);
        assert_eq!(NotAlphaNum.matches('f'), false);
        assert_eq!(NotAlphaNum.matches('й'), false);
        assert_eq!(NotAlphaNum.matches('も'), false);
        assert_eq!(NotAlphaNum.matches(';'), true);
        assert_eq!(NotAlphaNum.matches('.'), true);
        assert_eq!(NotAlphaNum.matches(','), true);
        assert_eq!(NotAlphaNum.matches(' '), true);
        assert_eq!(NotAlphaNum.matches('\0'), true);
    }

    #[test]
    fn slice() {
        let pattern = &[Whitespaces, Punctuation][..];

        assert_eq!(pattern.matches(' '), true);
        assert_eq!(pattern.matches(';'), true);
        assert_eq!(pattern.matches('f'), false);
        assert_eq!(pattern.matches('2'), false);
        assert_eq!(pattern.matches('\0'), false);
    }

    #[test]
    fn array_2() {
        let pattern = [Whitespaces, Punctuation];

        assert_eq!(pattern.matches(' '), true);
        assert_eq!(pattern.matches(';'), true);
        assert_eq!(pattern.matches('f'), false);
        assert_eq!(pattern.matches('2'), false);
        assert_eq!(pattern.matches('\0'), false);
    }
}
