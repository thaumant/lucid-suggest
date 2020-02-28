use std::borrow::Cow;
use std::default::Default;
use super::Scores;
use crate::lexis::{Text, tokenize_record};


#[derive(Clone, Debug)]
pub struct Record {
    id: usize,
    text: Text<Vec<char>>,
}


impl Record {
    pub fn new<T: IntoIterator<Item=char>>(id: usize, source: T) -> Record {
        let source: Vec<char> = source.into_iter().collect();
        Record {
            id,
            text: tokenize_record(&source).to_owned(),
        }
    }

    pub fn to_hit<'a>(&'a self) -> Hit<Cow<'a, [char]>> {
        Hit::new(self.id, self.text.to_cow())
    }
}


#[derive(Debug, Clone)]
pub struct Hit<T: AsRef<[char]>> {
    pub id:      usize,
    pub text:    Text<T>,
    pub scores:  Scores,
}


impl<T: AsRef<[char]>> Hit<T> {
    pub fn new(id: usize, text: Text<T>) -> Hit<T> {
        Hit { id, text, scores: Default::default() }
    }
}


#[derive(Debug, Clone)]
pub struct SearchResult {
    pub id:          usize,
    pub highlighted: Vec<char>,
}
