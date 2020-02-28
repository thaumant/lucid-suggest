use crate::lexis::{Text, WordMatch};
use super::{Hit, Scores};


pub fn highlight(hit: &Hit, hl: (&[char], &[char])) -> Vec<char> {
    let (left, right) = hl;
    let Hit {
        scores: Scores { matches, .. },
        text: Text { words, source },
        ..
    } = hit;

    let capacity   = source.as_ref().len() + words.len() * (left.len() + right.len() + 1);
    let mut result = Vec::with_capacity(capacity);

    for (i, w) in words.iter().enumerate() {
        match matches.iter().find(|m| m.record.pos == i) {
            Some(WordMatch { record: m, .. }) => {
                let match_start = w.slice.0 + m.slice.0;
                let match_end   = w.slice.0 + m.slice.1;
                result.extend(&w.source.as_ref()[.. match_start]);
                result.extend(left);
                result.extend(&w.source.as_ref()[match_start .. match_end]);
                result.extend(right);
                result.extend(&w.source.as_ref()[match_end .. ]);
            },
            None => {
                result.extend(w.source.as_ref());
            },
        }
        result.push(' ');
    }
    result.pop();

    result
}


#[cfg(test)]
mod tests {
    use crate::lexis::{WordMatch, MatchSide};
    use super::super::{Record, Hit};
    use super::highlight;

    #[test]
    fn test_highlight() {
        let record = Record::new(10, "metal detector");

        let mut hit = Hit::from_record(&record);
        hit.scores.matches.push(WordMatch {
            query:  MatchSide { pos: 0, len: 0, slice: (0, 0), },
            record: MatchSide { pos: 1, len: 6, slice: (0, 6), },
            typos:  0,
            fin:    false,
        });

        let left:  Vec<char> = "[".chars().collect();
        let right: Vec<char> = "]".chars().collect();

        let expected = "metal [detect]or";
        let received: String = highlight(&hit, (&left, &right)).iter().collect();

        assert_eq!(&received, expected);
    }

    #[test]
    fn test_highlight_stripped() {
        let record = Record::new(10, "'metal' mailbox");

        let mut hit = Hit::from_record(&record);
        hit.scores.matches.push(WordMatch {
            query:  MatchSide { pos: 0, len: 0, slice: (0, 0), },
            record: MatchSide { pos: 0, len: 5, slice: (0, 5), },
            typos:  0,
            fin:    false,
        });

        let left:  Vec<char> = "{{".chars().collect();
        let right: Vec<char> = "}}".chars().collect();

        let expected = "'{{metal}}' mailbox";
        let received: String = highlight(&hit, (&left, &right)).iter().collect();

        assert_eq!(&received, expected);
    }
}
