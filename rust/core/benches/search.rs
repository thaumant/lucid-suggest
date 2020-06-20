use criterion::{criterion_group, criterion_main, Criterion};
use lucid_suggest_core::{Store, Record, search, lang, lexis};

fn text_match_benchmark(criterion: &mut Criterion) {
    criterion.bench_function("search", |bench| {
        let lang = lang::lang_english();
        let mut store = Store::new();
        store.lang = Some(lang);
        store.add(Record::new(10, "AA 1.5 Alkaline Batteries — Pack of 12", 10, &store.lang));
        store.add(Record::new(20, "Lightning to USB A Cable",               20, &store.lang));
        store.add(Record::new(30, "Electric Toothbrush",                    30, &store.lang));
        store.add(Record::new(40, "Vacuum Compression Storage Bags",        40, &store.lang));

        bench.iter(|| {
            let query = lexis::tokenize_query("compressing bag for clothes", &store.lang);
            let query = query.to_ref();
            search(&store, &query).collect::<Vec<_>>()
        })
    });
}

criterion_group!(benches, text_match_benchmark);
criterion_main!(benches);