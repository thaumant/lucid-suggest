mod matrix;
mod utils;

use std::cmp::max;
use std::collections::BTreeMap;
use std::cell::RefCell;
use utils::common_affix_sizes;
use matrix::DistMatrix;


const DEFAULT_CAPACITY: usize = 20;


pub struct DamerauLevenshtein<T: PartialEq + Copy + Ord> {
    pub dists: RefCell<DistMatrix>,
    last_i1: RefCell<BTreeMap<T, usize>>,
}


impl<T: PartialEq + Copy + Ord> DamerauLevenshtein<T> {
    pub fn new() -> Self {
        let dists   = RefCell::new(DistMatrix::new(DEFAULT_CAPACITY + 2));
        let last_i1 = RefCell::new(BTreeMap::new());
        Self { dists, last_i1 }
    }

    pub fn distance(&self, slice1: &[T], slice2: &[T]) -> usize {
        let (prefix, postfix) = common_affix_sizes(slice1, slice2);
        let mut slice1 = { let len = slice1.len(); &slice1[prefix .. len - postfix] };
        let mut slice2 = { let len = slice2.len(); &slice2[prefix .. len - postfix] };
        if slice2.len() < slice1.len() {
            std::mem::swap(&mut slice1, &mut slice2);
        }

        let dists = &mut *self.dists.borrow_mut();
        dists.grow(max(slice1.len() + 2, slice2.len() + 2));

        let last_i1 = &mut *self.last_i1.borrow_mut();
        last_i1.clear();

        for (i1, &x1) in slice1.iter().enumerate() {
            let mut l2 = 0;

            for (i2, &x2) in slice2.iter().enumerate() {
                let l1 = *last_i1.get(&x2).unwrap_or(&0);

                unsafe {
                    dists.set(i1 + 2, i2 + 2, min!(
                        dists.get(i1 + 2, i2 + 1) + 1,
                        dists.get(i1 + 1, i2 + 2) + 1,
                        dists.get(i1 + 1, i2 + 1) + (x1 != x2) as usize,
                        dists.get(l1, l2) + (i1 - l1) + (i2 - l2) + 1
                    ));
                }

                if x1 == x2 { l2 = i2 + 1; }
            }
            last_i1.insert(x1, i1 + 1);
        }

        unsafe { dists.get(slice1.len() + 1, slice2.len() + 1) }
    }
}


#[cfg(test)]
mod tests {
    use super::DamerauLevenshtein;

    #[test]
    fn equality() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 3],
        ];
        for s in sample.iter() {
            assert_eq!(damlev.distance(s, s), 0);
        }
    }

    #[test]
    fn prefix() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (0, vec![1, 2, 3], vec![1, 2, 3]),
            (1, vec![1, 2, 3], vec![1, 2]),
            (2, vec![1, 2, 3], vec![1]),
            (3, vec![1, 2, 3], vec![]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(damlev.distance(s1, s2), *d);
            assert_eq!(damlev.distance(s2, s1), *d);
        }
    }

    #[test]
    fn add_del_continuous() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (1, vec![1, 2, 3], vec![0, 1, 2, 3]),
            (2, vec![1, 2, 3], vec![0, 0, 1, 2, 3]),
            (3, vec![1, 2, 3], vec![0, 0, 0, 1, 2, 3]),

            (1, vec![1, 2, 3], vec![1, 0, 2, 3]),
            (2, vec![1, 2, 3], vec![1, 0, 0, 2, 3]),
            (3, vec![1, 2, 3], vec![1, 0, 0, 0, 2, 3]),

            (1, vec![1, 2, 3], vec![1, 2, 3, 0]),
            (2, vec![1, 2, 3], vec![1, 2, 3, 0, 0]),
            (3, vec![1, 2, 3], vec![1, 2, 3, 0, 0, 0]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(damlev.distance(s1, s2), *d);
            assert_eq!(damlev.distance(s2, s1), *d);
        }
    }

    #[test]
    fn sub_continuous() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (1, vec![1, 2, 3, 4], vec![0, 2, 3, 4]),
            (2, vec![1, 2, 3, 4], vec![0, 0, 3, 4]),
            (3, vec![1, 2, 3, 4], vec![0, 0, 0, 4]),

            (1, vec![1, 2, 3, 4], vec![1, 0, 3, 4]),
            (2, vec![1, 2, 3, 4], vec![1, 0, 0, 4]),

            (1, vec![1, 2, 3, 4], vec![1, 2, 3, 0]),
            (2, vec![1, 2, 3, 4], vec![1, 2, 0, 0]),
            (3, vec![1, 2, 3, 4], vec![1, 0, 0, 0]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(damlev.distance(s1, s2), *d);
        }
    }

    #[test]
    fn trans_continuous() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (1, vec![1, 2, 3, 4], vec![2, 1, 3, 4]), // swap 1 and 2
            (2, vec![1, 2, 3, 4], vec![2, 1, 4, 3]), // swap 3 and 4
            (3, vec![1, 2, 3, 4], vec![2, 4, 1, 3]), // swap 1 and 4
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(damlev.distance(s1, s2), *d);
        }
    }

    #[test]
    fn add_del_intermittent() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (1, vec![1, 2, 3], vec![0, 1, 2, 3]),
            (2, vec![1, 2, 3], vec![0, 1, 0, 2, 3]),
            (3, vec![1, 2, 3], vec![0, 1, 0, 2, 0, 3]),

            (1, vec![1, 2, 3], vec![1, 2, 3, 0]),
            (2, vec![1, 2, 3], vec![1, 2, 0, 3, 0]),
            (3, vec![1, 2, 3], vec![1, 0, 2, 0, 3, 0]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(damlev.distance(s1, s2), *d);
            assert_eq!(damlev.distance(s2, s1), *d);
        }
    }

    #[test]
    fn sub_intermittent() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (1, vec![1, 2, 3, 4], vec![0, 2, 3, 4]),
            (2, vec![1, 2, 3, 4], vec![0, 2, 0, 4]),

            (1, vec![1, 2, 3, 4], vec![1, 2, 3, 0]),
            (2, vec![1, 2, 3, 4], vec![1, 0, 3, 0]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(damlev.distance(s1, s2), *d);
        }
    }

    #[test]
    fn growth() {
        let damlev = DamerauLevenshtein::new();
        for len in (1..501).step_by(100) {
            let mut v1 = Vec::with_capacity(len);
            let mut v2 = Vec::with_capacity(len);
            v1.resize(len, 1);
            v2.resize(len, 2);
            assert_eq!(damlev.distance(&v1, &v1), 0);
            assert_eq!(damlev.distance(&v1, &[]), len);
            assert_eq!(damlev.distance(&v1, &v2), len);
        }
    }
}
