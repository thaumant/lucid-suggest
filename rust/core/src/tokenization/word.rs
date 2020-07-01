use crate::lang::{PartOfSpeech};

pub trait Word {
    fn ix(&self)    -> usize;
    fn place(&self) -> (usize, usize);
    fn stem(&self)  -> usize;
    fn pos(&self)   -> Option<PartOfSpeech>;
    fn fin(&self)   -> bool;

    fn len(&self) -> usize {
        let (left, right) = self.place();
        right - left
    }

    fn is_empty(&self) -> bool {
        let (left, right) = self.place();
        right == left
    }

    fn is_function(&self) -> bool {
        match self.pos() {
            Some(PartOfSpeech::Article)     => true,
            Some(PartOfSpeech::Preposition) => true,
            Some(PartOfSpeech::Conjunction) => true,
            Some(PartOfSpeech::Particle)    => true,
            _ => false,
        }
    }

    fn dist(&self, other: &Self) -> usize {
        let (left1, right1) = self.place();
        let (left2, right2) = other.place();
        if left1 >= right2 { return left1 - right2; }
        if left2 >= right1 { return left2 - right1; }
        panic!("Malformed words: ({}, {}), ({}, {})", left1, right1, left2, right2);
    }
}

