//! the stats module contains

/// Returns the average/sample mean of a floating point array
///
/// # Example
///```
/// # use rustroperiods::stats::sample_mean;
/// let result = sample_mean(&[1.0, 2.0, 3.0]);
/// assert!((result -  2.0).abs() < f64::EPSILON);
///```
#[inline(always)]
pub fn sample_mean(data: &[f64]) -> f64 {
    assert!(data.len() > 0);
    let sum = data.into_iter().sum::<f64>();
    sum / data.len() as f64
}

/// Returns the variance of a floating point array
///
/// # Example
/// ```
/// # use rustroperiods::stats::sample_variance;
/// let result = sample_variance(&[1.0, 2.0, 3.0], true);
/// assert!((result - 1.0).abs() < f64::EPSILON)
/// ```
#[inline(always)]
pub fn sample_variance(data: &[f64], unbiased: bool) -> f64 {
    assert!(data.len() > 0);
    let average = sample_mean(data);
    let sum_of_squares = data
        .into_iter()
        .map(|xi| (xi - average).powi(2))
        .sum::<f64>();
    let mut denominator = data.len() as f64;
    if unbiased {
        denominator -= 1.0;
    }
    sum_of_squares / denominator
}
