mod fast_fourier_transform;
mod naive_fourier_transform;
mod plot;
mod utilities;

use num::complex::Complex;
use plotlib::page::Page;

use fast_fourier_transform::fast_fourier_transform;
use naive_fourier_transform::naive_fourier_transform;
use plot::view_from_data;

use std::f64::consts::PI;

const RED: &'static str = "#DD3355";
const BLUE: &'static str = "#4287f5";
const GREEN: &'static str = "#34eb56";

// Discrete sin wave generation
fn generate_sin_wave(range: usize, harmonic: i16) -> Vec<f64> {
    let mut v = vec![];
    for i in 0..range {
        v.push((2.0 * PI * (harmonic as f64 * i as f64) / (range as f64)).sin());
    }
    v
}

fn fourier_transforms(data: &mut Vec<f64>) -> (Vec<Complex<f64>>, Vec<Complex<f64>>) {
    (fast_fourier_transform(data), naive_fourier_transform(data))
}

fn main() {
    // Plot only 128 points
    let plot_size = 128;

    let mut sinwave = generate_sin_wave(plot_size, 5);

    let (frequencies, _frequencies2) = fourier_transforms(&mut sinwave);

    // Get the real parts of the Fourier transform
    let real_frequencies = frequencies
        .iter()
        .map(|z| 2.0 * z.re / plot_size as f64)
        .collect::<Vec<f64>>();

    // Get the imaginary parts of the Fourier transform
    let imaginary_frequencies = frequencies
        .iter()
        .map(|z| 2.0 * z.im / plot_size as f64)
        .collect::<Vec<f64>>();

    // Create plotting objects
    let view_amplitudes = view_from_data(&sinwave, plot_size, RED);
    let view_real_frequencies = view_from_data(&real_frequencies, plot_size, BLUE);
    let view_imaginary_frequencies = view_from_data(&imaginary_frequencies, plot_size, GREEN);

    // Saves the plot in .svg files. Open plots.html in your browser if you want to see them laid out next to each others in a single webpage
    Page::single(&view_amplitudes)
        .save("plots/amplitude_plot.svg")
        .unwrap();

    Page::single(&view_real_frequencies)
        .save("plots/real_frequencies.svg")
        .unwrap();

    Page::single(&view_imaginary_frequencies)
        .save("plots/imaginary_frequencies.svg")
        .unwrap();
    println!("Done !");
}
