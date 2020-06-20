use crate::lexis::{Text, WordMatch};
use crate::search::Hit;


pub fn highlight(hit: &Hit, dividers: (&[char], &[char])) -> String {
    let (div_left, div_right) = dividers;
    let Hit {
        title: Text { words, source, .. },
        matches,
        ..
    } = hit;

    let mut highlighted = {
        let chars_src = source.len();
        let chars_hl  = (div_left.len() + div_right.len() + 1) * words.len();
        String::with_capacity((chars_src + chars_hl) * 4)
    };

    let mut offset = 0;
    for (ix, word) in words.iter().enumerate() {
        match matches.iter().find(|m| m.record.ix == ix) {
            Some(WordMatch { record: m, .. }) => {
                let match_start = word.place.0 + m.slice.0;
                let match_end   = word.place.0 + m.slice.1;
                highlighted.extend(&source[offset .. match_start]);
                highlighted.extend(div_left);
                highlighted.extend(&source[match_start .. match_end]);
                highlighted.extend(div_right);
                highlighted.extend(&source[match_end .. word.place.1]);
            },
            None => {
                highlighted.extend(&source[offset .. word.place.1]);
            },
        }
        offset = word.place.1;
    }
    highlighted.extend(&source[offset .. ]);

    highlighted
}


#[cfg(test)]
mod tests {
    use crate::lexis::{WordMatch, MatchSide};
    use crate::store::Record;
    use crate::search::Hit;
    use super::highlight;

    #[test]
    fn test_highlight() {
        let record = Record::new(10, "metal detector", 0, &None);

        let mut hit = Hit::from_record(&record);
        hit.matches.push(WordMatch {
            query:  MatchSide { ix: 0, len: 0, slice: (0, 0), primary: true },
            record: MatchSide { ix: 1, len: 6, slice: (0, 6), primary: true },
            typos:  0,
            fin:    false,
        });

        let div_left:  Vec<char> = "[".chars().collect();
        let div_right: Vec<char> = "]".chars().collect();

        let expected = "metal [detect]or";
        let received: String = highlight(&hit, (&div_left, &div_right));

        assert_eq!(&received, expected);
    }

    #[test]
    fn test_highlight_stripped() {
        let record = Record::new(10, "'metal' mailbox!", 0, &None);

        let mut hit = Hit::from_record(&record);
        hit.matches.push(WordMatch {
            query:  MatchSide { ix: 0, len: 0, slice: (0, 0), primary: true },
            record: MatchSide { ix: 0, len: 5, slice: (0, 5), primary: true },
            typos:  0,
            fin:    false,
        });

        let left:  Vec<char> = "{{".chars().collect();
        let right: Vec<char> = "}}".chars().collect();

        let expected = "'{{metal}}' mailbox!";
        let received: String = highlight(&hit, (&left, &right));

        assert_eq!(&received, expected);
    }
}
