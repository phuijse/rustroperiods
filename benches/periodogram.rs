use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rustroperiods::lightcurve::LightCurve;
use rustroperiods::periodograms::string_length;

pub fn periodogram_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("periodograms");
    group.sample_size(10);
    let n = [10, 20, 50, 100, 200, 500, 1000];
    let lc = LightCurve::random(n[3]);
    group.bench_function("string length", |bencher| {
        bencher.iter(|| string_length(&lc, 1e-3, 3.0, 1e-4))
    });
}

criterion_group!(benches, periodogram_benchmark);
criterion_main!(benches);
