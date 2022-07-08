mod naive_fourier_transform;
mod utility;
use naive_fourier_transform::naive_fourier_transform;
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;
use std::f64::consts::PI;

const RED: &'static str = "#DD3355";
const BLUE: &'static str = "#4287f5";
const GREEN: &'static str = "#34eb56";

fn partial_plot(data: &[f64], amount_to_plot: usize, color: &'static str) -> Plot {
    let plot_points: Vec<(f64, f64)> = data[0..amount_to_plot]
        .iter()
        .enumerate()
        .map(|(index, value)| (index as f64, *value as f64))
        .collect();
    Plot::new(plot_points).point_style(
        PointStyle::new()
            .marker(PointMarker::Circle)
            .colour(color)
            .size(1.5),
    )
}

fn generate_sin_wave(range: usize, harmonic: i16) -> Vec<f64> {
    let mut v = vec![];
    for i in 0..range {
        v.push((2.0 * PI * (harmonic as f64 * i as f64) / (range as f64)).sin());
    }
    v
}

fn generate_cos_wave(range: usize, harmonic: i16) -> Vec<f64> {
    let mut v = vec![];
    for i in 0..range {
        v.push((2.0 * PI * (harmonic as f64 * i as f64) / (range as f64)).cos());
    }
    v
}

fn main() {
    /*let file: Vec<u8> = vec![];
    let wave = Wave::new(&file);
    println!("{:#?}", wave.header);
    */
    let plot_size = 100;

    let sinwave = generate_sin_wave(plot_size, 3);
    let coswave = generate_cos_wave(plot_size, 3);

    let frequencies = naive_fourier_transform(&sinwave);

    let cos_frequencies = frequencies
        .iter()
        .map(|z| 2.0 * z.re / plot_size as f64)
        .collect::<Vec<f64>>();

    let sin_frequencies = frequencies
        .iter()
        .map(|z| 2.0 * z.im / plot_size as f64)
        .collect::<Vec<f64>>();

    let plot_amplitudes = partial_plot(&sinwave, plot_size, RED);

    let plot_sin_frequencies = partial_plot(&sin_frequencies, plot_size, BLUE);
    let plot_cos_frequencies = partial_plot(&cos_frequencies, plot_size, GREEN);
    let view = ContinuousView::new()
        .add(plot_amplitudes)
        .x_range(0.0, plot_size as f64)
        .y_range(-1.0, 1.0);

    let view2 = ContinuousView::new()
        .add(plot_sin_frequencies)
        .x_range(0.0, plot_size as f64)
        .y_range(-1.0, 1.0);

    let view3 = ContinuousView::new()
        .add(plot_cos_frequencies)
        .x_range(0.0, plot_size as f64)
        .y_range(-1.0, 1.0);

    Page::single(&view)
        .save("plots/amplitudes_plot.svg")
        .unwrap();
    Page::single(&view2)
        .save("plots/sin_frequencies.svg")
        .unwrap();
    Page::single(&view3)
        .save("plots/cos_frequencies.svg")
        .unwrap();

    /*println!("{:?}", &sinwave);
    println!("{:?}", &frequencies);
    println!("{:?}", &coswave);
    println!("{:?}", naive_fourier_transform(&coswave));
    */
    println!("all good");
}
