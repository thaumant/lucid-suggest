use crate::tokenization::{TextOwn, tokenize_record};
use crate::lang::Lang;


#[derive(Debug)]
pub struct Record {
    pub ix:     usize,
    pub id:     usize,
    pub title:  TextOwn,
    pub rating: usize,
}


impl Record {
    pub fn new(id: usize, source: &str, rating: usize, lang: &Lang) -> Record {
        Record {
            ix: 0,
            id,
            title: tokenize_record(source, lang),
            rating,
        }
    }
}
