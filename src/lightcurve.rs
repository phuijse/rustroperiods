use rand::{distributions::Uniform, Rng};
use rand_distr::{Distribution, Normal};

pub struct LightCurve {
    pub mjd: Vec<f64>,
    pub mag: Vec<f64>,
    pub err: Vec<f64>,
}

/// Samples a vector of `n` uniformly distributed floating point values in [`a`, `b`)
fn random_uniform_vector(n: usize, a: f64, b: f64) -> Vec<f64> {
    assert!(a < b);
    let range = Uniform::new(a, b);
    //(0..n).map(|_| rng.sample(&range)).collect::<Vec<f64>>()
    rand::thread_rng()
        .sample_iter(&range)
        .take(n)
        .collect::<Vec<f64>>()
}

impl LightCurve {
    pub fn random(n: usize) -> Self {
        Self {
            mjd: random_uniform_vector(n, 0.01, 1.0),
            mag: random_uniform_vector(n, 0.01, 1.0),
            err: random_uniform_vector(n, 0.01, 1.0),
        }
    }
    pub fn noisy_sinewave(n: usize, length: f64, period: f64, stddev: f64) -> Self {
        let mut mjd = random_uniform_vector(n, 0.0, length);
        mjd.sort_by(|a, b| a.total_cmp(&b));
        let mut rng = rand::thread_rng();
        let normal = Normal::new(0.0, stddev).unwrap();
        let mag: Vec<f64> = mjd
            .iter()
            .map(|t| (2.0 * std::f64::consts::PI * t / period).sin() + normal.sample(&mut rng))
            .collect();
        let err = vec![stddev; n];
        Self { mjd, mag, err }
    }
}
