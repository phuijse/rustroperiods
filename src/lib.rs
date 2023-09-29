pub mod periodograms;
pub mod sorting;
pub mod stats;

pub struct LightCurve {
    mjd: Vec<f64>,
    mag: Vec<f64>,
    err: Vec<f64>,
}

pub enum Periodogram {
    StringLength,
    PDM,
    AOV,
    MHAOV,
}

pub fn single_band_periodogram(lc: &LightCurve, method: Periodogram) {
    match method {
        Periodogram::StringLength => periodograms::string_length(lc, 1e-3, 1e-5),
        _ => (),
    }
}

#[cfg(test)]
mod tests {
    use crate::stats::average;

    #[test]
    fn it_works() {
        let example = vec![1.0, 2.0, 3.0];
        let result = average(&example);
        assert_eq!(result, 2.0);
    }
}
