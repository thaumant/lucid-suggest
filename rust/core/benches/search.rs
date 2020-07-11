use criterion::{criterion_group, criterion_main, Criterion, black_box};
use std::fs;
use std::cmp::min;
use rand::prelude::*;
use rand::distributions::WeightedIndex;
use lucid_suggest_core::{
    Word,
    Store,
    Record,
    TextOwn,
    tokenize_query,
    tokenize_record,
};
use lucid_suggest_core::lang::{
    Lang,
    lang_english,
};


fn search_benchmark(criterion: &mut Criterion) {
    let dataset = SyntheticDataset::new();

    for &(min_words, max_words) in &[(2, 4), (4, 8)] {
        for &n_records in &[100, 1000, 10_000] {
            let bench_name = format!("search {} records, {}-{} words", n_records, min_words, max_words);
            criterion.bench_function(&bench_name, |bench| {
                let (store, queries) = dataset.gen_data(n_records, min_words, max_words);
                let mut i = 0;
                bench.iter(|| {
                    let query = &queries[i].to_ref();
                    store.search(black_box(query));
                    i = (i + 1) % queries.len();
                })
            });
        }
    }
}

criterion_group!(benches, search_benchmark);
criterion_main!(benches);


struct SyntheticDataset {
    pub lang:  Lang,
    pub chars: Vec<char>,
    pub words: Vec<String>,
    pub dist:  WeightedIndex<usize>,
}


impl SyntheticDataset {
    pub fn new() -> Self {
        let chars = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<_>>();

        let text  = fs::read_to_string("../../datasets/top_1000_words_en.csv").unwrap();
        let words = text
            .split("\n")
            .map(|w| w.trim())
            .filter(|w| w.len() > 0)
            .map(|w| w.to_string())
            .collect::<Vec<_>>();

        let weights = (0..words.len())
            .map(|i| (words.len() - i) / 100 + 1) // linear from 10x to 1x
            .collect::<Vec<_>>();
        let dist = WeightedIndex::new(weights).unwrap();

        let lang = lang_english();

        Self { chars, words, dist, lang }
    }

    pub fn gen_data(&self, len: usize, min_words: usize, max_words: usize) -> (Store, Vec<TextOwn>) {
        let mut records = self.gen_records(len, min_words, max_words);
        let mut store   = Store::new();
        let queries     = self.gen_queries(&records, 10000);
        for record in records.drain(..) {
            store.add(record);
        }
        (store, queries)
    }

    pub fn gen_records(&self, len: usize, min_words: usize, max_words: usize) -> Vec<Record> {
        let mut id      = 1;
        let mut records = Vec::with_capacity(len);
        let lang        = lang_english();
        for _ in 0..len {
            let title  = self.gen_title(min_words, max_words);
            let record = Record { ix: 0, id, title: tokenize_record(&title, &lang), rating: 0 };
            records.push(record);
            id += 1;
        }
        records
    }

    pub fn gen_queries(&self, records: &[Record], len: usize) -> Vec<TextOwn> {
        let mut queries = Vec::with_capacity(len);
        let mut rng     = thread_rng();
        for _ in 0..len {
            let record   = &records[rng.gen_range(0, records.len())];
            let title    = &record.title;
            let qlen_max = title.words[..min(3, title.words.len())]
                .iter()
                .map(|w| w.len())
                .sum::<usize>();
            let qlen     = rng.gen_range(0, qlen_max);
            let query    = self.corrupt(&title.source);
            let query    = &query[0 .. min(qlen, query.len())];
            let query    = query.iter().collect::<String>();
            queries.push(tokenize_query(&query, &self.lang));
        }
        queries
    }

    pub fn gen_title(&self, min_words: usize, max_words: usize) -> String {
        let mut rng   = thread_rng();
        let n_words   = rng.gen_range(min_words, max_words);
        let mut title = String::with_capacity(n_words * 10);
        for _ in 0..n_words {
            title.push_str(self.random_word());
            if rng.gen::<f64>() < 0.5 { title.push(','); }
            title.push(' ');
        }
        title.pop();
        if let Some(',') = title.chars().last() {
            title.pop();
        }
        title
    }

    pub fn corrupt(&self, source: &[char]) -> Vec<char> {
        let mut chars = source.to_vec();
        let mut rng = thread_rng();
        let n_typos = rng.gen_range(0, chars.len() / 10 + 1);
        for _ in 0..n_typos {
            if chars.len() <= 1 {
                break;
            }
            let mut ix = 0;
            while ix == 0 || chars[ix - 1] == ' ' {
                ix = rng.gen_range(0, chars.len());
            }
            let ch = self.chars[rng.gen_range(0, self.chars.len())];
            let choice = rng.gen_range(0, 3);
            if choice == 0 { chars[ix] = ch; }
            if choice == 1 { chars.remove(ix); }
            if choice == 2 { chars.insert(ix + 1, ch); }
        }
        chars
    }

    pub fn random_word(&self) -> &str {
        let mut rng = thread_rng();
        &self.words[self.dist.sample(&mut rng)]
    }
}
