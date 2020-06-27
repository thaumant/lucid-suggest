use std::collections::HashMap;
use crate::utils::FadingWindows;

const NORM_MAX_PATTERN_LEN: usize = 2;


pub struct Normalize<'a> {
    windows: FadingWindows<'a, char>,
    map:     &'a HashMap<Vec<char>, Vec<char>>,
    skip:    usize,
}


impl<'a> Normalize<'a> {
    pub fn new(source: &'a [char], map: &'a HashMap<Vec<char>, Vec<char>>) -> Self {
        Self {
            windows: FadingWindows::new(source, NORM_MAX_PATTERN_LEN),
            map,
            skip: 0,
        }
    }
}


impl<'a> Iterator for Normalize<'a> {
    type Item = (&'a [char], &'a [char]);

    fn next(&mut self) -> Option<Self::Item> {
        let mut window = self.windows.next()?;

        while self.skip > 0 {
            window = self.windows.next()?;
            self.skip -= 1;
        }

        for len in (1 .. window.len() + 1).rev() {
            let pattern = &window[..len];
            if let Some(replace) = self.map.get(pattern) {
                self.skip = pattern.len() - 1;
                return Some((pattern, replace));
            }
        }

        Some((&window[..1], &window[..1]))
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use insta::assert_debug_snapshot;
    use crate::utils::{to_vec, to_str};
    use super::Normalize;

    fn normalize<'a>(word: &'a [char]) -> (Vec<String>, Vec<String>) {
        let mut norm_map = HashMap::new();
        norm_map.insert(to_vec("ó"), to_vec("o"));
        norm_map.insert(to_vec("ó"), to_vec("o"));
        let mut source = Vec::new();
        let mut norm   = Vec::new();
        for (chunk1, chunk2) in Normalize::new(word, &norm_map) {
            source.push(to_str(chunk1));
            norm  .push(to_str(chunk2));
        }
        (source, norm)
    }

    #[test]
    fn unicode_normalize_empty() {
        let input  = to_vec("");
        let output = normalize(&input[..]);
        assert_debug_snapshot!(output);
    }

    #[test]
    fn unicode_normalize_singleton_normal() {
        let input  = to_vec("f");
        let output = normalize(&input[..]);
        assert_debug_snapshot!(output);
    }

    #[test]
    fn unicode_normalize_singleton_replaced() {
        let input  = to_vec("ó");
        let output = normalize(&input[..]);
        assert_debug_snapshot!(output);
    }

    #[test]
    fn unicode_normalize_single_nfd_replaced() {
        let input  = to_vec("ó");
        let output = normalize(&input[..]);
        assert_debug_snapshot!(output);
    }

    #[test]
    fn unicode_normalize_normal() {
        let input  = to_vec("foobar");
        let output = normalize(&input[..]);
        assert_debug_snapshot!(output);
    }

    #[test]
    fn unicode_normalize_nfc_beginning() {
        let input  = to_vec("óoobar");
        let output = normalize(&input[..]);
        assert_debug_snapshot!(output);
    }

    #[test]
    fn unicode_normalize_nfd_beginning() {
        let input  = to_vec("óoobar");
        let output = normalize(&input[..]);
        assert_debug_snapshot!(output);
    }

    #[test]
    fn unicode_normalize_nfc_middle() {
        let input  = to_vec("foóbar");
        let output = normalize(&input[..]);
        assert_debug_snapshot!(output);
    }

    #[test]
    fn unicode_normalize_nfd_middle() {
        let input  = to_vec("foóbar");
        let output = normalize(&input[..]);
        assert_debug_snapshot!(output);
    }

    #[test]
    fn unicode_normalize_nfc_end() {
        let input  = to_vec("foobaó");
        let output = normalize(&input[..]);
        assert_debug_snapshot!(output);
    }

    #[test]
    fn unicode_normalize_nfd_end() {
        let input  = to_vec("foobaó");
        let output = normalize(&input[..]);
        assert_debug_snapshot!(output);
    }
}