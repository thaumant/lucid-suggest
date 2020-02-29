use crate::lexis::{Text, WordMatch};
use crate::search::{Hit, SearchResult};


pub struct Highlighter<'a, Src: Iterator<Item=Hit<'a>>> {
    separators: (&'a [char], &'a [char]),
    source: Src,
}


impl<'a, Src: Iterator<Item=Hit<'a>>> Highlighter<'a, Src> {
    pub fn new(source: Src, separators: (&'a [char], &'a [char])) -> Self {
        Self { source, separators }
    }
}


impl<'a, Src: Iterator<Item=Hit<'a>>> Iterator for Highlighter<'a, Src> {
    type Item = SearchResult;

    fn next(&mut self) -> Option<Self::Item> {
        let separators = self.separators;
        self.source.next().map(|hit| {
            SearchResult {
                id: hit.id,
                highlighted: highlight(&hit, separators)
            }
        })
    }
}


fn highlight(hit: &Hit, separators: (&[char], &[char])) -> String {
    let (left, right) = separators;
    let Hit {
        text: Text { words, source },
        matches,
        ..
    } = hit;

    let mut highlighted = {
        let chars_src = source.len();
        let chars_hl  = (left.len() + right.len() + 1) * words.len();
        String::with_capacity((chars_src + chars_hl) * 4)
    };

    for (i, w) in words.iter().enumerate() {
        match matches.iter().find(|m| m.record.pos == i) {
            Some(WordMatch { record: m, .. }) => {
                let match_start = w.slice.0 + m.slice.0;
                let match_end   = w.slice.0 + m.slice.1;

                highlighted.extend(&w.source[.. match_start]);
                highlighted.extend(left);
                highlighted.extend(&w.source[match_start .. match_end]);
                highlighted.extend(right);
                highlighted.extend(&w.source[match_end .. ]);
            },
            None => {
                highlighted.extend(w.source);
            },
        }
        highlighted.push(' ');
    }
    highlighted.pop();

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
        let record = Record::new(10, "metal detector", 0);

        let mut hit = Hit::from_record(&record);
        hit.matches.push(WordMatch {
            query:  MatchSide { pos: 0, len: 0, slice: (0, 0), },
            record: MatchSide { pos: 1, len: 6, slice: (0, 6), },
            typos:  0,
            fin:    false,
        });

        let left:  Vec<char> = "[".chars().collect();
        let right: Vec<char> = "]".chars().collect();

        let expected = "metal [detect]or";
        let received: String = highlight(&hit, (&left, &right));

        assert_eq!(&received, expected);
    }

    #[test]
    fn test_highlight_stripped() {
        let record = Record::new(10, "'metal' mailbox", 0);

        let mut hit = Hit::from_record(&record);
        hit.matches.push(WordMatch {
            query:  MatchSide { pos: 0, len: 0, slice: (0, 0), },
            record: MatchSide { pos: 0, len: 5, slice: (0, 5), },
            typos:  0,
            fin:    false,
        });

        let left:  Vec<char> = "{{".chars().collect();
        let right: Vec<char> = "}}".chars().collect();

        let expected = "'{{metal}}' mailbox";
        let received: String = highlight(&hit, (&left, &right));

        assert_eq!(&received, expected);
    }
}
