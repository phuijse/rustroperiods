use crate::lightcurve::LightCurve;
//use crate::sorting::argsort;
use crate::stats::variance;
//use std::time::Instant;

/// Performs the epoch folding transformation
/// phase = modulo(time, period)/period
/// on each element of `times`
//#[inline(always)]
fn fold(times: &[f64], period: f64) -> Vec<f64> {
    times
        .into_iter()
        .map(|time| (time % period) / period)
        .collect()
}

fn argsort2(data: &[f64]) -> Vec<usize> {
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    indices.sort_unstable_by(|&i, &j| data[i].total_cmp(&data[j]));
    indices
}

pub fn string_length(lc: &LightCurve, fmin: f64, fmax: f64, fstep: f64) -> Vec<f64> {
    let n_samples = lc.mag.len() as f64;
    let mag_variance = variance(&lc.mag);
    let nsteps = ((fmax - fmin) / fstep) as i32;
    let denominator = 2.0 * n_samples * mag_variance;
    let mut periodogram: Vec<f64> = Vec::with_capacity(nsteps as usize);
    //let mut times: Vec<f64> = Vec::with_capacity(nsteps as usize);
    for k in 0..nsteps {
        let trial_frequency = f64::from(k).mul_add(fstep, fmin);
        let phase = fold(&lc.mjd, trial_frequency.powi(-1));
        //let start = Instant::now();
        let folded_indices: Vec<usize> = argsort2(&phase);
        //let end = Instant::now();
        let folded_magnitude = folded_indices
            .into_iter()
            .map(|i| lc.mag[i])
            .collect::<Vec<f64>>();
        //times.push((end - start).as_secs_f64());

        //let mut string_length: f64 = 0.0;
        //for i in 1..folded_magnitude.len() {
        //    string_length += (folded_magnitude[i] - folded_magnitude[i - 1]).powi(2);
        //}
        let string_length = folded_magnitude
            .windows(2)
            .map(|w| (w[0] - w[1]).powi(2))
            .sum::<f64>();
        periodogram.push(string_length / denominator);
    }
    //println!("{}", times.iter().sum::<f64>());
    periodogram
}
