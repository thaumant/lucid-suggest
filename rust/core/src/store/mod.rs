use crate::lexis::{Text, tokenize_record};


static DEFAULT_LIMIT: usize = 10;


#[derive(Debug)]
pub struct Record {
    pub id: usize,
    pub text: Text<Vec<char>>,
    pub prio: usize,
}


impl Record {
    pub fn new(id: usize, source: &str, prio: usize) -> Record {
        Record {
            id,
            text: tokenize_record(source),
            prio,
        }
    }
}


pub struct Store {
    pub records: Vec<Record>,
    pub limit:   usize,
    separators:  (Vec<char>, Vec<char>),
}


impl Store {
    pub fn new() -> Self {
        Self {
            records:    Vec::new(),
            separators: (vec!['['], vec![']']),
            limit:      DEFAULT_LIMIT,
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

    pub fn separators<'a>(&'a self) -> (&'a [char], &'a [char]) {
        (&self.separators.0, &self.separators.1)
    }
}
