mod matrix;

use std::cmp::max;
use fnv::{FnvHashMap as HashMap};
use std::cell::RefCell;
use matrix::DistMatrix;


const DEFAULT_CAPACITY: usize = 20;


pub struct DamerauLevenshtein {
    pub dists: RefCell<DistMatrix>,
    last_i1: RefCell<HashMap<char, usize>>,
}


impl DamerauLevenshtein {
    pub fn new() -> Self {
        let dists   = RefCell::new(DistMatrix::new(DEFAULT_CAPACITY + 2));
        let last_i1 = RefCell::new(HashMap::with_capacity_and_hasher(DEFAULT_CAPACITY, Default::default()));
        Self { dists, last_i1 }
    }

    pub fn distance(&self, slice1: &[char], slice2: &[char]) -> f64 {
        let dists = &mut *self.dists.borrow_mut();
        dists.grow(max(slice1.len() + 2, slice2.len() + 2));

        let last_i1 = &mut *self.last_i1.borrow_mut();
        last_i1.clear();

        for (i1, &x1) in slice1.iter().enumerate() {
            let mut l2 = 0;

            for (i2, &x2) in slice2.iter().enumerate() {
                let l1 = *last_i1.get(&x2).unwrap_or(&0);
                unsafe {
                    dists.set_unchecked(i1 + 2, i2 + 2, min4(
                        dists.get_unchecked(i1 + 2, i2 + 1) + 1.0,
                        dists.get_unchecked(i1 + 1, i2 + 2) + 1.0,
                        dists.get_unchecked(i1 + 1, i2 + 1) + ((x1 != x2) as usize) as f64,
                        dists.get_unchecked(l1, l2) + ((i1 - l1) + (i2 - l2) + 1) as f64
                    ));
                }

                if x1 == x2 { l2 = i2 + 1; }
            }
            last_i1.insert(x1, i1 + 1);
        }

        unsafe { dists.get_unchecked(slice1.len() + 1, slice2.len() + 1) }
    }
}


fn min4(x1: f64, x2: f64, x3: f64, x4: f64) -> f64 {
    let mut min = x1;
    if x2 < min { min = x2; }
    if x3 < min { min = x3; }
    if x4 < min { min = x4; }
    min
}


#[cfg(test)]
mod tests {
    use crate::utils::to_vec;
    use super::DamerauLevenshtein;

    #[test]
    fn equality() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            to_vec(""),
            to_vec("a"),
            to_vec("ab"),
            to_vec("abc"),
        ];
        for s in sample.iter() {
            assert_eq!(damlev.distance(s, s), 0.0);
        }
    }

    #[test]
    fn prefix() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (0.0, to_vec("abc"), to_vec("abc")),
            (1.0, to_vec("abc"), to_vec("ab")),
            (2.0, to_vec("abc"), to_vec("a")),
            (3.0, to_vec("abc"), vec![]),
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
            (1.0, to_vec("abc"), to_vec("_abc")),
            (2.0, to_vec("abc"), to_vec("__abc")),
            (3.0, to_vec("abc"), to_vec("___abc")),

            (1.0, to_vec("abc"), to_vec("a_bc")),
            (2.0, to_vec("abc"), to_vec("a__bc")),
            (3.0, to_vec("abc"), to_vec("a___bc")),

            (1.0, to_vec("abc"), to_vec("abc_")),
            (2.0, to_vec("abc"), to_vec("abc__")),
            (3.0, to_vec("abc"), to_vec("abc___")),
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
            (1.0, to_vec("abcd"), to_vec("_bcd")),
            (2.0, to_vec("abcd"), to_vec("__cd")),
            (3.0, to_vec("abcd"), to_vec("___d")),

            (1.0, to_vec("abcd"), to_vec("a_cd")),
            (2.0, to_vec("abcd"), to_vec("a__d")),

            (1.0, to_vec("abcd"), to_vec("abc_")),
            (2.0, to_vec("abcd"), to_vec("ab__")),
            (3.0, to_vec("abcd"), to_vec("a___")),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(damlev.distance(s1, s2), *d);
        }
    }

    #[test]
    fn trans_continuous() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (1.0, to_vec("abcd"), to_vec("bacd")), // swap 1 and 2
            (2.0, to_vec("abcd"), to_vec("badc")), // swap 3 and 4
            (3.0, to_vec("abcd"), to_vec("bdac")), // swap 1 and 4
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(damlev.distance(s1, s2), *d);
        }
    }

    #[test]
    fn add_del_intermittent() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (1.0, to_vec("abc"), to_vec("_abc")),
            (2.0, to_vec("abc"), to_vec("_a_bc")),
            (3.0, to_vec("abc"), to_vec("_a_b_c")),

            (1.0, to_vec("abc"), to_vec("abc_")),
            (2.0, to_vec("abc"), to_vec("ab_c_")),
            (3.0, to_vec("abc"), to_vec("a_b_c_")),
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
            (1.0, to_vec("abcd"), to_vec("_bcd")),
            (2.0, to_vec("abcd"), to_vec("_b_d")),

            (1.0, to_vec("abcd"), to_vec("abc_")),
            (2.0, to_vec("abcd"), to_vec("a_c_")),
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
            v1.resize(len, 'a');
            v2.resize(len, 'b');
            assert_eq!(damlev.distance(&v1, &v1), 0.0);
            assert_eq!(damlev.distance(&v1, &[]), len as f64);
            assert_eq!(damlev.distance(&v1, &v2), len as f64);
        }
    }
}
