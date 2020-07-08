use std::default::Default;
use crate::tokenization::TextRef;
use crate::matching::WordMatch;
use crate::store::Record;
use super::score::Scores;


#[derive(Debug)]
pub struct Hit<'a> {
    pub id:      usize,
    pub title:   TextRef<'a>,
    pub rating:  usize,
    pub rmatches: Vec<WordMatch>,
    pub qmatches: Vec<WordMatch>,
    pub scores:  Scores,
}


impl<'a> Hit<'a> {
    pub fn from_record(record: &'a Record) -> Hit<'a> {
        Hit {
            id:       record.id,
            title:    record.title.to_ref(),
            rating:   record.rating,
            scores:   Default::default(),
            rmatches: Vec::new(),
            qmatches: Vec::new(),
        }
    }
}
