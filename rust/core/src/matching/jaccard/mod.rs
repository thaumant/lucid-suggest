use std::cmp::Ordering;
use std::cell::RefCell;

use Ordering::{
    Less,
    Equal,
    Greater,
};


const DEFAULT_CAPACITY: usize = 20;


pub fn simple_similarity<T: PartialEq + Copy + Ord>(set1: &[T], set2: &[T]) -> f64 {
    let mut i1 = 0;
    let mut i2 = 0;
    let mut union = 0;
    let mut intersection = 0;
    while i1 < set1.len() && i2 < set2.len() {
        let item1 = unsafe { *set1.get_unchecked(i1) };
        let item2 = unsafe { *set2.get_unchecked(i2) };
        union += 1;
        match item1.cmp(&item2) {
            Less    => i1 += 1,
            Greater => i2 += 1,
            Equal   => {
                intersection += 1;
                i1 += 1;
                i2 += 1;
            },
        }
    }
    union += set1.len() - i1;
    union += set2.len() - i2;
    intersection as f64 / union as f64
}


pub struct Jaccard<T: PartialEq + Copy + Ord + Default> {
    set1: RefCell<Vec<T>>,
    set2: RefCell<Vec<T>>,
}


impl<T: PartialEq + Copy + Ord + Default> Jaccard<T> {
    pub fn new() -> Self {
        Self {
            set1: RefCell::new(Vec::with_capacity(DEFAULT_CAPACITY)),
            set2: RefCell::new(Vec::with_capacity(DEFAULT_CAPACITY)),
        }
    }

    pub fn similarity(&self, slice1: &[T], slice2: &[T]) -> f64 {
        match (slice1.len(), slice2.len()) {
            (0, 0) => return 1.0,
            (0, _) => return 0.0,
            (_, 0) => return 0.0,
            (_, _) => { },
        }
        let set1 = &mut *self.set1.borrow_mut();
        let set2 = &mut *self.set2.borrow_mut();
        set1.resize(slice1.len(), Default::default());
        set2.resize(slice2.len(), Default::default());
        set1.copy_from_slice(&slice1);
        set2.copy_from_slice(&slice2);
        set1.sort_unstable();
        set2.sort_unstable();
        set1.dedup();
        set2.dedup();
        simple_similarity(&set1, &set2)
    }

    pub fn rel_dist(&self, slice1: &[T], slice2: &[T]) -> f64 {
        1.0 - self.similarity(slice1, slice2)
    }
}


#[cfg(test)]
mod tests {
    use super::Jaccard;


    fn round2(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }


    #[test]
    fn empty_both() {
        let jaccard = Jaccard::new();
        let s1: &[usize] = &[];
        let s2: &[usize] = &[];
        assert_eq!(round2(jaccard.rel_dist(s1, s2)), 0.0);
    }


    #[test]
    fn empty_one() {
        let jaccard = Jaccard::new();
        let sample = [
            (1.00, vec![], vec![1]),
            (1.00, vec![], vec![1, 2]),
            (1.00, vec![], vec![1, 2, 3]),
            (1.00, vec![], vec![1, 2, 3, 4]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(round2(jaccard.rel_dist(s1, s2)), *d);
        }
    }


    #[test]
    fn equal() {
        let jaccard = Jaccard::new();
        let sample = [
            vec![1],
            vec![1, 2],
            vec![1, 2, 3],
            vec![1, 2, 3, 4],
        ];
        for s in sample.iter() {
            assert_eq!(round2(jaccard.rel_dist(s, s)), 0.0);
        }
    }


    #[test]
    fn different() {
        let jaccard = Jaccard::new();
        let sample = [
            (1.00, vec![1],          vec![5]),
            (1.00, vec![1, 2],       vec![5, 6]),
            (1.00, vec![1, 2, 3],    vec![5, 6, 7]),
            (1.00, vec![1, 2, 3, 4], vec![5, 6, 7, 8]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(round2(jaccard.rel_dist(s1, s2)), *d);
        }
    }


    #[test]
    fn partial() {
        let jaccard = Jaccard::new();
        let sample = [
            (0.25, vec![1, 2, 3, 4], vec![1, 2, 3]),
            (0.50, vec![1, 2, 3, 4], vec![1, 2]),
            (0.75, vec![1, 2, 3, 4], vec![1]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(round2(jaccard.rel_dist(s1, s2)), *d);
        }
    }


    #[test]
    fn permutated() {
        let jaccard = Jaccard::new();
        let sample = [
            (0.00, vec![1, 2, 3, 4], vec![3, 1, 2, 4]),
            (0.00, vec![1, 2, 3, 4], vec![4, 2, 3, 1]),
            (0.00, vec![1, 2, 3, 4], vec![1, 4, 2 ,3]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(round2(jaccard.rel_dist(s1, s2)), *d);
        }
    }


    #[test]
    fn repeated() {
        let jaccard = Jaccard::new();
        let sample = [
            (0.75, vec![1, 2, 3, 4], vec![1, 1, 1, 1]),
            (0.50, vec![1, 2, 3, 4], vec![2, 1, 2, 1]),
            (0.25, vec![1, 2, 3, 4], vec![3, 2, 1, 3]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(round2(jaccard.rel_dist(s1, s2)), *d);
        }
    }


    #[test]
    fn complex() {
        let jaccard = Jaccard::new();
        let sample = [
            (0.40, vec![1, 2, 3, 4], vec![0, 1, 2, 4]),
            (0.40, vec![1, 2, 3, 4], vec![0, 2, 3, 1]),
            (0.50, vec![1, 2, 3, 4], vec![0, 4, 2 ,3, 5]),
        ];
        for (d, s1, s2) in sample.iter() {
            assert_eq!(round2(jaccard.rel_dist(s1, s2)), *d);
        }
    }
}