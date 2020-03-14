use crate::lexis::{Text, tokenize_record};


static DEFAULT_LIMIT: usize = 10;


#[derive(Debug)]
pub struct Record {
    pub id:     usize,
    pub title:  Text<Vec<char>>,
    pub rating: usize,
}


impl Record {
    pub fn new(id: usize, source: &str, rating: usize) -> Record {
        Record {
            id,
            title: tokenize_record(source),
            rating,
        }
    }
}


pub struct Store {
    pub records: Vec<Record>,
    pub limit:   usize,
    dividers:  (Vec<char>, Vec<char>),
}


impl Store {
    pub fn new() -> Self {
        Self {
            records:  Vec::new(),
            dividers: (vec!['['], vec![']']),
            limit:    DEFAULT_LIMIT,
        }
    }

    pub fn add(&mut self, record: Record) {
        self.records.push(record);
    }

    pub fn clear(&mut self) {
        self.records.clear();
    }

    pub fn highlight_with(&mut self, dividers: (&str, &str)) {
        let left:  Vec<char> = dividers.0.chars().collect();
        let right: Vec<char> = dividers.1.chars().collect();
        self.dividers = (left, right);
    }

    pub fn dividers<'a>(&'a self) -> (&'a [char], &'a [char]) {
        (&self.dividers.0, &self.dividers.1)
    }
}
