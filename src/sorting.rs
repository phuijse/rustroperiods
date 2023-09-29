/// Returns the indexes that sort a `data`. Works with floating point vectors.
/// ```
/// use rustroperiods::sorting::argsort;
/// let input: Vec<f64> = vec![2.0, 1.0, 3.0];
/// let result: Vec<usize> = vec![1, 0, 2];
/// assert_eq!(argsort(&input), result);
/// ```
pub fn argsort(data: &[f64]) -> Vec<usize> {
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    indices.sort_by(|&i, &j| data[i].total_cmp(&data[j]));
    indices
}

/// Returns the indexes of the `npeaks` highest local maxima in `data`
/// ```
/// use rustroperiods::sorting::find_peaks;
/// let input: Vec<f64> = vec![0.0, 1.0, 2.0, 1.0, 0.0];
/// let result: Vec<usize> = vec![2];
/// assert_eq!(find_peaks(&input, 1), result);
/// ```
pub fn find_peaks(data: &[f64], npeaks: usize) -> Vec<usize> {
    assert!(npeaks > 0);
    let mut local_maxima = (1..data.len() - 1)
        .filter(|&i| (data[i] >= data[i - 1]) & (data[i] >= data[i + 1]))
        .collect::<Vec<usize>>();
    assert!(npeaks <= local_maxima.len());
    local_maxima.sort_by(|&i, &j| data[i].total_cmp(&data[j]));
    local_maxima[0..npeaks].to_vec()
}
