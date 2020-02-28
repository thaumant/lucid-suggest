use std::default::Default;
use super::Scores;
use crate::lexis::{Text, tokenize_record};


#[derive(Debug)]
pub struct Record {
    id: usize,
    text: Text<Vec<char>>,
}


impl Record {
    pub fn new(id: usize, source: &str) -> Record {
        Record {
            id,
            text: tokenize_record(source),
        }
    }
}


#[derive(Debug)]
pub struct Hit<'a> {
    pub id:      usize,
    pub text:    Text<&'a [char]>,
    pub scores:  Scores,
}


impl<'a> Hit<'a> {
    pub fn from_record(record: &'a Record) -> Hit<'a> {
        Hit {
            id:     record.id,
            text:   record.text.to_ref(),
            scores: Default::default(),
        }
    }
}


#[derive(Debug)]
pub struct SearchResult {
    pub id:          usize,
    pub highlighted: Vec<char>,
}
