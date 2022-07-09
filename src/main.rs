mod fast_fourier_transform;
mod naive_fourier_transform;
mod plot;
mod utilities;

use plotlib::page::Page;

use naive_fourier_transform::naive_fourier_transform;
use plot::view_from_data;
use std::f64::consts::PI;

use crate::fast_fourier_transform::fast_fourier_transform;

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

fn main() {
    // Plot only 128 points
    let plot_size = 128;

    let sinwave = generate_sin_wave(plot_size, 5);

    let frequencies = naive_fourier_transform(&sinwave);

    // Get the cos coefficients in Fourier series decomposition
    let cos_frequencies = frequencies
        .iter()
        .map(|z| 2.0 * z.re / plot_size as f64)
        .collect::<Vec<f64>>();

    // Get the sin coefficients in Fourier series decomposition
    let sin_frequencies = frequencies
        .iter()
        .map(|z| 2.0 * z.im / plot_size as f64)
        .collect::<Vec<f64>>();

    // Create plotting objects
    let view_amplitudes = view_from_data(&sinwave, plot_size, RED);
    let view_sin_frequencies = view_from_data(&sin_frequencies, plot_size, BLUE);
    let view_cos_frequencies = view_from_data(&cos_frequencies, plot_size, GREEN);

    // Saves the plot in .svg files. Open plots.html in your browser if you want to see them laid out next to each others in a single webpage
    Page::single(&view_amplitudes)
        .save("plots/amplitudes_plot.svg")
        .unwrap();

    Page::single(&view_sin_frequencies)
        .save("plots/sin_frequencies.svg")
        .unwrap();

    Page::single(&view_cos_frequencies)
        .save("plots/cos_frequencies.svg")
        .unwrap();
    println!(
        "{:?}",
        naive_fourier_transform(&sinwave) == fast_fourier_transform(&sinwave)
    );
    println!("Done !");
}
