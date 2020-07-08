use std::cmp::Ordering;
use fnv::{FnvHashMap as HashMap};
use crate::utils::{Trigrams, LimitSort};
use crate::tokenization::{Word, TextRef};
use super::Record;

use Ordering::{
    Less,
    Equal,
    Greater,
};


pub struct TrigramIndex {
    len:     usize,
    dict:    HashMap<[char; 3], Vec<usize>>,
    counts:  Vec<usize>,
}


impl TrigramIndex {
    pub fn new() -> Self {
        Self {
            len:    0,
            dict:   HashMap::default(),
            counts: Vec::new(),
        }
    }

    pub fn add(&mut self, record: &Record) {
        let Self { dict, len, .. } = self;
        let Record { ix, title, .. } = record;
        let grams = Self::collect_grams(&title.to_ref());
        *len += 1;
        for &gram in grams.iter() {
            dict
                .entry(gram)
                .and_modify(|ixs| {
                    for i in 0..ixs.len() {
                        match ix.cmp(unsafe { ixs.get_unchecked(i) }) {
                            Greater => continue,
                            Equal   => return,
                            Less    => return ixs.insert(i, *ix),
                        }
                    }
                    ixs.push(*ix);
                })
                .or_insert_with(|| vec![*ix]);
        }
    }

    pub fn prepare(
        &mut self,
        query:   &TextRef,
        size:    usize,
    ) -> Vec<usize> {
        let Self { counts, dict, .. } = self;

        if query.words.len() == 0 {
            return Vec::new();
        }

        counts.clear();
        counts.resize(self.len, 0);

        let grams = Self::collect_grams(&query);

        for gram in grams.iter() {
            if let Some(ixs) = dict.get(gram) {
                for &ix in ixs {
                    unsafe {
                        *counts.get_unchecked_mut(ix) += 1;
                    }
                }
            }
        }

        counts
            .iter()
            .enumerate()
            .filter(|(_, &count)| count > 0)
            .limit_sort_unstable(size * 10, |(_, count1), (_, count2)| count2.cmp(count1))
            .map(|(ix, _)| ix)
            .collect()
    }

    fn collect_grams(text: &TextRef) -> Vec<[char; 3]> {
        let cap       = text.words.iter().map(|w| w.len()).sum::<usize>();
        let mut grams = Vec::with_capacity(cap);
        for word in text.words {
            let chars = &text.chars[word.slice.0 .. word.slice.1];
            for gram in chars.trigrams() {
                grams.push(gram);
            }
        }
        grams.sort_unstable();
        grams.dedup();
        grams
    }
}


#[cfg(test)]
mod tests {
    use insta::{assert_debug_snapshot, assert_snapshot};
    use crate::lang::Lang;
    use crate::tokenization::tokenize_query;
    use super::Record;
    use super::TrigramIndex;

    fn get_index() -> (TrigramIndex, [Record; 5]) {
        let lang = Lang::new();
        let mut records = [
            Record::new(10, "brown plush bear",     10, &lang),
            Record::new(20, "the metal detector",   20, &lang),
            Record::new(30, "yellow metal mailbox", 30, &lang),
            Record::new(40, "thesaurus",            40, &lang),
            Record::new(50, "wi-fi router",         50, &lang),
        ];
        let mut index = TrigramIndex::new();
        for (ix, record) in records.iter_mut().enumerate() {
            record.ix = ix;
            index.add(&record);
        }
        (index, records)
    }

    fn export_dict(index: &TrigramIndex) -> String {
        let mut dict = index.dict
            .iter()
            .map(|(gram, ixs)| (
                gram.iter().cloned().collect::<String>(),
                ixs.clone(),
            ))
            .collect::<Vec<_>>();

        dict.sort_by(|(g1, _), (g2, _)| g1.cmp(g2));

        let mut result = String::new();
        for (gram, ixs) in &dict {
            result.push_str(&format!("\"{:3}\" {:?}\n", gram, ixs));
        }
        result
    }

    fn check_prepare(name: &str, size: usize, queries: &[&str]) {
        let lang = Lang::new();
        let (mut index, _) = get_index();
        for (i, query) in queries.iter().enumerate() {
            let query = tokenize_query(query, &lang);
            let query = query.to_ref();
            let mut prepared = index.prepare(&query, size);
            dbg!(&query);
            prepared.sort();
            assert_debug_snapshot!(format!("{}-{}", name, i), prepared);
        }
    }

    #[test]
    fn add_first() {
        let lang       = Lang::new();
        let mut index  = TrigramIndex::new();
        index.add(&Record::new(10, "Foo Bar", 10, &lang));
        assert_snapshot!(export_dict(&index));
    }

    #[test]
    fn add_second() {
        let lang        = Lang::new();
        let mut index   = TrigramIndex::new();
        let mut record1 = Record::new(10, "Foo Bar", 10, &lang);
        let mut record2 = Record::new(20, "Bar Baz", 20, &lang);
        record1.ix = 0;
        record2.ix = 1;
        index.add(&record1);
        index.add(&record2);
        assert_snapshot!(export_dict(&index));
    }

    #[test]
    fn prepare_mismatch() {
        check_prepare("mismatch", 3, &["zzzap!"]);
    }

    #[test]
    fn prepare_all_matches() {
        check_prepare("all_matches", 3, &[
            "metal",
            "the",
            "rou",
        ]);
    }

    #[test]
    fn prepare_empty() {
        check_prepare("empty", 3, &[
            "",
        ]);
    }

    #[test]
    fn prepare_first_char() {
        check_prepare("first_char", 3, &[
            "m",
            "t",
            "r",
        ]);
    }

    #[test]
    fn prepare_two_chars() {
        check_prepare("two_chars", 3, &[
            "me",
            "th",
        ]);
    }
}
