use crate::lexis::tokenize_query;
use super::data::{Record, SearchResult};
use super::engine::Engine;
use super::highlight::highlight;


static DEFAULT_LIMIT: usize = 10;


pub struct Store {
    records:    Vec<Record>,
    separators: (Vec<char>, Vec<char>),
    limit:      usize,
    results:    Vec<SearchResult>,
}


impl Store {
    pub fn new() -> Self {
        Self {
            records:    Vec::new(),
            separators: (vec!['['], vec![']']),
            limit:      DEFAULT_LIMIT,
            results:    Vec::with_capacity(DEFAULT_LIMIT),
        }
    }

    pub fn add(&mut self, record: Record) {
        self.records.push(record);
    }

    pub fn highlight_using(&mut self, separators: (&str, &str)) {
        let left:  Vec<char> = separators.0.chars().collect();
        let right: Vec<char> = separators.1.chars().collect();
        self.separators = (left, right);
    }

    pub fn results<'a>(&'a self) -> &'a [SearchResult] {
        &self.results
    }

    pub fn search(&mut self, query: &str) {
        let query: Vec<char> = query.chars().collect();
        let query = tokenize_query(&query);

        let mut engine = Engine::new(&query, self.limit);
        engine.push_many(&self.records);
        engine.sort_and_truncate();

        let separators = (&self.separators.0[..], &self.separators.1[..]);
        let results = &mut self.results;
        results.clear();
        results.extend(
            engine.hits()
                .iter()
                .map(|hit| {
                    SearchResult {
                        id: hit.id,
                        highlighted: highlight(hit, separators)
                    }
                })
        );
    }
}
