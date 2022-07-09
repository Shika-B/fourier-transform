use plotlib::{
    repr::Plot,
    style::{PointMarker, PointStyle},
    view::ContinuousView,
};

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
            .size(2.5),
    )
}
fn create_view(plot: Plot, plot_size: usize) -> ContinuousView {
    ContinuousView::new()
        .add(plot)
        .x_range(0.0, plot_size as f64)
        .y_range(-1.0, 1.0)
}

pub fn view_from_data(data: &[f64], plot_size: usize, color: &'static str) -> ContinuousView {
    create_view(partial_plot(data, plot_size, color), plot_size)
}
