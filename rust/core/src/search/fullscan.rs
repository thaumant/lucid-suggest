use crate::store::Record;
use crate::search::Hit;


pub struct FullScan<'a, Src: Iterator<Item=&'a Record>> {
    source: Src,
}


impl<'a, Src: Iterator<Item=&'a Record>> FullScan<'a, Src> {
    pub fn new(source: Src) -> Self {
        Self { source }
    }
}


impl<'a, Src: Iterator<Item=&'a Record>> Iterator for FullScan<'a, Src> {
    type Item = Hit<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.source
            .next()
            .map(Hit::from_record)
    }
}
