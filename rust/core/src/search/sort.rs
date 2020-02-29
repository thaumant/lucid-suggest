use std::cmp::Ordering;
use crate::search::Hit;


pub fn compare_hits(hit1: &Hit, hit2: &Hit) -> Ordering {
    hit1.scores.iter()
        .zip(hit2.scores.iter())
        .map(|(s1, s2)| s2.cmp(s1))
        .find(|&ord| ord != Ordering::Equal)
        .unwrap_or(Ordering::Equal)
}
