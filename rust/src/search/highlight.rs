use crate::lexis::{Text, WordMatch};
use super::{Hit, Scores};

pub fn highlight(hits: &[Hit], left: &[char], right: &[char]) -> Vec<Vec<char>> {
    hits.iter()
        .map(|hit| highlight_one(hit, left, right))
        .collect()
}

fn highlight_one(hit: &Hit, left: &[char], right: &[char]) -> Vec<char> {
    let Hit { 
        scores: Scores { matches, .. }, 
        text: Text { words, source }, 
        ..
    } = hit;

    let capacity   = source.len() + words.len() * (left.len() + right.len() + 1);
    let mut result = Vec::with_capacity(capacity);

    for (i, w) in words.iter().enumerate() {
        match matches.iter().find(|m| m.record.pos == i) {
            Some(WordMatch { record: m, .. }) => {
                let match_start = w.slice.0 + m.slice.0;
                let match_end   = w.slice.0 + m.slice.1;
                result.extend(&w.source[.. match_start]);
                result.extend(left);
                result.extend(&w.source[match_start .. match_end]);
                result.extend(right);
                result.extend(&w.source[match_end .. ]);
            },
            None => {
                result.extend(w.source);
            },
        }
        result.push(' ');
    }
    result.pop();

    result
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use crate::lexis::{Text, Chars};
    use super::super::{Hit, search};
    use super::highlight;

    fn chars(s: &str) -> Vec<char> {
        s.chars().collect()
    }

    fn record(chars: &[char]) -> Text {
        Text::new(chars)
            .split(&Chars::Whitespaces)
            .strip(&Chars::NotAlphaNum)
            .lower()
    }

    fn query(s: &[char]) -> Text {
        record(s).fin(false)
    }

    #[test]
    fn test_highlight() {
        let cr1 = chars("brown plush bear");
        let cr2 = chars("Metal detector");
        let cr3 = chars("yellow Metal Mailbox");

        let mut hits = vec![
            Hit::new(10, record(&cr1)),
            Hit::new(20, record(&cr2)),
            Hit::new(30, record(&cr3)),
        ];

        let cq = chars("metall mail");
        let q  = query(&cq);

        search(&q, &mut hits);

        let left:  Vec<char> = "[".chars().collect();
        let right: Vec<char> = "]".chars().collect();
        let highlighted: Vec<String> = highlight(&hits, &left, &right)
            .iter()
            .map(|h| h.iter().collect())
            .collect();

        assert_debug_snapshot!(highlighted);
    }

    #[test]
    fn test_highlight_stripped() {
        let cr1 = chars("brown plush bear");
        let cr2 = chars("Metal detector");
        let cr3 = chars("yellow 'Metal' -Mailbox-");

        let mut hits = vec![
            Hit::new(10, record(&cr1)),
            Hit::new(20, record(&cr2)),
            Hit::new(30, record(&cr3)),
        ];

        let cq = chars("metall mail");
        let q  = query(&cq);

        search(&q, &mut hits);

        let left:  Vec<char> = "{{".chars().collect();
        let right: Vec<char> = "}}".chars().collect();
        let highlighted: Vec<String> = highlight(&hits, &left, &right)
            .iter()
            .map(|h| h.iter().collect())
            .collect();

        assert_debug_snapshot!(highlighted);
    }
}