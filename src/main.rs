use fftr::*;
use plotters::prelude::*;
use std::f32::consts::PI;

// Working through this jupiter notebook to understand the concepts
// https://github.com/thatSaneKid/fourier/blob/master/Fourier%20Transform%20-%20A%20Visual%20Introduction.ipynb

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = 3.0;
    let t = arange(0., 2., 0.001);
    let cos_wav: Vec<_> = t.iter().map(|t| (2. * PI * f * t).cos()).collect();

    // plot
    let root = BitMapBackend::new("plots/cos_wav.png", (640, 640/3)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Cosine Wave with 3 Hz frequency", ("sans-serif", 20).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(50)
        .build_cartesian_2d(-0.1f32..2.1f32, -1.1f32..1.1f32)?;

    chart
        .draw_series(LineSeries::new(
            std::iter::zip(t, cos_wav),
            &BLUE,
        ))?;

    chart
        .configure_mesh()
        .x_desc("Time (in seconds)")
        .x_labels(9)
        .x_max_light_lines(0)
        .x_label_formatter(&|x| format!("{:.2}", x))
        .y_desc("Amplitude")
        .y_labels(9)
        .y_max_light_lines(0)
        .y_label_formatter(&|y| format!("{:.2}", y))
        .draw()?;

    root.present()?;

    Ok(())
}
