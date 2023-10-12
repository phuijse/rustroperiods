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
//! The periodograms implemented in this crate are not estimators of the PSD per-se, but they do
//! work in a similar way, in the sense that they can be interpreted as the likelihood of a given
//! oscillation frequency being present in the time series. All the periodograms are computed using
//! a brute force strategy, i.e. evaluating an user-defined grid of candidate frequencies. Please
//! see the documentation in the module `periodograms` for information on the methods included
//! in this crate.
//! This crate uses the PyO3 ecosystem of tools to provide interfaces for Python 3. It is intended
//! as a replacement for the P4J python/cython library by the same author of this crate.
pub mod lightcurve;
pub mod periodograms;
pub mod sorting;
pub mod stats;
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray2};
use numpy::{PyArray, PyArrayDyn, PyReadonlyArrayDyn};
use periodograms::{single_band_periodogram, Periodogram};
use pyo3::prelude::*;

#[pyfunction]
#[pyo3(name = "single_band_periodogram")]
fn single_band_periodogram_py<'py>(
    py: Python<'py>,
    mjd: PyReadonlyArrayDyn<'py, f64>,
    mag: PyReadonlyArrayDyn<'py, f64>,
    err: PyReadonlyArrayDyn<'py, f64>,
) -> PyResult<&'py PyArray1<f64>> {
    let lc = lightcurve::LightCurve {
        mjd: mjd.to_vec().unwrap(),
        mag: mag.to_vec().unwrap(),
        err: err.to_vec().unwrap(),
    };
    let per = single_band_periodogram(&lc, Periodogram::StringLength, 1e-3, 3.0, 1e-4);
    Ok(PyArray::from_vec(py, per))
}

#[pymodule]
fn rustroperiods(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(single_band_periodogram_py, m)?)?;
    Ok(())
}
