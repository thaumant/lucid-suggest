use std::cmp::Ordering;


pub trait LimitSort: Iterator + Sized {
    fn limit_sort<F>(self, limit: usize, sort_fn: F) -> LimitSortIter<Self::Item, Self, F> where
        F: (FnMut(&Self::Item, &Self::Item) -> Ordering)
    {
        LimitSortIter {
            sort_fn,
            source: self,
            buffer: Vec::with_capacity(limit * 2),
            limit,
            done: false,
        }
    }
}


pub struct LimitSortIter<T, I: Iterator<Item=T>, F: FnMut(&T, &T) -> Ordering> {
    sort_fn: F,
    source:  I,
    buffer:  Vec<T>,
    limit:   usize,
    done:    bool,
}


impl<T, I: Iterator<Item=T>, F: FnMut(&T, &T) -> Ordering> Iterator for LimitSortIter<T, I, F> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let Self { buffer, source, sort_fn, done, .. } = self;
        let limit = self.limit;

        if !*done {
            while let Some(item) = source.next() {
                buffer.push(item);
                if buffer.len() >= limit * 2 {
                    buffer.sort_by(|x, y| sort_fn(x, y));
                    buffer.truncate(limit);
                }
            }
            buffer.sort_by(|x, y| sort_fn(x, y));
            buffer.truncate(limit);
            buffer.reverse();
            *done = true;
        }

        buffer.pop()
    }
}


impl<I: Iterator> LimitSort for I { }
