//! The rustroperiods crate contains implementations of different periodograms that have been
//! developed for astronomical time series or light curves. Periodograms are used to search for
//! periodicity in light curves, which makes them a fundamental tool in the analysis of variable
//! stars, such as Cepheids, RR Lyrae and Eclipsing Binaries.
//! Formally speaking, a periodogram is defined as an estimator of the power spectral density (PSD)
//! or spectrum of the signal, i.e. the variance of the signal as a function of frequency, and is
//! typically computed using the Fast Fourier Transform (FFT). In general, light curves are
//! irregularly sampled in time (non-constant time between observations) and because of this, the
//! FFT cannot be used. Fortunately, the astronomical and signal processing communities have
//! proposed several alternatives in the past decades.
//! The periodograms implemented in this crate are not estimators of the PSD, but they do work in a
//! similar way, in the sense that they can be interpreted as the likelihood of a given oscillation
//! frequency being present in the time series. All the periodograms are computed using a brute
//! force strategy, evaluating an user-defined grid of candidate frequencies. Please see the
//! documentation in the module `periodograms` for information on the methods included in this
//! crate.

pub mod lightcurve;
pub mod periodograms;
pub mod sorting;
pub mod stats;

pub enum Periodogram {
    StringLength,
}

pub fn single_band_periodogram(lc: &lightcurve::LightCurve, method: Periodogram) -> Vec<f64> {
    match method {
        Periodogram::StringLength => periodograms::string_length(lc, 1e-3, 3.0, 1e-5),
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::stats::average;
//
//     #[test]
//     fn it_works() {
//         let example = vec![1.0, 2.0, 3.0];
//         let result = average(&example);
//         assert_eq!(result, 2.0);
//     }
// }
