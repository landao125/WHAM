
// mean
pub fn mean(values: &[f64]) -> f64 {
    let n  = values.len() as f64;
    let sum = values.iter().sum::<f64>();
    1.0 / n * sum
}

// standard deviation
pub fn std(values: &[f64]) -> f64 {
    var(values).sqrt()
}

// variance
pub fn var(values: &[f64]) -> f64 {
    let n = values.len() as f64;
    let mean = mean(values);
    let sum = values.iter().map({|val| (val - mean)*(val - mean)}).sum::<f64>();
    1.0 / (n-1.0) * sum
}

// covariance
pub fn cov(values1: &[f64], values2: &[f64]) -> f64 {
    let mean1 = mean(values1); 
    let mean2 = mean(values2);
    let n = values1.len() as f64;
    let sum = values1.iter().zip(values2.iter()).map({ |(val1, val2)|
        (val1 - mean1) * (val2 - mean2)
    }).sum::<f64>();
    1.0 / (n-1.0) * sum
}

// correlation
pub fn corr(values1: &[f64], values2: &[f64]) -> f64 {
    let cov = cov(values1, values2);
    let std1 = std(values1);
    let std2 = std(values2);
    cov / (std1 * std2)
}

#[cfg(test)]
mod tests {
    
    #[test]
    fn mean() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let mean = super::mean(&x[..]);
        assert_approx_eq!(mean, 3.0);
    }

    #[test]
    fn std() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let std = super::std(&x[..]);
        let expected = (2.5 as f64).sqrt();
        assert_approx_eq!(std, expected);
    }

    #[test]
    fn var() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let var = super::var(&x[..]);
        assert_approx_eq!(var, 2.5);
    }    

    #[test]
    fn corr() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![3.0, 2.0, 1.0, 0.0, -1.0];

        let corr = super::corr(&x[..], &x[..]);
        assert_approx_eq!(corr, 1.0);

        let corr = super::corr(&x[..], &y[..]);
        assert_approx_eq!(corr, -1.0);
    }
}
