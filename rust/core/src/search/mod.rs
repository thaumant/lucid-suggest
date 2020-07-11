mod hit;
mod score;
mod result;
mod filter;
mod sort;
mod highlight;

use crate::utils::LimitSort;
use crate::tokenization::TextRef;
use crate::store::Store;
pub use hit::Hit;
pub use result::SearchResult;


impl Store {
    pub fn search<'a>(
        &'a self,
        query: &'a TextRef<'a>,
    ) -> Vec<SearchResult> {
        let dividers = self.dividers();

        let ixs = if query.words.len() > 0 {
            self.index.borrow_mut().prepare(&query, self.limit)
        } else {
            self.top_ixs()
        };

        ixs.iter()
            .map(|&ix| {
                Hit::from_record(&self.records[ix])
            })
            .map(|mut hit| {
                score::score(query, &mut hit);
                hit
            })
            .filter(|hit| {
                filter::hit_matches(query, hit)
            })
            .limit_sort_unstable(self.limit, sort::compare_hits)
            .map(|hit| {
                SearchResult {
                    id:    hit.id,
                    title: highlight::highlight(&hit, dividers),
                }
            })
            .collect()
    }

    fn top_ixs(&self) -> Vec<usize> {
        let top_ixs = &mut *self.top_ixs.borrow_mut();

        if let Some(ixs) = top_ixs {
            return ixs.clone();
        }

        let ixs = self.records
            .iter()
            .limit_sort_unstable(
                self.limit,
                |r1, r2| {
                    r2.rating
                        .cmp(&r1.rating)
                        .then_with(|| r1.title.chars.cmp(&r2.title.chars))
                },
            )
            .map(|r| r.ix)
            .collect::<Vec<_>>();

        *top_ixs = Some(ixs.clone());
        ixs
    }
}


#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use crate::tokenization::{tokenize_query, tokenize_record};
    use crate::lang::{Lang, lang_english, lang_german};
    use crate::store::{Store, Record};

    fn check(name: &str, lang: Lang, queries: &[&str]) {
        let mut store = Store::new();
        store.add(Record { ix: 0, id: 10, title: tokenize_record("brown plush bear",     &lang), rating: 10 });
        store.add(Record { ix: 0, id: 20, title: tokenize_record("the metal detector",   &lang), rating: 20 });
        store.add(Record { ix: 0, id: 30, title: tokenize_record("yellow metal mailbox", &lang), rating: 30 });
        store.add(Record { ix: 0, id: 40, title: tokenize_record("thesaurus",            &lang), rating: 40 });
        store.add(Record { ix: 0, id: 50, title: tokenize_record("wi-fi router",         &lang), rating: 50 });

        for (i, query) in queries.iter().enumerate() {
            let query   = tokenize_query(query, &lang);
            let query   = query.to_ref();
            let results = store.search(&query);
            assert_debug_snapshot!(format!("{}-{}", name, i), results);
        }
    }

    #[test]
    fn search_empty() {
        check("empty", Lang::new(), &[""]);
    }

    #[test]
    fn search_empty_lexicographic() {
        let mut store = Store::new();
        let lang = Lang::new();
        store.add(Record { ix: 0, id: 10, title: tokenize_record("brown plush bear",     &lang), rating: 10 });
        store.add(Record { ix: 0, id: 20, title: tokenize_record("the metal detector",   &lang), rating: 10 });
        store.add(Record { ix: 0, id: 30, title: tokenize_record("yellow metal mailbox", &lang), rating: 10 });
        store.add(Record { ix: 0, id: 40, title: tokenize_record("thesaurus",            &lang), rating: 10 });
        store.add(Record { ix: 0, id: 50, title: tokenize_record("wi-fi router",         &lang), rating: 10 });
        assert_debug_snapshot!(store.top_ixs());
    }

    #[test]
    fn search_equal() {
        check("equal", Lang::new(), &["yelow metall maiblox"]);
    }

    #[test]
    fn search_partial() {
        check("partial", Lang::new(), &[
            "brown plush bear",
            "metal detector",
            "yellow metal mailbox",
        ]);
    }

    #[test]
    fn search_intersection() {
        check("intersection", Lang::new(), &[
            "red wooden mailbox",
            "red wooden mail",
        ]);
    }

    #[test]
    fn search_min_match() {
        check("min_match", Lang::new(), &[
            "wooden mai",
            "wooden mail",
        ]);
    }

    #[test]
    fn search_transpositions() {
        check("transpositions", Lang::new(), &[
            "metal mailbox",
            "mailbox metal",
        ]);
    }


    #[test]
    fn search_stemming() {
        let lang_empty = Lang::new();
        let lang_en    = lang_english();
        let mut store = Store::new();
        store.add(Record { ix: 0, id: 30, title: tokenize_record("universe", &lang_en), rating: 30 });

        let query1   = tokenize_query("university", &lang_empty);
        let query2   = tokenize_query("university", &lang_en);
        let query1   = query1.to_ref();
        let query2   = query2.to_ref();
        let results1 = store.search(&query1);
        let results2 = store.search(&query2);

        assert_debug_snapshot!(results1);
        assert_debug_snapshot!(results2);
    }

    #[test]
    fn search_particles() {
        check("particles_nolang", Lang::new(), &[
            "the",
        ]);

        check("particles", lang_english(), &[
            "the",
        ]);
    }

    #[test]
    fn search_joined() {
        check("joined_query", lang_english(), &[
            "wifi",
        ]);

        check("joined_record", lang_english(), &[
            "the saurus",
        ]);
    }


    #[test]
    fn search_utf_normalization() {
        let mut store = Store::new();
        let lang = lang_german();
        store.add(Record { ix: 0, id: 10, title: tokenize_record("Mitteltöner", &lang), rating: 10 });
        store.add(Record { ix: 0, id: 20, title: tokenize_record("Passstraße",  &lang), rating: 20 });

        let queries = [
            "mitteltö",
            "mitteltö", // ö in nfd!
            "mittelto",
            "passstras",
        ];

        for query in &queries {
            let query  = tokenize_query(query, &lang);
            let query  = query.to_ref();
            let result = store.search(&query);
            assert_debug_snapshot!(result);
        }
    }
}
