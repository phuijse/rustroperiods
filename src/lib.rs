pub struct LightCurve {
    mjd: Vec<f64>,
    mag: Vec<f64>,
    err: Vec<f64>
}

/// Returns the indexes that sort a vector of numeric values. Works with floating point vectors.
/// ```
/// use rustronomy::argsort;
/// let input: Vec<f64> = vec![2.0, 1.0, 3.0];
/// let result: Vec<usize> = vec![1, 0, 2];
/// assert_eq!(argsort(&input), result);
/// ```
pub fn argsort<T>(data: &Vec<T>) -> Vec<usize>  where T: PartialOrd{
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    indices.sort_by(|&i, &j| data[i].partial_cmp(&data[j]).unwrap());
    indices
}

/// Performs the epoch folding transformation
/// phase = modulo(time, period)/period
fn fold(times: &Vec<f64>, period: f64) -> Vec<f64> {
    times.iter().map(|time: &f64| (time % period)/period).collect()
}

fn average(data: &Vec<f64>) -> f64 {
    let n: f64 = data.len() as f64;
    let sum = data.iter().sum::<f64>();
    sum/n
}

fn variance(data: &Vec<f64>) -> f64 {
    let n: f64 = data.len() as f64;
    let average: f64 = average(data);
    let sum_of_squares = data.iter().map(|xi| (xi-average).powf(2.0)).sum::<f64>();
    sum_of_squares/(n-1.0)
}

pub fn periodogram(lc: &LightCurve, printit: bool) {
    let n_samples = lc.mag.len() as f64;
    let mag_variance: f64 = variance(&lc.mag);
    
    for k in 0..1_000_000 {
        let trial_frequency: f64 = 0.001 + (k as f64)*1e-5;
        let phase: Vec<f64> = fold(&lc.mjd, trial_frequency.powi(-1));
        let folded_indices: Vec<usize> = argsort(&phase);

        let mut string_length: f64 = 0.0;
        for i in 1..lc.mag.len() {
            string_length += (lc.mag[folded_indices[i]]-lc.mag[folded_indices[i-1]]).powi(2);
        }
        string_length /= 2.0*n_samples*mag_variance;
        if printit {
            println!("{trial_frequency},{string_length}");
        }
}
 

}


#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn it_works() {
        let example = vec![1.0, 2.0, 3.0];
        let result = average(&example);
        assert_eq!(result, 2.0);
    }
}
