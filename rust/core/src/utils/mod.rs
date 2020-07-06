#![macro_use]

mod trigrams;
mod limitsort;
mod fading_windows;

pub use trigrams::{Trigrams, TrigramIter};
pub use limitsort::{LimitSort, LimitSortIter};
pub use fading_windows::FadingWindows;


macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => (::std::cmp::min($x, min!($($z),*)));
}


macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => (::std::cmp::max($x, max!($($z),*)));
}


pub fn to_vec<T: AsRef<str>>(chars: T) -> Vec<char> {
    chars.as_ref().chars().collect()
}


#[cfg(test)]
pub fn to_str<T: AsRef<[char]>>(chars: T) -> String {
    chars.as_ref().iter().collect()
}
