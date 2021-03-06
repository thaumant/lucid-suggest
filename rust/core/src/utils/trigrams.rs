
pub trait Trigrams {
    fn trigrams<'a>(&'a self) -> TrigramIter<'a>;
}


impl Trigrams for [char] {
    fn trigrams<'a>(&'a self) -> TrigramIter<'a> {
        TrigramIter::new(self)
    }
}


#[derive(Debug)]
pub struct TrigramIter<'a> {
    word: &'a [char],
    size: usize,
}

impl<'a> TrigramIter<'a> {
    pub fn new(word: &'a [char]) -> Self {
        Self { word, size: 1 }
    }
}

impl<'a> Iterator for TrigramIter<'a> {
    type Item = [char; 3];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.word.len() < self.size {
            return None;
        }

        let mut gram = ['\0', '\0', '\0'];
        gram[..self.size].copy_from_slice(&self.word[..self.size]);

        if self.size < 3 {
            self.size += 1;
        } else {
            self.word = &self.word[1..];
        }

        Some(gram)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.word.len();
        (len, Some(len))
    }
}

impl<'a> ExactSizeIterator for TrigramIter<'a> { }


#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use crate::utils::to_vec;
    use super::Trigrams;

    #[test]
    fn trigrams_basic() {
        let input = to_vec("foobar");
        for len in 0 .. input.len() {
            let slice  = &input[..len];
            let output = slice.trigrams().collect::<Vec<_>>();
            assert_debug_snapshot!(output);
        }
    }

    #[test]
    fn trigrams_size_hint() {
        let input = to_vec("foobar");
        for len in 0 .. input.len() {
            let slice  = &input[..len];
            let output = slice.trigrams().collect::<Vec<_>>();
            assert_eq!(output.len(), slice.len());
        }
    }
}
