use criterion::*;
use rustroperiods::lightcurve::LightCurve;
use rustroperiods::periodograms::sweep_frequency_grid;

pub fn periodogram_benchmark(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    let mut group = c.benchmark_group("periodograms");
    group.plot_config(plot_config);
    group.sample_size(10);
    group.sampling_mode(SamplingMode::Flat);
    for n in [10, 20, 50, 100, 200, 500, 1000].iter() {
        let lc = LightCurve::noisy_sinewave(*n as usize, 100.0, 10.0, 0.2);
        group.bench_with_input(BenchmarkId::from_parameter(n), n, |b, &n| {
            b.iter(|| sweep_frequency_grid(black_box(&lc), 1e-3, 3.0, 1e-4))
        });
    }
    group.finish();
    // group.bench_function("string length", |bencher| {
    //     bencher.iter(|| string_length(black_box(&lc), 1e-3, 3.0, 1e-4))
    // });
}

criterion_group!(benches, periodogram_benchmark);
criterion_main!(benches);
