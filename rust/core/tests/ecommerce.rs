#![allow(non_snake_case)]

use std::cell::RefCell;
use std::fs;
use serde_json::Value;
use regex::Regex;
use lucid_suggest_core::{Store, Record, tokenize_query, lang_english, SearchResult};


thread_local! {
    static STORE: RefCell<Option<Store>> = RefCell::new(None);
}


fn init_store() {
    STORE.with(|cell| {
        if cell.borrow().is_none() {
            let mut store = Store::new();
            store.lang = lang_english();
            store.highlight_with(("<", ">"));

            let text   = fs::read_to_string("../../datasets/e_commerce.json").unwrap();
            let parsed = serde_json::from_str::<Value>(&text).unwrap();
            for val in parsed.as_array().unwrap() {
                store.add(Record::new(
                    val["id"].as_u64().unwrap() as usize,
                    val["title"].as_str().unwrap(),
                    val["rating"].as_u64().unwrap() as usize,
                    &store.lang,
                ))
            }

            cell.replace(Some(store));
        }
    });
}


fn using_store<T, F>(f: F) -> T where F: (FnOnce(&mut Store) -> T) {
    init_store();
    STORE.with(|cell| {
        let store_opt = &mut *cell.borrow_mut();
        let store = store_opt.as_mut().unwrap();
        f(store)
    })
}


fn assert_hit_match(hit: &SearchResult, pattern: &str) {
    let title_lower = hit.title.to_lowercase();
    assert!(
        Regex::new(pattern).unwrap().is_match(&title_lower),
        "Pattern \"{}\" does not match hit {} \"{}\"",
        &pattern,
        hit.id,
        &hit.title
    );
}


fn assert_all_match(hits: &[SearchResult], pattern: &str) {
    for hit in hits {
        assert_hit_match(hit, pattern);
    }
}


fn assert_any_match(hits: &[SearchResult], pattern: &str) -> usize {
    for (i, hit) in hits.iter().enumerate() {
        if Regex::new(pattern).unwrap().is_match(&hit.title) {
            return i;
        }
    }
    panic!("No hit matches pattern \"{}\"", pattern);
}


#[test]
fn ecommerce_joined_match() {
    using_store(|store| {
        let query = tokenize_query("night light", &store.lang);
        let hits  = store.search(&query.to_ref());
        assert_hit_match(&hits[0], r"<night> <light>");
        assert_any_match(&hits,    r"<nightlight>");
    });
}


#[test]
fn ecommerce_unpopular_hit() {
    using_store(|store| {
        let query = tokenize_query("wise", &store.lang);
        let hits  = store.search(&query.to_ref());
        assert_hit_match(&hits[0], r"<wise>");
    });
}


#[test]
fn ecommerce_longest_match() {
    using_store(|store| {
        let query = tokenize_query("i wise", &store.lang);
        let hits  = store.search(&query.to_ref());
        assert_hit_match(&hits[0], r"<wise>");
        assert_any_match(&hits,    r"<i>");
    });
}


#[test]
fn ecommerce_match_surface() {
    using_store(|store| {
        let query = tokenize_query("it it ornament", &store.lang);
        let hits  = store.search(&query.to_ref());
        assert_hit_match(&hits[0], r"<ornament>");
        assert_hit_match(&hits[1], r"<it>.*<it>");
    });
}


/**
 * Proper top1 were missing.
 */
#[test]
fn ecommerce_case__little_bird() {
    using_store(|store| {
        let query = tokenize_query("little bird", &store.lang);
        let hits  = store.search(&query.to_ref());
        assert_hit_match(&hits[0], r"<little> <bird>");
        assert_all_match(&hits, r"<(little|bird)>")
    });
}


/**
 * "50's" did not match "50s".
 */
#[test]
fn ecommerce_case__50s() {
    using_store(|store| {
        let query = tokenize_query("50s", &store.lang);
        let hits  = store.search(&query.to_ref());
        assert_hit_match(&hits[0], r"<50's>");
    });
}


/**
 * Top results were seized by less relevant "<gold>".
 */
#[test]
fn ecommerce_case__hold() {
    using_store(|store| {
        let query = tokenize_query("hold", &store.lang);
        let hits  = store.search(&query.to_ref());
        assert_all_match(&hits, r"<hold");
    });
}


/**
 * Top1 was seized by less relevant "<black>".
 */
#[test]
fn ecommerce_case__backp() {
    using_store(|store| {
        let query = tokenize_query("backp", &store.lang);
        let hits  = store.search(&query.to_ref());
        assert_all_match(&hits[..5], r"<backp>ack");
    });
}


/**
 * Proper records were displaced with less relevant ones, like "<choc>", "<choi>r".
 */
#[test]
fn ecommerce_case__chop() {
    using_store(|store| {
        let query = tokenize_query("chop", &store.lang);
        let hits  = store.search(&query.to_ref());
        assert_all_match(&hits[..5], r"<chop>ping");
    });
}


/**
 * Got top1 result right, missed top2 altogether.
 */
#[test]
fn ecommerce_case__pop_corn() {
    using_store(|store| {
        let query = tokenize_query("pop corn", &store.lang);
        let hits  = store.search(&query.to_ref());
        assert_all_match(&hits[..2], r"<popcorn>");
    });
}


/**
 * When typed "ornament", on "orn" and "orna" the top was seized with "oranges"
 */
#[test]
fn ecommerce_case__orna() {
    using_store(|store| {
        let query = "ornament";
        for len in 3 .. query.chars().count() {
            let query = query.chars().take(len).collect::<String>();
            let query = tokenize_query(&query, &store.lang);
            let hits  = store.search(&query.to_ref());
            assert_hit_match(&hits[0], r"<orn");
        }
    });
}


/**
 * Got empty results when typed "vn" or "vnt".
 */
#[test]
fn ecommerce_case__vnt() {
    using_store(|store| {
        let query = tokenize_query("vn", &store.lang);
        let hits  = store.search(&query.to_ref());
        assert_hit_match(&hits[0], r"<vin>");

        let query = tokenize_query("vnt", &store.lang);
        let hits  = store.search(&query.to_ref());
        assert_hit_match(&hits[0], r"<vint>age");
    });
}


// // TODO fix
// /**
//  * When typing a joined word, all typos are absorbed by the second part,
//  * so "greenpink" matches "<green> <pend>ant" or "<green> <polk>adot".
//  */
// #[test]
// fn ecommerce_case__greenpink() {
//     using_store(|store| {
//         let query = tokenize_query("greenpink", &store.lang);
//         let hits  = store.search(&query.to_ref());
//         dbg!(&hits);
//         // assert_hit_match(&hits[0], r"<vin>");
//     });
// }


// // TODO fix
// /**
//  * When typing "babule" most proper records are lost
//  * in trigram index due to char transposition.
//  */
// #[test]
// fn ecommerce_case__bauble() {
//     using_store(|store| {
//         let query = tokenize_query("babule", &store.lang);
//         let hits  = store.search(&query.to_ref());
//         dbg!(&hits);
//         // assert_all_match(&hits[..5], r"<bauble>");
//     });
// }
