use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn mesh_benchmark(c: &mut Criterion) {
    c.bench_function("mesh processing", |b| {
        b.iter(|| {
            black_box(42)
        })
    });
}

criterion_group!(benches, mesh_benchmark);
criterion_main!(benches);
