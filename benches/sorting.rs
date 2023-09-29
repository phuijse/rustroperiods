use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rand::distributions::Standard;
use rand::{thread_rng, Rng};
use rustroperiods::sorting::find_peaks;

fn create_random_data(n: usize) -> Vec<f64> {
    let data: Vec<f64> = thread_rng().sample_iter(&Standard).take(n).collect();
    data
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

criterion_group!(benches, peaks_benchmark);
criterion_main!(benches);
