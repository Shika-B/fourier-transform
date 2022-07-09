use crate::utilities::round_complex;
use num::complex::Complex;
use std::f64::consts::PI;

/// Should ideally be called with a `data` vector of length a power of two.
pub fn fast_fourier_transform(data: &mut Vec<f64>) -> Vec<Complex<f64>> {
    let old_len = data.len();

    // Find smallest power of two bigger than data.len()
    let mut pow2: u32 = 0;
    loop {
        if data.len() <= 2usize.pow(pow2) {
            break;
        }
        pow2 += 1
    }

    // Pad data with zeroes so that data.len() is a power of 2
    data.resize(2usize.pow(pow2), 0.0);

    let mut v = fast_fourier_transform_2(&data, 0, data.len(), 1);

    // Round floats to 2 decimal points
    for i in 0..v.len() {
        v[i] = round_complex(v[i], 2);
    }

    v.truncate(old_len);
    v
}

// Radix-2 implementation based on https://en.wikipedia.org/wiki/Cooley%E2%80%93Tukey_FFT_algorithm
// The variable `n` in the wiki pseudo is named `size` here, and `s` is `index_jump`
// The start variable is here to do index arithmetic
//
// A few optimization ideas:
// Instead of allocating `size`-many vector with a single element, one could carry a mutable reference over a `size`-sized vector
// Precompute the `complex_factor`s
// Replace recursion with loops

fn fast_fourier_transform_2(
    data: &[f64],
    start: usize,
    size: usize,
    index_jump: usize,
) -> Vec<Complex<f64>> {
    let sizef64 = size as f64;

    if size == 1 {
        let mut v = Vec::with_capacity(1);
        v.push(Complex::new(data[start], 0.0));
        return v;
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
