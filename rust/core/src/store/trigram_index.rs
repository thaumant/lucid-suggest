use fnv::{FnvHashMap as HashMap, FnvHashSet as HashSet};
use crate::tokenization::TextRef;
use super::Record;
use super::trigrams::Trigrams;


const DEFAULT_CAPACITY_IDS:   usize = 10;
const DEFAULT_CAPACITY_COUNT: usize = 100;


pub struct TrigramIndex {
    ids_by_gram:      HashMap<[char; 3], HashSet<usize>>,
    grams_by_id:      HashMap<usize, HashSet<[char; 3]>>,
    ids_indexed:      HashSet<usize>,
    buffer_grams:     HashSet<[char; 3]>,
    buffer_count_map: HashMap<usize, usize>,
    buffer_count_vec: Vec<(usize, usize)>,
}


impl TrigramIndex {
    pub fn new() -> Self {
        Self {
            ids_by_gram:      HashMap::default(),
            grams_by_id:      HashMap::default(),
            ids_indexed:      HashSet::default(),
            buffer_grams:     HashSet::default(),
            buffer_count_map: HashMap::with_capacity_and_hasher(DEFAULT_CAPACITY_COUNT, Default::default()),
            buffer_count_vec: Vec::with_capacity(DEFAULT_CAPACITY_COUNT),
        }
    }

    pub fn matches(&self, record: &Record) -> bool {
        self.ids_indexed.contains(&record.id)
    }

    pub fn add(&mut self, record: &Record) {
        let Self { ids_by_gram, grams_by_id, .. } = self;
        let Record { id, title, .. } = record;

        if grams_by_id.contains_key(id) {
            panic!("Duplicate id {}", id);
        }

        let mut grams = HashSet::default();
        Self::collect_grams(&title.to_ref(), &mut grams);

        for &gram in grams.iter() {
            ids_by_gram
                .entry(gram)
                .and_modify(|ids| {
                    ids.insert(*id);
                })
                .or_insert_with(|| {
                    let mut ids = HashSet::with_capacity_and_hasher(DEFAULT_CAPACITY_IDS, Default::default());
                    ids.insert(*id);
                    ids
                });
        }

        grams_by_id.insert(*id, grams.clone());
    }

    pub fn prepare(
        &mut self,
        query:   &TextRef,
        size:    usize,
    ) {
        if query.words.len() == 0 {
            return;
        }

        let Self {
            ids_by_gram,
            ids_indexed,
            buffer_grams:     grams,
            buffer_count_vec: count_vec,
            buffer_count_map: count_map,
            ..
        } = self;

        count_map.clear();

        Self::collect_grams(&query, grams);

        let sync_count_vec = |map: &HashMap<usize, usize>, vec: &mut Vec<(usize, usize)>| {
            vec.clear();
            vec.extend(map.iter().map(|(&id, &count)| (id, count)));
            vec.sort_by(|(_, count1), (_, count2)| count2.cmp(count1));
        };

        for (i, gram) in grams.iter().enumerate() {
            if let Some(ids) = ids_by_gram.get(gram) {
                for &id in ids {
                    count_map
                        .entry(id)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }
            if i > 0 && count_map.len() > size * 3 {
                sync_count_vec(count_map, count_vec);
                while count_vec.len() > size * 2 {
                    let (id, _) = count_vec.pop().unwrap();
                    count_map.remove(&id);
                }
            }
        }

        sync_count_vec(count_map, count_vec);

        ids_indexed.clear();
        for (id, _) in count_vec.iter().take(size) {
            ids_indexed.insert(*id);
        }
    }

    fn collect_grams(text: &TextRef, grams: &mut HashSet<[char; 3]>) {
        grams.clear();
        let len = text.words.iter().map(|w| w.len()).sum::<usize>();
        if len > grams.capacity() {
            grams.reserve(len - grams.capacity());
        }
        for word in text.words {
            let chars = word.view(text.chars.as_ref());
            for gram in Trigrams::new(chars) {
                grams.insert(gram);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use insta::{assert_debug_snapshot, assert_snapshot};
    use crate::tokenization::tokenize_query;
    use super::Record;
    use super::TrigramIndex;

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

    fn export_tree(index: &TrigramIndex) -> String {
        let mut ids = index.ids_by_gram
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
        let (mut index, _) = get_index();
        for (i, query) in queries.iter().enumerate() {
            let query = tokenize_query(query, &None);
            let query = query.to_ref();
            index.prepare(&query, size);
            let mut sorted = index.ids_indexed.iter().collect::<Vec<_>>();
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
        let query = query.to_ref();
        index.prepare(&query, 10);
        assert_eq!(index.matches(&records[0]), false);
        assert_eq!(index.matches(&records[1]), true);
        assert_eq!(index.matches(&records[2]), true);
        assert_eq!(index.matches(&records[3]), false);
        assert_eq!(index.matches(&records[4]), false);
    }
}
