use std::path::Path;

use anyhow::Result;
use plotters::{prelude::*, style::full_palette::GREY_300};

use crate::parser::Timing;

#[allow(
    clippy::too_many_lines,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
pub fn plot_histogram<P>(
    path: P,
    data: &[(usize, (Timing, Timing, Timing))],
    error: bool,
) -> Result<()>
where
    P: AsRef<Path>,
{
    let total_bar_width = 0.8;
    let bar_width = total_bar_width / 3.0;
    let canvas_size = (640, 480);

    let root = SVGBackend::new(&path, canvas_size).into_drawing_area();
    root.fill(&WHITE)?;

    let max_x = data
        .iter()
        .map(|&(day, _)| day)
        .max()
        .ok_or_else(|| anyhow::anyhow!("Data is empty"))?;
    let max_y = data
        .iter()
        .flat_map(|(_, (parse, part1, part2))| {
            [
                parse.median * parse.scale.to_decimal(),
                part1.median * part1.scale.to_decimal(),
                part2.median * part2.scale.to_decimal(),
            ]
        })
        .max_by(f64::total_cmp)
        .ok_or_else(|| anyhow::anyhow!("Data is empty"))?;
    let max_y = 10.0f64.powf(max_y.log10().ceil());

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Run time", ("sans-serif", 30.0))
        .build_cartesian_2d(0f64..(max_x + 1) as f64, (1.0..max_y).log_scale())?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(GREY_300)
        .x_labels(max_x + 2)
        .x_label_formatter(&|x| {
            let idx = x.round() as usize;
            if idx > 0 && idx <= max_x {
                idx.to_string()
            } else {
                String::new()
            }
        })
        .y_desc("Time")
        .y_label_formatter(&|x| match x.log10().round() as usize {
            0 => "1ns".to_string(),
            1 => "10ns".to_string(),
            2 => "100ns".to_string(),
            3 => "1µs".to_string(),
            4 => "10µs".to_string(),
            5 => "100µs".to_string(),
            6 => "1ms".to_string(),
            7 => "10ms".to_string(),
            8 => "100ms".to_string(),
            9 => "1s".to_string(),
            10 => "10s".to_string(),
            11 => "100s".to_string(),
            _ => format!("{x:e}"),
        })
        .x_desc("Day")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    let getter = |(x, y, z), idx| match idx {
        0 => x,
        1 => y,
        2 => z,
        _ => unreachable!(),
    };

    for (idx, name) in ["Parse", "Part 1", "Part 2"].into_iter().enumerate() {
        let color = Palette99::pick(idx + 3).to_rgba();
        chart
            .draw_series(data.iter().map(|&(day, values)| {
                let t = getter(values, idx);
                let y = t.median * t.scale.to_decimal();
                let center = day as f64;

                let x_start = (idx as f64).mul_add(bar_width, center - total_bar_width / 2.0);
                let x_end = x_start + bar_width;

                Rectangle::new([(x_start, 1.0), (x_end, y)], color.filled())
            }))?
            .label(name)
            .legend(move |(x, y)| Rectangle::new([(x, y - 3), (x + 20, y + 3)], color.filled()));

        if error {
            chart.draw_series(data.iter().map(|&(day, values)| {
                let t = getter(values, idx);
                let y = t.median * t.scale.to_decimal();
                let center = day as f64;

                let x_start = (idx as f64).mul_add(bar_width, center - total_bar_width / 2.0);
                let x_error_bar = x_start + bar_width / 2.0;

                ErrorBar::new_vertical(x_error_bar, y - t.mad, y, y + t.mad, color, 5)
            }))?;
        }
    }

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .background_style(WHITE.mix(0.5))
        .border_style(BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}
