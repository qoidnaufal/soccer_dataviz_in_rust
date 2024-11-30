use std::{collections::HashMap, path::Path};

use plotters::{
    chart::ChartBuilder,
    prelude::{BitMapBackend, Circle, IntoDrawingArea, Text},
    style::{Color, IntoFont, RGBColor, TextStyle, BLACK, WHITE},
};
use serde::Deserialize;
use tsg2425::{Result, Teams};

const CKA_OUTPUT: &str = "chart_output/cka.png";
const CKD_OUTPUT: &str = "chart_output/ckd.png";

#[derive(Debug, Clone, Deserialize)]
pub struct DataSource {
    team: Teams,
    game: String,
    game_week: u32,
    total_ck_for: u32,
    shots_from_ck: u32,
    xg: f64,
}

#[derive(Debug)]
pub struct CornerKickData {
    team: Teams,
    _game_week: u32,
    _opponent: Teams,
    total_ck_for: u32,
    total_ck_against: u32,
    shots_from_ck: u32,
    shots_against_from_ck: u32,
    xg: f64,
    xg_against: f64,
}

pub fn parse_csv<P: AsRef<Path>>(path: P) -> Result<HashMap<u32, Vec<CornerKickData>>> {
    let file = std::fs::File::open(path)?;
    let mut csv_reader = csv::Reader::from_reader(file);

    let mut records: HashMap<u32, Vec<CornerKickData>> = HashMap::new();
    let mut data_sources: Vec<DataSource> = Vec::new();

    for ds in csv_reader.deserialize() {
        let data_source: DataSource = ds?;
        data_sources.push(data_source)
    }

    for i in 0..10 {
        let data_i = data_sources
            .iter()
            .filter(|ds| ds.game_week == i + 1)
            .map(|ds| {
                let teams = ds.game.split("vs").map(|s| s.trim()).collect::<Vec<_>>();
                let opponent = teams
                    .iter()
                    .find(|s| Teams::from(**s) != ds.team)
                    .map(|s| Teams::from(*s))
                    .unwrap();

                let opp_data = data_sources
                    .iter()
                    .find(|d| d.team == opponent && d.game_week == i + 1)
                    .unwrap();

                let total_ck_against = opp_data.total_ck_for;
                let shots_against_from_ck = opp_data.shots_from_ck;
                let xg_against = opp_data.xg;

                CornerKickData {
                    team: ds.team,
                    _game_week: ds.game_week,
                    _opponent: opponent,
                    total_ck_for: ds.total_ck_for,
                    total_ck_against,
                    shots_from_ck: ds.shots_from_ck,
                    shots_against_from_ck,
                    xg: ds.xg,
                    xg_against,
                }
            })
            .collect::<Vec<_>>();

        println!("{:#?}", data_i);
        records.insert(i, data_i);
    }

    Ok(records)
}

#[derive(Debug, Clone)]
pub struct TeamCKData {
    team_name: Teams,
    total_ck_for: u32,
    total_ck_against: u32,
    shots_from_ck: u32,
    shots_against_from_ck: u32,
    xg: f64,
    xg_against: f64,
}

pub fn accumulate(input: HashMap<u32, Vec<CornerKickData>>) -> Vec<TeamCKData> {
    let mut team_names: Vec<Teams> = Vec::new();
    if let Some(vd) = input.get(&0) {
        vd.iter().for_each(|d| team_names.push(d.team))
    }

    let mut accumulated_data: Vec<TeamCKData> = Vec::new();

    for team_name in team_names {
        let team_data_iter = input
            .values()
            .map(|vd| {
                vd.iter()
                    .filter(|d| d.team == team_name)
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();
        let total_ck_for = team_data_iter.iter().map(|d| d.total_ck_for).sum::<u32>();
        let total_ck_against = team_data_iter
            .iter()
            .map(|d| d.total_ck_against)
            .sum::<u32>();
        let shots_from_ck = team_data_iter.iter().map(|d| d.shots_from_ck).sum::<u32>();
        let shots_against_from_ck = team_data_iter
            .iter()
            .map(|d| d.shots_against_from_ck)
            .sum::<u32>();
        let xg = team_data_iter.iter().map(|d| d.xg).sum::<f64>();
        let xg_against = team_data_iter.iter().map(|d| d.xg_against).sum::<f64>();

        let team_data = TeamCKData {
            team_name,
            total_ck_for,
            total_ck_against,
            shots_from_ck,
            shots_against_from_ck,
            xg,
            xg_against,
        };

        accumulated_data.push(team_data);
    }

    accumulated_data
}

#[derive(Debug, Clone)]
pub struct DataFrame {
    pub team_name: Vec<Teams>,
    pub total_ck_for: Vec<u32>,
    pub total_ck_against: Vec<u32>,
    pub shots_from_ck: Vec<u32>,
    pub shots_against_from_ck: Vec<u32>,
    pub xg: Vec<f64>,
    pub xg_against: Vec<f64>,
}

pub fn create_dataframe(data: Vec<TeamCKData>) -> DataFrame {
    let mut team_name = vec![];
    let mut total_ck_for = vec![];
    let mut total_ck_against = vec![];
    let mut shots_from_ck = vec![];
    let mut shots_against_from_ck = vec![];
    let mut xg = vec![];
    let mut xg_against = vec![];

    data.iter().for_each(|d| {
        team_name.push(d.team_name);
        total_ck_for.push(d.total_ck_for);
        total_ck_against.push(d.total_ck_against);
        shots_from_ck.push(d.shots_from_ck);
        shots_against_from_ck.push(d.shots_against_from_ck);
        xg.push(d.xg);
        xg_against.push(d.xg_against);
    });

    DataFrame {
        team_name,
        total_ck_for,
        total_ck_against,
        shots_from_ck,
        shots_against_from_ck,
        xg,
        xg_against,
    }
}

pub fn plot_cka(df: &DataFrame) -> Result<()> {
    let xg_per_shot = df
        .xg
        .iter()
        .zip(&df.shots_from_ck)
        .map(|(xg, shot)| if *shot == 0 { 0. } else { xg / *shot as f64 })
        .collect::<Vec<_>>();

    let shot_per_ck = df
        .shots_from_ck
        .iter()
        .zip(&df.total_ck_for)
        .map(|(shot, ck)| *shot as f64 / *ck as f64)
        .collect::<Vec<_>>();

    let xy_data = xg_per_shot.iter().zip(&shot_per_ck).collect::<Vec<_>>();
    let plot_data = df.team_name.iter().zip(&xy_data).collect::<Vec<_>>();

    let x_max = xg_per_shot
        .iter()
        .map(|n| (*n * 1000.) as u32)
        .max()
        .unwrap() as f64
        / 1000.;

    let x_min = xg_per_shot
        .iter()
        .map(|n| (*n * 1000.) as u32)
        .min()
        .unwrap() as f64
        / 1000.;

    let y_max = shot_per_ck
        .iter()
        .map(|n| (*n * 1000.) as u32)
        .max()
        .unwrap() as f64
        / 1000.;

    let y_min = shot_per_ck
        .iter()
        .map(|n| (*n * 1000.) as u32)
        .min()
        .unwrap() as f64
        / 1000.;

    let x_max_padding = x_max + (x_max * 10. / 100.);
    let x_min_padding = x_min - (x_min * 10. / 100.);

    let y_max_padding = y_max + (y_max * 10. / 100.);
    let y_min_padding = y_min - (y_min * 10. / 100.);

    let root = BitMapBackend::new(CKA_OUTPUT, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut scatter_ctx = ChartBuilder::on(&root)
        .margin(10)
        .x_label_area_size(50)
        .y_label_area_size(55)
        .caption("Attacking Corner Proficiency", ("sans-serif", 35))
        .build_cartesian_2d(x_min_padding..x_max_padding, y_min_padding..y_max_padding)?;

    scatter_ctx
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .x_desc("xG per Shot from Corner Kick")
        .y_desc("Shot per Corner Kick")
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

    let root = BitMapBackend::new(CKD_OUTPUT, (1024, 768)).into_drawing_area();
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
        .y_desc("Shot Conceded per Corner Kick Faced")
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

fn main() -> Result<()> {
    let path = "dataset/xg_corner.csv";
    let parsed_data = parse_csv(path)?;

    let accumulated_data = accumulate(parsed_data);
    let df = create_dataframe(accumulated_data);

    plot_cka(&df)?;
    plot_ckd(&df)?;

    Ok(())
}
