use anyhow::Result;
use chrono::{prelude::*, Duration};
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

fn main() {
    console_error_panic_hook::set_once();

    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    render_plot(canvas).unwrap();
}

fn render_plot(
    canvas: HtmlCanvasElement,
) -> Result<impl Fn((i32, i32)) -> Option<(DateTime<Utc>, i32)>> {
    let backend = match CanvasBackend::with_canvas_object(canvas) {
        None => return Err(anyhow::anyhow!("cannot init plotters canvas backend")),
        Some(backend) => backend,
    };

    let draw_area = backend.into_drawing_area();
    draw_area.fill(&WHITE)?;

    let x_range = Utc::now()..(Utc::now() + Duration::minutes(10));
    let y_range = 0i32..10i32;
    let data = (0..=10)
        .enumerate()
        .map(|(i, n)| (Utc::now() + Duration::minutes(i as i64), n as i32));

    let mut chart = ChartBuilder::on(&draw_area)
        .margin(25u32)
        .caption("test plot", FontDesc::from(("sans-serif", 12.0)))
        .x_label_area_size(30u32)
        .y_label_area_size(30u32)
        .build_cartesian_2d(x_range, y_range)?;

    chart.draw_series(LineSeries::new(data, RED))?;

    chart
        .configure_mesh()
        .x_labels(4)
        .y_labels(4)
        .x_label_formatter(&|x| x.format("%H:%M:%S %.3f").to_string())
        .draw()?;

    draw_area.present()?;

    Ok(chart.into_coord_trans())
}
