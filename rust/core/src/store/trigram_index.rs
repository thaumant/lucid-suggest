use std::collections::{BTreeMap, HashSet, HashMap};
use crate::tokenization::Text;
use super::Record;
use super::trigrams::Trigrams;


const DEFAULT_CAPACITY_TREE:  usize = 10;
const DEFAULT_CAPACITY_COUNT: usize = 100;


pub struct TrigramIndex {
    tree:             BTreeMap<[char; 3], HashSet<usize>>,
    indexed:          HashSet<usize>,
    buffer_grams:     HashSet<[char; 3]>,
    buffer_count_map: HashMap<usize, usize>,
    buffer_count_vec: Vec<(usize, usize)>,
}


impl TrigramIndex {
    pub fn new() -> Self {
        Self {
            tree:             BTreeMap::new(),
            indexed:          HashSet::new(),
            buffer_grams:     HashSet::new(),
            buffer_count_map: HashMap::with_capacity(DEFAULT_CAPACITY_COUNT),
            buffer_count_vec: Vec::with_capacity(DEFAULT_CAPACITY_COUNT),
        }
    }

    pub fn matches(&self, record: &Record) -> bool {
        self.indexed.contains(&record.id)
    }

    pub fn add(&mut self, record: &Record) {
        self.buffer_grams.clear();
        for word in &record.title.words {
            let chars = word.view(&record.title.chars.as_ref());
            for gram in Trigrams::new(chars) {
                self.buffer_grams.insert(gram);
            }
        }
        for &gram in self.buffer_grams.iter() {
            self.tree
                .entry(gram)
                .and_modify(|ids| {
                    ids.insert(record.id);
                })
                .or_insert_with(|| {
                    let mut ids = HashSet::with_capacity(DEFAULT_CAPACITY_TREE);
                    ids.insert(record.id);
                    ids
                });
        }
    }

    pub fn prepare<T: AsRef<[char]>>(
        &mut self,
        query:   &Text<T>,
        size:    usize,
    ) {
        if query.words.len() == 0 {
            return;
        }

        let grams     = &mut self.buffer_grams;
        let count_vec = &mut self.buffer_count_vec;
        let count_map = &mut self.buffer_count_map;

        count_map.clear();

        grams.clear();
        for word in &query.words {
            let chars = word.view(query.chars.as_ref());
            for gram in Trigrams::new(chars) {
                grams.insert(gram);
            }
        }

        let update_count_vec = |map: &HashMap<usize, usize>, vec: &mut Vec<(usize, usize)>| {
            vec.clear();
            vec.extend(map.iter().map(|(&id, &count)| (id, count)));
            vec.sort_by(|(_, count1), (_, count2)| count2.cmp(count1));
        };

        for (i, gram) in grams.iter().enumerate() {
            if let Some(ids) = self.tree.get(gram) {
                for &id in ids {
                    count_map
                        .entry(id)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }
            if i > 0 && count_map.len() > size * 3 {
                update_count_vec(count_map, count_vec);
                while count_vec.len() > size * 2 {
                    let (id, _) = count_vec.pop().unwrap();
                    count_map.remove(&id);
                }
            }
        }

        update_count_vec(count_map, count_vec);

        self.indexed.clear();
        for (id, _) in count_vec.iter().take(size) {
            self.indexed.insert(*id);
        }
    }
}


#[cfg(test)]
mod tests {
    use insta::{assert_debug_snapshot, assert_snapshot};
    use crate::tokenization::tokenize_query;
    use super::Record;
    use super::TrigramIndex;

    fn export_tree(index: &TrigramIndex) -> String {
        let mut result = String::new();
        for (gram, ids) in index.tree.iter() {
            let gram    = gram.iter().cloned().collect::<String>();
            let mut ids = ids.iter().cloned().collect::<Vec<_>>();
            ids.sort();
            result.push_str(&format!("\"{:3}\" {:?}\n", gram, ids));
        }
        result
    }

    fn get_index() -> (TrigramIndex, [Record; 5]) {
        let records = [
            Record::new(10, "brown plush bear",     10, &None),
            Record::new(20, "the metal detector",   20, &None),
            Record::new(30, "yellow metal mailbox", 30, &None),
            Record::new(40, "thesaurus",            40, &None),
            Record::new(50, "wi-fi router",         50, &None),
        ];
        let mut index = TrigramIndex::new();
        for record in &records {
            index.add(&record);
        }
        (index, records)
    }

    fn check_prepare(name: &str, size: usize, queries: &[&str]) {
        let (mut index, _) = get_index();
        for (i, query) in queries.iter().enumerate() {
            let query = tokenize_query(query, &None);
            index.prepare(&query, size);
            let mut sorted = index.indexed.iter().collect::<Vec<_>>();
            sorted.sort();
            assert_debug_snapshot!(format!("{}-{}", name, i), sorted);
        }
    }

    #[test]
    fn add_first() {
        let mut index = TrigramIndex::new();
        index.add(&Record::new(10, "Foo Bar", 10, &None));
        assert_snapshot!(export_tree(&index));
    }

    #[test]
    fn add_second() {
        let mut index = TrigramIndex::new();
        index.add(&Record::new(10, "Foo Bar", 10, &None));
        index.add(&Record::new(20, "Bar Baz", 20, &None));
        assert_snapshot!(export_tree(&index));
    }

    #[test]
    fn prepare_mismatch() {
        check_prepare("mismatch", 3, &["zzzap!"]);
    }

    #[test]
    fn prepare_best_match() {
        check_prepare("best_match", 1, &[
            "metal detector",
            "thesaurus",
            "router",
        ]);
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

    #[test]
    fn matches_basic() {
        let (mut index, records) = get_index();
        let query = tokenize_query("metal", &None);
        index.prepare(&query, 10);
        assert_eq!(index.matches(&records[0]), false);
        assert_eq!(index.matches(&records[1]), true);
        assert_eq!(index.matches(&records[2]), true);
        assert_eq!(index.matches(&records[3]), false);
        assert_eq!(index.matches(&records[4]), false);
    }
}
