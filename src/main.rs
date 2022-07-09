mod naive_fourier_transform;
mod plot;
mod utilities;

use plotlib::page::Page;

use naive_fourier_transform::naive_fourier_transform;
use plot::view_from_data;
use std::f64::consts::PI;

const RED: &'static str = "#DD3355";
const BLUE: &'static str = "#4287f5";
const GREEN: &'static str = "#34eb56";

fn generate_sin_wave(range: usize, harmonic: i16) -> Vec<f64> {
    let mut v = vec![];
    for i in 0..range {
        v.push((2.0 * PI * (harmonic as f64 * i as f64) / (range as f64)).sin());
    }
    v
}

/*fn generate_cos_wave(range: usize, harmonic: i16) -> Vec<f64> {
    let mut v = vec![];
    for i in 0..range {
        v.push((2.0 * PI * (harmonic as f64 * i as f64) / (range as f64)).cos());
    }
    v
}*/

fn main() {
    let plot_size = 100;

    let sinwave = generate_sin_wave(plot_size, 3);

    let frequencies = naive_fourier_transform(&sinwave);
    let cos_frequencies = frequencies
        .iter()
        .map(|z| 2.0 * z.re / plot_size as f64)
        .collect::<Vec<f64>>();
    let sin_frequencies = frequencies
        .iter()
        .map(|z| 2.0 * z.im / plot_size as f64)
        .collect::<Vec<f64>>();

    let view_amplitudes = view_from_data(&sinwave, plot_size, RED);
    let view_sin_frequencies = view_from_data(&sin_frequencies, plot_size, BLUE);
    let view_cos_frequencies = view_from_data(&cos_frequencies, plot_size, GREEN);

    Page::single(&view_amplitudes)
        .save("plots/amplitudes_plot.svg")
        .unwrap();

    Page::single(&view_sin_frequencies)
        .save("plots/sin_frequencies.svg")
        .unwrap();

    Page::single(&view_cos_frequencies)
        .save("plots/cos_frequencies.svg")
        .unwrap();

    println!("Done !");
}
