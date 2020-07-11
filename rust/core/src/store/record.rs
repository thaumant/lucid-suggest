use crate::tokenization::TextOwn;

#[derive(Debug)]
pub struct Record {
    pub ix:     usize,
    pub id:     usize,
    pub title:  TextOwn,
    pub rating: usize,
}
