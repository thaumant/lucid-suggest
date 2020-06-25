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

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.windows.size_hint()
    }
}


#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use super::super::utils::compile_utf_map;
    use super::Normalize;

    fn normalize<'a>(word: &'a [char]) -> (Vec<String>, Vec<String>) {
        let norm_map = compile_utf_map(&[
            ("ó", "o"),
            ("ó", "o"),
            ]);
        let mut source = Vec::new();
        let mut norm   = Vec::new();
        for (chunk1, chunk2) in Normalize::new(word, &norm_map) {
            source.push(chunk1.iter().collect::<String>());
            norm  .push(chunk2.iter().collect::<String>());
        }
        (source, norm)
    }

    #[test]
    fn utf_normalize_empty() {
        let input  = "".chars().collect::<Vec<_>>();
        let output = normalize(&input[..]);
        assert_debug_snapshot!(output);
    }

    #[test]
    fn utf_normalize_singleton_normal() {
        let input  = "f".chars().collect::<Vec<_>>();
        let output = normalize(&input[..]);
        assert_debug_snapshot!(output);
    }

    #[test]
    fn utf_normalize_singleton_replaced() {
        let input  = "ó".chars().collect::<Vec<_>>();
        let output = normalize(&input[..]);
        assert_debug_snapshot!(output);
    }

    #[test]
    fn utf_normalize_single_nfd_replaced() {
        let input  = "ó".chars().collect::<Vec<_>>();
        let output = normalize(&input[..]);
        assert_debug_snapshot!(output);
    }

    #[test]
    fn utf_normalize_normal() {
        let input  = "foobar".chars().collect::<Vec<_>>();
        let output = normalize(&input[..]);
        assert_debug_snapshot!(output);
    }

    #[test]
    fn utf_normalize_nfc_beginning() {
        let input  = "óoobar".chars().collect::<Vec<_>>();
        let output = normalize(&input[..]);
        assert_debug_snapshot!(output);
    }

    #[test]
    fn utf_normalize_nfd_beginning() {
        let input  = "óoobar".chars().collect::<Vec<_>>();
        let output = normalize(&input[..]);
        assert_debug_snapshot!(output);
    }

    #[test]
    fn utf_normalize_nfc_middle() {
        let input  = "foóbar".chars().collect::<Vec<_>>();
        let output = normalize(&input[..]);
        assert_debug_snapshot!(output);
    }

    #[test]
    fn utf_normalize_nfd_middle() {
        let input  = "foóbar".chars().collect::<Vec<_>>();
        let output = normalize(&input[..]);
        assert_debug_snapshot!(output);
    }

    #[test]
    fn utf_normalize_nfc_end() {
        let input  = "foobaó".chars().collect::<Vec<_>>();
        let output = normalize(&input[..]);
        assert_debug_snapshot!(output);
    }

    #[test]
    fn utf_normalize_nfd_end() {
        let input  = "foobaó".chars().collect::<Vec<_>>();
        let output = normalize(&input[..]);
        assert_debug_snapshot!(output);
    }
}