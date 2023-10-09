//! This module contains implementations of periodograms for irregularly-sampled time series

use crate::lightcurve::LightCurve;
use crate::sorting::argsort;
use crate::stats;
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

/// Computes the String Length Lafler-Kinman (SLLK) statistic of a `lc` over a linearly-spaced
/// range of frequencies starting in `fmin`, ending in `fmax` and separated by `fstep`.
///
/// The SLLK statistic corresponde to Eq. 9 in David Clarke, "String/Rope length methods using the
/// Lafler-Kinman statistic", Astronomy & Astrophysics, vol. 386, n. 2, pp. 763-774, 2002.
/// Reference: <https://ui.adsabs.harvard.edu/abs/2002A%26A...386..763C/abstract>
///
pub fn string_length(lc: &LightCurve, fmin: f64, fmax: f64, fstep: f64) -> Vec<f64> {
    let nsteps = ((fmax - fmin) / fstep) as i32;
    let statistic = LaflerKinmanStringLength::new(lc);
    let mut periodogram: Vec<f64> = Vec::with_capacity(nsteps as usize);
    //let mut times: Vec<f64> = Vec::with_capacity(nsteps as usize);
    for k in 0..nsteps {
        let trial_frequency = f64::from(k).mul_add(fstep, fmin);
        let phase = fold(&lc.mjd, trial_frequency.powi(-1));
        //let start = Instant::now();
        let folded_indices: Vec<usize> = argsort(&phase);
        //let end = Instant::now();
        let folded_magnitude = folded_indices
            .into_iter()
            .map(|i| lc.mag[i])
            .collect::<Vec<f64>>();
        //times.push((end - start).as_secs_f64());

        periodogram.push(statistic.compute(&folded_magnitude));
    }
    //println!("{}", times.iter().sum::<f64>());
    periodogram
}

struct LaflerKinmanStringLength {
    normalizing_constant: f64,
}

impl LaflerKinmanStringLength {
    fn new(lc: &LightCurve) -> Self {
        let n_samples = lc.mag.len() as f64;
        let mag_variance = stats::sample_variance(&lc.mag, true);
        Self {
            normalizing_constant: 2.0 * n_samples * mag_variance,
        }
    }
    #[inline(always)]
    fn compute(&self, folded_magnitude: &[f64]) -> f64 {
        folded_magnitude
            .windows(2)
            .map(|w| (w[0] - w[1]).powi(2))
            .sum::<f64>()
            / self.normalizing_constant
    }
}
