//! Module with sorting functions

/// Returns the indexes that sort the floating point array `data`.
///
/// # Examples
/// ```
/// # use rustroperiods::sorting::argsort;
/// let input: Vec<f64> = vec![2.0, 1.0, 3.0];
/// let result = argsort(&input);
/// assert_eq!(argsort(&input), vec![1, 0, 2]);
/// ```
///
pub fn argsort(data: &[f64]) -> Vec<usize> {
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    indices.sort_unstable_by(|&i, &j| data[i].total_cmp(&data[j]));
    indices
}

/// Returns the indexes of the `npeaks` highest local maxima in the floating point array `data`.
///
/// # Examples
/// ```
/// # use rustroperiods::sorting::find_peaks;
/// let input: Vec<f64> = vec![0.0, 1.0, 2.0, 1.0, 0.0];
/// let result = find_peaks(&input, 1);
/// assert_eq!(result, vec![2]);
/// ```
///
pub fn find_peaks(data: &[f64], npeaks: usize) -> Vec<usize> {
    assert!(npeaks > 0);
    let mut local_maxima = (1..data.len() - 1)
        .filter(|&i| (data[i] >= data[i - 1]) & (data[i] >= data[i + 1]))
        .collect::<Vec<usize>>();
    assert!(npeaks <= local_maxima.len());
    local_maxima.sort_unstable_by(|&i, &j| data[i].total_cmp(&data[j]));
    local_maxima[0..npeaks].to_vec()
}
