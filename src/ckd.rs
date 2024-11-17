use plotters::{
    chart::ChartBuilder,
    prelude::{BitMapBackend, Circle, IntoDrawingArea, Text},
    style::{Color, IntoFont, RGBColor, TextStyle, BLACK, WHITE},
};

use crate::{DataFrame, Result};

const OUTPUT: &str = "chart_output/ckd.png";

pub fn plot_ckd(df: &DataFrame) -> Result<()> {
    let xg_conceded_per_shot = df
        .xg_against
        .iter()
        .zip(&df.shots_against_from_ck)
        .map(|(xg, shots)| xg / *shots as f64)
        .collect::<Vec<_>>();

    let shot_ratio = df
        .shots_against_from_ck
        .iter()
        .zip(&df.total_ck_against)
        .map(|(shot, tck)| *shot as f64 / *tck as f64)
        .collect::<Vec<_>>();

    let xy_data = xg_conceded_per_shot
        .iter()
        .zip(&shot_ratio)
        .collect::<Vec<_>>();

    let plot_data = df.team_name.iter().zip(&xy_data).collect::<Vec<_>>();

    let x_max = xg_conceded_per_shot
        .iter()
        .map(|n| (n * 1000.) as u32)
        .max()
        .unwrap() as f64
        / 1000.;
    let x_max_padding = x_max + (x_max * 10. / 100.);

    let x_min = xg_conceded_per_shot
        .iter()
        .map(|n| (n * 1000.) as u32)
        .min()
        .unwrap() as f64
        / 1000.;
    let x_min_padding = x_min - (x_min * 10. / 100.);

    let y_max = shot_ratio.iter().map(|n| (n * 1000.) as u32).max().unwrap() as f64 / 1000.;
    let y_max_padding = y_max + (y_max * 10. / 100.);

    let y_min = shot_ratio.iter().map(|n| (n * 1000.) as u32).min().unwrap() as f64 / 1000.;
    let y_min_padding = y_min - (y_min * 10. / 100.);

    let root = BitMapBackend::new(OUTPUT, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut scatter_ctx = ChartBuilder::on(&root)
        .margin(10)
        .x_label_area_size(50)
        .y_label_area_size(55)
        .caption("Defensive Corner Proficiency", ("sans-serif", 35))
        .build_cartesian_2d(x_min_padding..x_max_padding, y_min_padding..y_max_padding)?;

    scatter_ctx
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .x_desc("xG per Shot Conceded from Corner Kick")
        .y_desc("Shots Conceded per Corner Kicks Faced")
        .axis_desc_style(("sans-serif", 20))
        .draw()?;

    scatter_ctx.draw_series(plot_data.iter().map(|(name, (x, y))| {
        let color: RGBColor = (**name).into();
        Circle::new((**x, **y), 5, color.filled())
    }))?;

    scatter_ctx.draw_series(plot_data.iter().map(|(name, (x, y))| {
        Text::new(
            name.to_string(),
            (**x + (**x * 1. / 100.), **y + (**y * 1. / 100.)),
            TextStyle::from(("sans-serif", 20).into_font()).color(&BLACK),
        )
    }))?;

    root.present()?;

    Ok(())
}
