use std::cmp::Ordering;
use crate::search::Hit;


pub struct LimitSort<'a, Src: Iterator<Item=Hit<'a>>> {
    source: Src,
    buffer: Vec<Hit<'a>>,
    limit:  usize,
    done:   bool,
}


impl<'a, Src: Iterator<Item=Hit<'a>>> LimitSort<'a, Src> {
    pub fn new(source: Src, limit: usize) -> Self {
        Self {
            source,
            buffer: Vec::with_capacity(limit * 2),
            limit,
            done: false,
        }
    }

    fn sort_and_truncate(&mut self) {
        self.buffer.sort_by(|hit1, hit2| {
            hit1.scores.linear.iter()
                .zip(hit2.scores.linear.iter())
                .map(|(s1, s2)| s2.cmp(s1))
                .find(|&ord| ord != Ordering::Equal)
                .unwrap_or(Ordering::Equal)
        });
        self.buffer.truncate(self.limit);
    }
}


impl<'a, Src: Iterator<Item=Hit<'a>>> Iterator for LimitSort<'a, Src> {
    type Item = Hit<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.done {
            while let Some(hit) = self.source.next() {
                self.buffer.push(hit);
                if self.buffer.len() >= self.limit * 2 {
                    self.sort_and_truncate();
                }
            }
            self.sort_and_truncate();
            self.buffer.reverse();
            self.done = true;
        }

        self.buffer.pop()
    }
}
