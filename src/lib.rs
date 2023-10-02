pub mod lightcurve;
pub mod periodograms;
pub mod sorting;
pub mod stats;

pub enum Periodogram {
    StringLength,
}

pub fn single_band_periodogram(lc: &lightcurve::LightCurve, method: Periodogram) -> Vec<f64>{
    match method {
        Periodogram::StringLength => periodograms::string_length(lc, 1e-3, 3.0, 1e-5)
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
