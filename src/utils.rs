pub trait Stats {
    fn mean(&self) -> f64;
    fn variance(&self) -> f64;
    fn std_dev(&self) -> f64;
}

impl Stats for [f64] {
    fn mean(&self) -> f64 {
        if self.is_empty() {
            return 0.0;
        }
        self.iter().sum::<f64>() / self.len() as f64
    }

    fn variance(&self) -> f64 {
        if self.is_empty() {
            return 0.0;
        }
        let m = self.mean();
        self.iter().map(|&t| (t - m).powi(2)).sum::<f64>() / self.len() as f64
    }

    fn std_dev(&self) -> f64 {
        self.variance().sqrt()
    }
}
