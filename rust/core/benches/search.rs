use criterion::{criterion_group, criterion_main, Criterion};
use lucid_suggest_core::{Store, Record, search, lang, tokenization};

fn text_match_benchmark(criterion: &mut Criterion) {
    criterion.bench_function("search", |bench| {
        let mut store = Store::new();
        store.lang = Some(lang::lang_english());
        store.add(Record::new(10, "AA 1.5 Alkaline Batteries — Pack of 12", 10, &store.lang));
        store.add(Record::new(20, "Lightning to USB A Cable",               20, &store.lang));
        store.add(Record::new(30, "Electric Toothbrush",                    30, &store.lang));
        store.add(Record::new(40, "Vacuum Compression Storage Bags",        40, &store.lang));

        let query = tokenization::tokenize_query("compressing bag for clothes", &store.lang);
        let query = query.to_ref();

        bench.iter(|| {
            search(&store, &query).collect::<Vec<_>>()
        })
    });
}

criterion_group!(benches, text_match_benchmark);
criterion_main!(benches);