use crate::tokenization::Text;
use crate::matching::WordMatch;
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
    highlighted.retain(|ch| ch != '\0');

    highlighted
}


#[cfg(test)]
mod tests {
    use crate::matching::{WordMatch, MatchSide};
    use crate::store::Record;
    use crate::search::Hit;
    use crate::lang::{lang_german, lang_portuguese};
    use super::highlight;

    const L: &[char] = &['['];
    const R: &[char] = &[']'];

    fn mock_match(ix: usize, size: usize) -> WordMatch {
        WordMatch {
            query:  MatchSide { ix:  0, len: size, slice: (0, size), function: false },
            record: MatchSide { ix: ix, len: size, slice: (0, size), function: false },
            typos:  0,
            fin:    false,
        }
    }

    #[test]
    fn highlight_basic() {
        let record = Record::new(10, "metal detector", 0, &None);

        let mut hit = Hit::from_record(&record);
        hit.matches.push(mock_match(1, 6));

        let expected = "metal [detect]or";
        let received = highlight(&hit, (L, R));

        assert_eq!(&received, expected);
    }

    #[test]
    fn highlight_stripped() {
        let record = Record::new(10, "'metal' mailbox!", 0, &None);

        let mut hit = Hit::from_record(&record);
        hit.matches.push(mock_match(0, 5));

        let expected = "'[metal]' mailbox!";
        let received = highlight(&hit, (L, R));

        assert_eq!(&received, expected);
    }

    #[test]
    fn highlight_multichar_dividers() {
        let record = Record::new(10, "metal detector", 0, &None);

        let mut hit = Hit::from_record(&record);
        hit.matches.push(mock_match(1, 6));

        let l: &[char] = &['{', '{'];
        let r: &[char] = &['}', '}'];

        let expected = "metal {{detect}}or";
        let received = highlight(&hit, (l, r));

        assert_eq!(&received, expected);
    }

    #[test]
    fn highlight_utf_padded() {
        let record = Record::new(10, "Passstraße", 0, &Some(lang_german()));

        let mut hit = Hit::from_record(&record);
        hit.matches.push(mock_match(0, 9));

        let expected = "[Passstraß]e";
        let received = highlight(&hit, (L, R));

        assert_eq!(&received, expected);
    }

    #[test]
    fn highlight_utf_nfd() {
        let record = Record::new(10, "Passstraße", 0, &Some(lang_portuguese()));

        let mut hit = Hit::from_record(&record);
        hit.matches.push(mock_match(0, 9));

        let expected = "[Passstraß]e";
        let received = highlight(&hit, (L, R));

        assert_eq!(&received, expected);
    }
}
