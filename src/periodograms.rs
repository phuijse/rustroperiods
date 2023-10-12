//! This module contains implementations of periodograms for irregularly-sampled time series

use crate::lightcurve::LightCurve;
use crate::sorting::argsort;
use crate::stats;

/// The periodogram algorithms available
pub enum Periodogram {
    StringLength,
}

/// Computes a given periodogram over a range of linearly-spaced frequencies starting in `fmin`,
/// ending in `fmax` and separated by `fstep`
pub fn single_band_periodogram(
    lc: &LightCurve,
    method: Periodogram,
    fmin: f64,
    fmax: f64,
    fstep: f64,
) -> Vec<f64> {
    let statistic = match method {
        Periodogram::StringLength => lafler_kinman_string_length(&lc),
    };
    let nsteps = ((fmax - fmin) / fstep) as i32;
    (0..nsteps)
        .map(|k| statistic(f64::from(k).mul_add(fstep, fmin)))
        .collect()
}

/// Performs the epoch folding transformation
/// phase = modulo(time, period)/period
/// on each element of `times`
#[inline(always)]
fn fold(times: &[f64], period: f64) -> Vec<f64> {
    times
        .into_iter()
        .map(|time| (time % period) / period)
        .collect()
}

/// The String Length Lafler-Kinman (SLLK) statistic corresponds to Eq. 9 in David Clarke,
/// "String/Rope length methods using the Lafler-Kinman statistic", Astronomy & Astrophysics, vol.
/// 386, n. 2, pp. 763-774, 2002.
///
/// Reference: <https://ui.adsabs.harvard.edu/abs/2002A%26A...386..763C/abstract>
///
fn lafler_kinman_string_length<'a>(lc: &'a LightCurve) -> impl Fn(f64) -> f64 + 'a {
    let n_samples = lc.mag.len() as f64;
    let mag_variance = stats::sample_variance(&lc.mag, true);
    let normalizing_constant = 2.0 * n_samples * mag_variance;
    move |trial_frequency| {
        let phase: Vec<f64> = fold(&lc.mjd, trial_frequency.powi(-1));
        let folded_magnitude = argsort(&phase)
            .into_iter()
            .map(|i| lc.mag[i])
            .collect::<Vec<f64>>();
        folded_magnitude
            .windows(2)
            .map(|w| (w[0] - w[1]).powi(2))
            .sum::<f64>()
            / normalizing_constant
    }
}
