use criterion::{criterion_group, criterion_main, Criterion};
use lucid_suggest_core::{Record, lang_english};

fn text_match_benchmark(criterion: &mut Criterion) {
    criterion.bench_function("tokenize", |bench| {
        let lang = lang_english();

        bench.iter(|| {
            Record::new(10, "AA 1.5 Alkaline Batteries — Pack of 12", 10, &lang)
        })
    });
}

criterion_group!(benches, text_match_benchmark);
criterion_main!(benches);