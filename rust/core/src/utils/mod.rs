#![macro_use]

mod limitsort;
mod fading_windows;

pub use limitsort::{LimitSortIterator, LimitSort};
pub use fading_windows::FadingWindows;


macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => (::std::cmp::min($x, min!($($z),*)));
}


macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => (::std::cmp::max($x, max!($($z),*)));
}
