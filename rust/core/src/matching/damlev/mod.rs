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

                let dist_add   = unsafe { dists.get_unchecked(i1 + 2, i2 + 1) + 1.0 };
                let dist_del   = unsafe { dists.get_unchecked(i1 + 1, i2 + 2) + 1.0 };
                let dist_sub   = unsafe { dists.get_unchecked(i1 + 1, i2 + 1) + ((x1 != x2) as usize) as f64 };
                let dist_trans = unsafe { dists.get_unchecked(l1, l2) + ((i1 - l1) + (i2 - l2) + 1) as f64 };
                let dist       = min4(dist_add, dist_del, dist_sub, dist_trans);

                unsafe {
                    dists.set_unchecked(i1 + 2, i2 + 2, dist);
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
            to_vec("k"),
            to_vec("kl"),
            to_vec("klm"),
        ];
        for s in sample.iter() {
            assert_eq!(damlev.distance(s, s), 0.0);
        }
    }

    #[test]
    fn prefix() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (0.0, to_vec("klm"), to_vec("klm")),
            (1.0, to_vec("klm"), to_vec("kl")),
            (2.0, to_vec("klm"), to_vec("k")),
            (3.0, to_vec("klm"), vec![]),
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
            (1.0, to_vec("klm"), to_vec("_klm")),
            (2.0, to_vec("klm"), to_vec("__klm")),
            (3.0, to_vec("klm"), to_vec("___klm")),

            (1.0, to_vec("klm"), to_vec("k_lm")),
            (2.0, to_vec("klm"), to_vec("k__lm")),
            (3.0, to_vec("klm"), to_vec("k___lm")),

            (1.0, to_vec("klm"), to_vec("klm_")),
            (2.0, to_vec("klm"), to_vec("klm__")),
            (3.0, to_vec("klm"), to_vec("klm___")),
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
            (1.0, to_vec("klmn"), to_vec("_lmn")),
            (2.0, to_vec("klmn"), to_vec("__mn")),
            (3.0, to_vec("klmn"), to_vec("___n")),

            (1.0, to_vec("klmn"), to_vec("k_mn")),
            (2.0, to_vec("klmn"), to_vec("k__n")),

            (1.0, to_vec("klmn"), to_vec("klm_")),
            (2.0, to_vec("klmn"), to_vec("kl__")),
            (3.0, to_vec("klmn"), to_vec("k___")),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(damlev.distance(s1, s2), *d);
        }
    }

    #[test]
    fn trans_continuous() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (1.0, to_vec("klmn"), to_vec("lkmn")), // swap 1 and 2
            (2.0, to_vec("klmn"), to_vec("lknm")), // swap 3 and 4
            (3.0, to_vec("klmn"), to_vec("lnkm")), // swap 1 and 4
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(damlev.distance(s1, s2), *d);
        }
    }

    #[test]
    fn add_del_intermittent() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (1.0, to_vec("klm"), to_vec("_klm")),
            (2.0, to_vec("klm"), to_vec("_k_lm")),
            (3.0, to_vec("klm"), to_vec("_k_l_m")),

            (1.0, to_vec("klm"), to_vec("klm_")),
            (2.0, to_vec("klm"), to_vec("kl_m_")),
            (3.0, to_vec("klm"), to_vec("k_l_m_")),
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
            (1.0, to_vec("klmn"), to_vec("_lmn")),
            (2.0, to_vec("klmn"), to_vec("_l_n")),

            (1.0, to_vec("klmn"), to_vec("klm_")),
            (2.0, to_vec("klmn"), to_vec("k_m_")),
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
            v1.resize(len, 'k');
            v2.resize(len, 'l');
            assert_eq!(damlev.distance(&v1, &v1), 0.0);
            assert_eq!(damlev.distance(&v1, &[]), len as f64);
            assert_eq!(damlev.distance(&v1, &v2), len as f64);
        }
    }
}
