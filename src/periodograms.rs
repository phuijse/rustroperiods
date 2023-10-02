use crate::lightcurve::LightCurve;
use crate::sorting::{argsort, find_peaks};
use crate::stats::variance;

/// Performs the epoch folding transformation
/// phase = modulo(time, period)/period
/// on each element of `times`
fn fold(times: &[f64], period: f64) -> Vec<f64> {
    times.iter().map(|time| (time % period) / period).collect()
}

pub fn string_length(lc: &LightCurve, fmin: f64, fmax: f64, fstep: f64) {
    let n_samples = lc.mag.len() as f64;
    let mag_variance = variance(&lc.mag);
    let nsteps = ((fmax - fmin) / fstep) as i32;
    for k in 0..nsteps {
        let trial_frequency = f64::from(k).mul_add(fstep, fmin);
        let phase = fold(&lc.mjd, trial_frequency.powi(-1));
        let folded_indices: Vec<usize> = argsort(&phase);

        let mut string_length: f64 = 0.0;
        for i in 1..lc.mag.len() {
            string_length += (lc.mag[folded_indices[i]] - lc.mag[folded_indices[i - 1]]).powi(2);
        }
        string_length /= 2.0 * n_samples * mag_variance;
        // if printit {
        //     println!("{trial_frequency},{string_length}");
        // }
    }
}
