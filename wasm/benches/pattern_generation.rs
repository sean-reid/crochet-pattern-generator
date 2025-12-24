use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn pattern_benchmark(c: &mut Criterion) {
    c.bench_function("pattern generation", |b| {
        b.iter(|| {
            black_box(42)
        })
    });
}

criterion_group!(benches, pattern_benchmark);
criterion_main!(benches);
