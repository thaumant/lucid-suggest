use fnv::{FnvHashMap as HashMap, FnvHashSet as HashSet};
use crate::utils::LimitSortIterator;
use crate::tokenization::{Word, TextRef};
use super::Record;
use super::trigrams::Trigrams;


const DEFAULT_CAPACITY_DICT: usize = 10;


pub struct TrigramIndex {
    dict:    HashMap<[char; 3], HashSet<usize>>,
    counts:  HashMap<usize, usize>,
}


impl TrigramIndex {
    pub fn new() -> Self {
        Self {
            dict:   HashMap::default(),
            counts: HashMap::default(),
        }
    }

    pub fn add(&mut self, record: &Record) {
        let Self {
            dict,
            ..
        } = self;
        let Record { id, title, .. } = record;

        let grams = Self::collect_grams(&title.to_ref());

        for &gram in grams.iter() {
            dict
                .entry(gram)
                .and_modify(|ids| {
                    ids.insert(*id);
                })
                .or_insert_with(|| {
                    let mut ids = HashSet::with_capacity_and_hasher(DEFAULT_CAPACITY_DICT, Default::default());
                    ids.insert(*id);
                    ids
                });
        }
    }

    pub fn prepare(
        &mut self,
        query:   &TextRef,
        size:    usize,
    ) -> Vec<usize> {
        if query.words.len() == 0 {
            return Vec::new();
        }

        self.counts.clear();

        let grams = Self::collect_grams(&query);

        for gram in grams.iter() {
            if let Some(ids) = self.dict.get(gram) {
                for &id in ids {
                    self.counts
                        .entry(id)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }
        }

        self.counts
            .drain()
            .limit_sort(size * 10, |(_, cout1), (_, count2)| count2.cmp(cout1))
            .map(|(id, _)| id)
            .collect()
    }

    fn collect_grams(text: &TextRef) -> HashSet<[char; 3]> {
        let cap       = text.words.iter().map(|w| w.len()).sum::<usize>();
        let mut grams = HashSet::with_capacity_and_hasher(cap, Default::default());
        for word in text.words {
            let chars = &text.chars[word.place.0 .. word.place.1];
            for gram in Trigrams::new(chars) {
                grams.insert(gram);
            }
        }
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
        let lang    = Lang::new();
        let records = [
            Record::new(10, "brown plush bear",     10, &lang),
            Record::new(20, "the metal detector",   20, &lang),
            Record::new(30, "yellow metal mailbox", 30, &lang),
            Record::new(40, "thesaurus",            40, &lang),
            Record::new(50, "wi-fi router",         50, &lang),
        ];
        let mut index = TrigramIndex::new();
        for record in &records {
            index.add(&record);
        }
        (index, records)
    }

    fn export_tree(index: &TrigramIndex) -> String {
        let mut ids = index.dict
            .iter()
            .map(|(gram, ids)| {
                let gram    = gram.iter().cloned().collect::<String>();
                let mut ids = ids.iter().cloned().collect::<Vec<_>>();
                ids.sort();
                (gram, ids)
            })
            .collect::<Vec<_>>();

        ids.sort_by(|(g1, _), (g2, _)| g1.cmp(g2));

        let mut result = String::new();
        for (gram, ids) in &ids {
            result.push_str(&format!("\"{:3}\" {:?}\n", gram, ids));
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
        let lang = Lang::new();
        let mut index = TrigramIndex::new();
        index.add(&Record::new(10, "Foo Bar", 10, &lang));
        assert_snapshot!(export_tree(&index));
    }

    #[test]
    fn add_second() {
        let lang = Lang::new();
        let mut index = TrigramIndex::new();
        index.add(&Record::new(10, "Foo Bar", 10, &lang));
        index.add(&Record::new(20, "Bar Baz", 20, &lang));
        assert_snapshot!(export_tree(&index));
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
