use crate::utilities::round_complex;
use num::complex::Complex;
use std::f64::consts::PI;

pub fn fast_fourier_transform(data: &[f64]) -> Vec<Complex<f64>> {
    let mut v = fast_fourier_transform_2(data, 0, data.len(), 1);
    for i in 0..v.len() {
        v[i] = round_complex(v[i], 2);
    }
    v
}

// Implementation based on https://en.wikipedia.org/wiki/Cooley%E2%80%93Tukey_FFT_algorithm
// The variable "n" in the wiki pseudo is called "size" here, and "s" is "index_jump"
// The start variable is here to do index arithmetic instead of copying Vecs.

fn fast_fourier_transform_2(
    data: &[f64],
    start: usize,
    size: usize,
    index_jump: usize,
) -> Vec<Complex<f64>> {
    let sizef64 = size as f64;
    if size == 1 {
        return vec![Complex::new(data[start], 0.0)];
    }

    let mut even_indexed_fft = fast_fourier_transform_2(&data, start, size / 2, 2 * index_jump);
    let mut odd_indexed_fft =
        fast_fourier_transform_2(&data, start + index_jump, size / 2, 2 * index_jump);

    for k in 0..size / 2 {
        let p = even_indexed_fft[k];
        let complex_factor = Complex::from_polar(1.0, -2.0 * PI * (k as f64) / sizef64);
        let q = complex_factor * odd_indexed_fft[k];
        even_indexed_fft[k] = p + q;
        odd_indexed_fft[k] = p - q;
    }
    even_indexed_fft.append(&mut odd_indexed_fft);
    even_indexed_fft
}
