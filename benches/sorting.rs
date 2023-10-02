use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rand::distributions::Standard;
use rand::{thread_rng, Rng};
use rustroperiods::periodograms::string_length;
use rustroperiods::sorting::find_peaks;
use rustroperiods::LightCurve;

fn create_random_data(n: usize) -> Vec<f64> {
    let data: Vec<f64> = thread_rng().sample_iter(&Standard).take(n).collect();
    data
}

pub fn string_length_benchmark(c: &mut Criterion) {
    let lc = LightCurve::random(1000);
    c.bench_function("string length", |bencher| {
        bencher.iter(|| string_length(&lc, 1e-3, 3.0, 1e-4))
    });
}

pub fn peaks_benchmark(c: &mut Criterion) {
    let input = create_random_data(10000);
    c.bench_function("find peaks", move |bencher| {
        bencher.iter_batched(
            || input.clone(),
            |data| find_peaks(&data, 10),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, string_length_benchmark, peaks_benchmark);
criterion_main!(benches);
