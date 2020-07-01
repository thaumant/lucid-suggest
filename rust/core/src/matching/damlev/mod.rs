mod matrix;

use std::cmp::max;
use fnv::{FnvHashMap as HashMap};
use std::cell::RefCell;
use crate::tokenization::{Word, WordView};
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

    pub fn distance(&self, word1: &WordView, word2: &WordView) -> f64 {
        let dists = &mut *self.dists.borrow_mut();
        dists.grow(max(word1.len() + 2, word2.len() + 2));

        let last_i1 = &mut *self.last_i1.borrow_mut();
        last_i1.clear();

        for (i1, &x1) in word1.chars().iter().enumerate() {
            let mut l2 = 0;

            for (i2, &x2) in word2.chars().iter().enumerate() {
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

        unsafe { dists.get_unchecked(word1.len() + 1, word2.len() + 1) }
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
    use crate::tokenization::Text;
    use super::DamerauLevenshtein;


    #[test]
    fn equality() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            Text::from_str(""),
            Text::from_str("k"),
            Text::from_str("kl"),
            Text::from_str("klm"),
        ];
        for text in sample.iter() {
            assert_eq!(damlev.distance(&text.view(0), &text.view(0)), 0.0);
        }
    }

    #[test]
    fn prefix() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (0.0, Text::from_str("klm"), Text::from_str("klm")),
            (1.0, Text::from_str("klm"), Text::from_str("kl")),
            (2.0, Text::from_str("klm"), Text::from_str("k")),
            (3.0, Text::from_str("klm"), Text::from_str("")),
        ];
        for (d, t1, t2) in sample.iter() {
            assert_eq!(damlev.distance(&t1.view(0), &t2.view(0)), *d);
            assert_eq!(damlev.distance(&t2.view(0), &t1.view(0)), *d);
        }
    }

    #[test]
    fn add_del_continuous() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (1.0, Text::from_str("klm"), Text::from_str("_klm")),
            (2.0, Text::from_str("klm"), Text::from_str("__klm")),
            (3.0, Text::from_str("klm"), Text::from_str("___klm")),

            (1.0, Text::from_str("klm"), Text::from_str("k_lm")),
            (2.0, Text::from_str("klm"), Text::from_str("k__lm")),
            (3.0, Text::from_str("klm"), Text::from_str("k___lm")),

            (1.0, Text::from_str("klm"), Text::from_str("klm_")),
            (2.0, Text::from_str("klm"), Text::from_str("klm__")),
            (3.0, Text::from_str("klm"), Text::from_str("klm___")),
        ];
        for (d, t1, t2) in sample.iter() {
            assert_eq!(damlev.distance(&t1.view(0), &t2.view(0)), *d);
            assert_eq!(damlev.distance(&t2.view(0), &t1.view(0)), *d);
        }
    }

    #[test]
    fn sub_continuous() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (1.0, Text::from_str("klmn"), Text::from_str("_lmn")),
            (2.0, Text::from_str("klmn"), Text::from_str("__mn")),
            (3.0, Text::from_str("klmn"), Text::from_str("___n")),

            (1.0, Text::from_str("klmn"), Text::from_str("k_mn")),
            (2.0, Text::from_str("klmn"), Text::from_str("k__n")),

            (1.0, Text::from_str("klmn"), Text::from_str("klm_")),
            (2.0, Text::from_str("klmn"), Text::from_str("kl__")),
            (3.0, Text::from_str("klmn"), Text::from_str("k___")),
        ];
        for (d, t1, t2) in sample.iter() {
            assert_eq!(damlev.distance(&t1.view(0), &t2.view(0)), *d);
        }
    }

    #[test]
    fn trans_continuous() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (1.0, Text::from_str("klmn"), Text::from_str("lkmn")), // swap 1 and 2
            (2.0, Text::from_str("klmn"), Text::from_str("lknm")), // swap 3 and 4
            (3.0, Text::from_str("klmn"), Text::from_str("lnkm")), // swap 1 and 4
        ];
        for (d, t1, t2) in sample.iter() {
            assert_eq!(damlev.distance(&t1.view(0), &t2.view(0)), *d);
        }
    }

    #[test]
    fn add_del_intermittent() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (1.0, Text::from_str("klm"), Text::from_str("_klm")),
            (2.0, Text::from_str("klm"), Text::from_str("_k_lm")),
            (3.0, Text::from_str("klm"), Text::from_str("_k_l_m")),

            (1.0, Text::from_str("klm"), Text::from_str("klm_")),
            (2.0, Text::from_str("klm"), Text::from_str("kl_m_")),
            (3.0, Text::from_str("klm"), Text::from_str("k_l_m_")),
        ];
        for (d, t1, t2) in sample.iter() {
            assert_eq!(damlev.distance(&t1.view(0), &t2.view(0)), *d);
            assert_eq!(damlev.distance(&t2.view(0), &t1.view(0)), *d);
        }
    }

    #[test]
    fn sub_intermittent() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (1.0, Text::from_str("klmn"), Text::from_str("_lmn")),
            (2.0, Text::from_str("klmn"), Text::from_str("_l_n")),

            (1.0, Text::from_str("klmn"), Text::from_str("klm_")),
            (2.0, Text::from_str("klmn"), Text::from_str("k_m_")),
        ];
        for (d, t1, t2) in sample.iter() {
            assert_eq!(damlev.distance(&t1.view(0), &t2.view(0)), *d);
        }
    }

    #[test]
    fn growth() {
        let damlev = DamerauLevenshtein::new();
        for len in (1..501).step_by(100) {
            let mut s1 = String::with_capacity(len);
            let mut s2 = String::with_capacity(len);
            for _ in 0..len { s1.push('k'); }
            for _ in 0..len { s2.push('l'); }
            let t0 = Text::from_str("");
            let t1 = Text::from_str(&s1);
            let t2 = Text::from_str(&s2);
            assert_eq!(damlev.distance(&t1.view(0), &t1.view(0)), 0.0);
            assert_eq!(damlev.distance(&t1.view(0), &t0.view(0)), len as f64);
            assert_eq!(damlev.distance(&t1.view(0), &t2.view(0)), len as f64);
        }
    }
}
