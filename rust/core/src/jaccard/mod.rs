use std::cell::RefCell;
use std::hash::Hash;
use std::collections::HashSet;


const DEFAULT_CAPACITY: usize = 20;


pub struct Jaccard<T: PartialEq + Copy + Ord + Hash> {
    set1: RefCell<HashSet<T>>,
    set2: RefCell<HashSet<T>>,
}


impl<T: PartialEq + Copy + Ord + Hash> Jaccard<T> {
    pub fn new() -> Self {
        Self {
            set1: RefCell::new(HashSet::with_capacity(DEFAULT_CAPACITY)),
            set2: RefCell::new(HashSet::with_capacity(DEFAULT_CAPACITY)),
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

        set1.clear();
        set2.clear();
        for &item1 in slice1 { set1.insert(item1); }
        for &item2 in slice2 { set2.insert(item2); }

        let count_union        = set1.union(set2).count();
        let count_intersection = set1.intersection(set2).count();

        count_intersection as f64 / count_union as f64
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