use crate::utilities::round_complex;
use num::complex::Complex;
use std::f64::consts::PI;

pub fn naive_fourier_transform(data: &[f64]) -> Vec<Complex<f64>> {
    let mut v = vec![];
    let n = data.len();
    let nf64 = n as f64;
    for j in 0..n {
        let mut sum_j: Complex<f64> = Complex::new(0.0, 0.0);
        for k in 0..n {
            sum_j +=
                (data[k] as f64) * Complex::from_polar(1.0, -2.0 * PI * ((k * j) as f64) / nf64);
        }
        v.push(round_complex(sum_j, 2));
    }
    v
}
