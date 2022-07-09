use num::complex::Complex;

fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i32.pow(decimals) as f64;
    (x * y).round() / y
}

pub fn round_complex(x: Complex<f64>, decimals: u32) -> Complex<f64> {
    Complex::new(round(x.re, decimals), round(x.im, decimals))
}
