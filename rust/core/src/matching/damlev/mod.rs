mod matrix;

use std::f64;
use fnv::{FnvHashMap as HashMap};
use std::cell::RefCell;
use crate::lang::CharClass;
use crate::tokenization::{Word, WordView};
use matrix::DistMatrix;


const DEFAULT_CAPACITY: usize = 20;

const COST_TRANS:     f64 = 0.5;
const COST_DOUBLE:    f64 = 0.5;
const COST_VOWEL:     f64 = 0.5;
const COST_NOTALPHA:  f64 = 0.5;
const COST_CONSONANT: f64 = 1.0;
const COST_DEFAULT:   f64 = 1.0;


pub struct DamerauLevenshtein {
    pub dists: RefCell<DistMatrix>,
    last_i1: RefCell<HashMap<char, usize>>,
    costs1:  RefCell<Vec<f64>>,
    costs2:  RefCell<Vec<f64>>,
}


impl DamerauLevenshtein {
    pub fn new() -> Self {
        let dists   = RefCell::new(DistMatrix::new(DEFAULT_CAPACITY + 2));
        let last_i1 = RefCell::new(HashMap::with_capacity_and_hasher(DEFAULT_CAPACITY, Default::default()));
        let costs1  = RefCell::new(Vec::with_capacity(DEFAULT_CAPACITY));
        let costs2  = RefCell::new(Vec::with_capacity(DEFAULT_CAPACITY));
        Self { dists, last_i1, costs1, costs2 }
    }

    fn get_cost(class: &CharClass) -> f64 {
        match class {
            CharClass::Consonant => COST_CONSONANT,
            CharClass::Vowel     => COST_VOWEL,
            CharClass::NotAlpha  => COST_NOTALPHA,
            _                    => COST_DEFAULT,
        }
    }

    pub fn distance(&self, word1: &WordView, word2: &WordView) -> f64 {
        let chars1 = word1.chars();
        let chars2 = word2.chars();

        let costs1 = &mut *self.costs1.borrow_mut();
        let costs2 = &mut *self.costs2.borrow_mut();
        costs1.clear();
        costs2.clear();
        costs1.extend(word1.classes().iter().map(Self::get_cost));
        costs2.extend(word2.classes().iter().map(Self::get_cost));

        let dists = &mut *self.dists.borrow_mut();
        dists.prepare(&costs1, &costs2);

        let last_i1 = &mut *self.last_i1.borrow_mut();
        last_i1.clear();

        for (i1, &ch1) in chars1.iter().enumerate() {
            let mut l2 = 0;

            let cost1        = unsafe { *costs1.get_unchecked(i1) };
            let double1      = i1 > 0 && ch1 == unsafe { *chars1.get_unchecked(i1 - 1) };
            let cost_double1 = if double1 { COST_DOUBLE } else { COST_DEFAULT };
            let cost_del     = fmin(cost1, cost_double1);

            for (i2, &ch2) in chars2.iter().enumerate() {
                let l1 = *last_i1.get(&ch2).unwrap_or(&0);

                let cost2        = unsafe { *costs2.get_unchecked(i2) };
                let double2      = i2 > 0 && ch2 == unsafe { *chars2.get_unchecked(i2 - 1) };
                let cost_double2 = if double2 { COST_DOUBLE } else { COST_DEFAULT };
                let cost_add     = fmin(cost2, cost_double2);
                let cost_sub     = if ch1 == ch2 { 0.0 } else { fmax(cost1, cost2) };
                let cost_trans   = COST_TRANS * ((i1 - l1) + (i2 - l2) + 1) as f64;

                let dist_add   = cost_add   + unsafe { dists.get_unchecked(i1 + 2, i2 + 1) };
                let dist_del   = cost_del   + unsafe { dists.get_unchecked(i1 + 1, i2 + 2) };
                let dist_sub   = cost_sub   + unsafe { dists.get_unchecked(i1 + 1, i2 + 1) };
                let dist_trans = cost_trans + unsafe { dists.get_unchecked(l1, l2) };
                let dist       = fmin4(dist_add, dist_del, dist_sub, dist_trans);

                unsafe {
                    dists.set_unchecked(i1 + 2, i2 + 2, dist);
                }

                if ch1 == ch2 { l2 = i2 + 1; }
            }
            last_i1.insert(ch1, i1 + 1);
        }

        unsafe { dists.get_unchecked(word1.len() + 1, word2.len() + 1) }
    }
}


#[inline]
fn fmin4(x1: f64, x2: f64, x3: f64, x4: f64) -> f64 {
    let mut min = x1;
    if x2 < min { min = x2; }
    if x3 < min { min = x3; }
    if x4 < min { min = x4; }
    min
}

#[inline]
fn fmin(x1: f64, x2: f64) -> f64 {
    if x1 < x2 { x1 } else { x2 }
}

#[inline]
fn fmax(x1: f64, x2: f64) -> f64 {
    if x1 > x2 { x1 } else { x2 }
}


#[cfg(test)]
mod tests {
    use crate::lang::lang_english;
    use crate::tokenization::Text;
    use super::DamerauLevenshtein;


    #[test]
    fn equality() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            Text::from_str(""),
            Text::from_str("a"),
            Text::from_str("ab"),
            Text::from_str("abc"),
        ];
        for text in sample.iter() {
            assert_eq!(damlev.distance(&text.view(0), &text.view(0)), 0.0);
        }
    }

    #[test]
    fn prefix() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (0.0, Text::from_str("abc"), Text::from_str("abc")),
            (1.0, Text::from_str("abc"), Text::from_str("ab")),
            (2.0, Text::from_str("abc"), Text::from_str("a")),
            (3.0, Text::from_str("abc"), Text::from_str("")),
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
            (1.0, Text::from_str("abc"), Text::from_str("xabc")),
            (2.0, Text::from_str("abc"), Text::from_str("xyabc")),
            (3.0, Text::from_str("abc"), Text::from_str("xyzabc")),

            (1.0, Text::from_str("abc"), Text::from_str("axbc")),
            (2.0, Text::from_str("abc"), Text::from_str("axybc")),
            (3.0, Text::from_str("abc"), Text::from_str("axyzbc")),

            (1.0, Text::from_str("abc"), Text::from_str("abcx")),
            (2.0, Text::from_str("abc"), Text::from_str("abcxy")),
            (3.0, Text::from_str("abc"), Text::from_str("abcxyz")),
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
            (1.0, Text::from_str("abcd"), Text::from_str("xbcd")),
            (2.0, Text::from_str("abcd"), Text::from_str("xycd")),
            (3.0, Text::from_str("abcd"), Text::from_str("xyzd")),

            (1.0, Text::from_str("abcd"), Text::from_str("axcd")),
            (2.0, Text::from_str("abcd"), Text::from_str("axyd")),

            (1.0, Text::from_str("abcd"), Text::from_str("abcz")),
            (2.0, Text::from_str("abcd"), Text::from_str("abyz")),
            (3.0, Text::from_str("abcd"), Text::from_str("axyz")),
        ];
        for (d, t1, t2) in sample.iter() {
            assert_eq!(damlev.distance(&t1.view(0), &t2.view(0)), *d);
        }
    }

    #[test]
    fn trans_continuous() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (0.5, Text::from_str("abcd"), Text::from_str("bacd")), // swap 1 and 2
            (1.0, Text::from_str("abcd"), Text::from_str("badc")), // swap 3 and 4
        ];
        for (d, t1, t2) in sample.iter() {
            assert_eq!(damlev.distance(&t1.view(0), &t2.view(0)), *d);
            assert_eq!(damlev.distance(&t2.view(0), &t1.view(0)), *d);
        }
    }

    #[test]
    fn add_del_intermittent() {
        let damlev = DamerauLevenshtein::new();
        let sample = [
            (1.0, Text::from_str("abc"), Text::from_str("xabc")),
            (2.0, Text::from_str("abc"), Text::from_str("xaybc")),
            (3.0, Text::from_str("abc"), Text::from_str("xaybzc")),

            (1.0, Text::from_str("abc"), Text::from_str("abcz")),
            (2.0, Text::from_str("abc"), Text::from_str("abycz")),
            (3.0, Text::from_str("abc"), Text::from_str("axbycz")),
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
            (1.0, Text::from_str("abcd"), Text::from_str("xbcd")),
            (2.0, Text::from_str("abcd"), Text::from_str("xbyd")),

            (1.0, Text::from_str("abcd"), Text::from_str("abcy")),
            (2.0, Text::from_str("abcd"), Text::from_str("axcy")),
        ];
        for (d, t1, t2) in sample.iter() {
            assert_eq!(damlev.distance(&t1.view(0), &t2.view(0)), *d);
            assert_eq!(damlev.distance(&t2.view(0), &t1.view(0)), *d);
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

    #[test]
    pub fn add_del_lang_consonant() {
        let lang   = lang_english();
        let damlev = DamerauLevenshtein::new();
        let text   = |s| Text::from_str(s).set_char_classes(&lang);
        let sample = [
            (1.0, text("pink"), text("spink")),
            (2.0, text("pink"), text("shpink")),
            (3.0, text("pink"), text("schpink")),

            (1.0, text("pink"), text("plink")),
            (2.0, text("pink"), text("prlink")),

            (1.0, text("pink"), text("pinks")),
            (2.0, text("pink"), text("pinkst")),
            (3.0, text("pink"), text("pinkstr")),
        ];
        for (d, t1, t2) in sample.iter() {
            assert_eq!(damlev.distance(&t1.view(0), &t2.view(0)), *d);
            assert_eq!(damlev.distance(&t2.view(0), &t1.view(0)), *d);
        }
    }

    #[test]
    pub fn add_del_lang_vowel() {
        let lang   = lang_english();
        let damlev = DamerauLevenshtein::new();
        let text   = |s| Text::from_str(s).set_char_classes(&lang);
        let sample = [
            (0.5, text("pink"), text("opink")),
            (1.0, text("pink"), text("aopink")),
            (1.5, text("pink"), text("aiopink")),

            (0.5, text("pink"), text("poink")),
            (1.0, text("pink"), text("paoink")),

            (0.5, text("pink"), text("pinky")),
            (1.0, text("pink"), text("pinkie")),
            (1.5, text("pink"), text("pinkaio")),
        ];
        for (d, t1, t2) in sample.iter() {
            assert_eq!(damlev.distance(&t1.view(0), &t2.view(0)), *d);
            assert_eq!(damlev.distance(&t2.view(0), &t1.view(0)), *d);
        }
    }

    #[test]
    fn sub_lang() {
        let lang   = lang_english();
        let damlev = DamerauLevenshtein::new();
        let text   = |s| Text::from_str(s).set_char_classes(&lang);
        let sample = [
            (0.5, text("pinky"), text("punky")),
            (1.0, text("pinky"), text("psnky")),
            (1.0, text("pinky"), text("ponko")),
            (1.5, text("pinky"), text("pesky")),
            (2.0, text("pinky"), text("psnkn")),
        ];
        for (d, t1, t2) in sample.iter() {
            assert_eq!(damlev.distance(&t1.view(0), &t2.view(0)), *d);
            assert_eq!(damlev.distance(&t2.view(0), &t1.view(0)), *d);
        }
    }

    #[test]
    fn add_del_double_lang() {
        let lang   = lang_english();
        let damlev = DamerauLevenshtein::new();
        let text   = |s| Text::from_str(s).set_char_classes(&lang);
        let sample = [
            // vowel end
            (0.5, text("pink"),  text("pinky")),
            (1.0, text("pink"),  text("pinkyy")),
            // vowel mid
            (0.5, text("pinky"),  text("piinky")),
            (1.0, text("pinky"),  text("piiinky")),
            // consonant mid
            (1.5, text("pinky"),  text("pinkxxy")),
            (2.0, text("pinky"),  text("pinkxxxy")),
            // consonant end
            (1.5, text("pinky"),  text("pinkyss")),
            (2.0, text("pinky"),  text("pinkysss")),
        ];
        for (d, t1, t2) in sample.iter() {
            assert_eq!(damlev.distance(&t1.view(0), &t2.view(0)), *d);
            assert_eq!(damlev.distance(&t2.view(0), &t1.view(0)), *d);
        }
    }

    #[test]
    fn sub_double_lang() {
        let lang   = lang_english();
        let damlev = DamerauLevenshtein::new();
        let text   = |s| Text::from_str(s).set_char_classes(&lang);
        let sample = [
            // consonant mid
            (0.5, text("pinky"), text("pinkky")),
            (1.0, text("pinky"), text("pinkkky")),
            (1.5, text("pinky"), text("pinkkkky")),
            // vowel end
            (0.5, text("pinky"), text("pinkyy")),
            (1.0, text("pinky"), text("pinkyyy")),
            (1.5, text("pinky"), text("pinkyyyy")),
            // consonant begin
            (0.5, text("pinky"), text("ppinky")),
            (1.0, text("pinky"), text("pppinky")),
            (1.5, text("pinky"), text("ppppinky")),
            // consonant mid
            (1.5, text("pinky"), text("pillky")),
            (2.0, text("pinky"), text("pilllky")),
            // vowel mid
            (1.0, text("pinky"), text("poonky")),
            (1.5, text("pinky"), text("pooonky")),
        ];
        for (d, t1, t2) in sample.iter() {
            assert_eq!(damlev.distance(&t1.view(0), &t2.view(0)), *d);
            assert_eq!(damlev.distance(&t2.view(0), &t1.view(0)), *d);
        }
    }

    #[test]
    fn notalpha_lang() {
        let lang   = lang_english();
        let damlev = DamerauLevenshtein::new();
        let text   = |s| Text::from_str(s).set_char_classes(&lang);
        let sample = [
            (0.5, text("pinky"), text("p_nky")),
            (1.0, text("pinky"), text("p_nk_")),
            (2.0, text("pinky"), text("__nk_")),
        ];
        for (d, t1, t2) in sample.iter() {
            assert_eq!(damlev.distance(&t1.view(0), &t2.view(0)), *d);
            assert_eq!(damlev.distance(&t2.view(0), &t1.view(0)), *d);
        }
    }
}
