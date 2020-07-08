use crate::tokenization::TextRef;
use crate::search::Hit;

pub fn hit_matches(query: &TextRef, hit: &Hit) -> bool {
    if query.is_empty() { return true; }
    if hit.rmatches.len() == 0 { return false; }
    if hit.rmatches.len() == 1 && hit.qmatches.len() == 1 && query.words.len() > 1 {
        let rmatch     = &hit.rmatches[0];
        let qmatch     = &hit.qmatches[0];
        let first_half = (qmatch.word_len() * 2) < rmatch.word_len();
        if !rmatch.fin && first_half { return false; }
    }
    true
}
