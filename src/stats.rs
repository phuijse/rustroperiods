/// Returns the average/sample mean of a floating point array
///
///```
///use rustroperiods::stats::average;
///let input = [1.0, 2.0, 3.0];
///assert!((average(&input) -  2.0).abs() < f64::EPSILON);
///```
#[inline(always)]
pub fn average(data: &[f64]) -> f64 {
    let sum = data.into_iter().sum::<f64>();
    sum / data.len() as f64
}

/// Returns the variance of a floating point array
///
/// ```
/// use rustroperiods::stats::variance;
/// let input = [1.0, 2.0, 3.0];
/// assert!((variance(&input) - 1.0).abs() < f64::EPSILON)
/// ```
#[inline(always)]
pub fn variance(data: &[f64]) -> f64 {
    let average = average(data);
    let sum_of_squares = data.into_iter().map(|xi| (xi - average).powi(2)).sum::<f64>();
    sum_of_squares / (data.len() as f64 - 1.0)
}
