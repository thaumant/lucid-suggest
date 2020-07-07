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
            stable: true,
            done:   false,
        }
    }

    fn limit_sort_unstable<F>(self, limit: usize, sort_fn: F) -> LimitSortIter<Self::Item, Self, F> where
        F: (FnMut(&Self::Item, &Self::Item) -> Ordering)
    {
        LimitSortIter {
            sort_fn,
            source: self,
            buffer: Vec::with_capacity(limit * 2),
            limit,
            stable: false,
            done:   false,
        }
    }
}


pub struct LimitSortIter<T, I, F> where
 I: Iterator<Item=T>,
 F: FnMut(&T, &T) -> Ordering
{
    sort_fn: F,
    source:  I,
    buffer:  Vec<T>,
    limit:   usize,
    stable:  bool,
    done:    bool,
}


impl<T, I, F> Iterator for LimitSortIter<T, I, F> where
    I: Iterator<Item=T>,
    F: FnMut(&T, &T) -> Ordering
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let Self { buffer, source, sort_fn, done, .. } = self;
        let limit  = self.limit;
        let stable = self.stable;

        let mut sort = |buffer: &mut Vec<T>| {
            if stable {
                buffer.sort_by(|x, y| sort_fn(x, y));
            } else {
                buffer.sort_unstable_by(|x, y| sort_fn(x, y));
            }
        };

        if !*done {
            while let Some(item) = source.next() {
                buffer.push(item);
                if buffer.len() >= limit * 2 {
                    sort(buffer);
                    buffer.truncate(limit);
                }
            }
            sort(buffer);
            buffer.truncate(limit);
            buffer.reverse();
            *done = true;
        }

        buffer.pop()
    }
}


impl<I: Iterator> LimitSort for I { }
