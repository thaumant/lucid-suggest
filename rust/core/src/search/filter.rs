use crate::tokenization::Text;
use crate::search::Hit;

pub fn hit_matches(query: &Text<&[char]>, hit: &Hit) -> bool {
    if query.is_empty() { return true; }
    if hit.matches.len() == 0 { return false; }
    if hit.matches.len() == 1 && query.words.len() > 1 {
        let word_match = &hit.matches[0];
        let unfinished = !word_match.fin;
        let first_half = (word_match.query.len * 2) < word_match.record.len;
        if unfinished && first_half { return false; }
    }
    true
}
