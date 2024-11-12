#![allow(dead_code)]

use std::{collections::HashMap, io, path::Path};

use plotters::{
    chart::ChartBuilder,
    prelude::{BitMapBackend, Circle, IntoDrawingArea, Text},
    style::{
        full_palette::{
            BLACK, BLUE, BLUE_200, GREEN, GREEN_700, ORANGE, PURPLE, RED, RED_700, RED_900, WHITE,
            YELLOW_500, YELLOW_600,
        },
        Color, IntoFont, TextStyle,
    },
};
use serde::Deserialize;

type Result<T> = std::result::Result<T, MyError>;

#[derive(Debug, Clone, Deserialize)]
struct DataSource {
    team: String,
    game: String,
    game_week: u32,
    total_ck_for: u32,
    shots_from_ck: u32,
    xg: f64,
}

#[derive(Debug, Clone)]
struct Data {
    team: String,
    game: String,
    game_week: u32,
    opponent: String,
    total_ck_for: u32,
    total_ck_against: u32,
    shots_from_ck: u32,
    shots_against_from_ck: u32,
    xg: f64,
    xg_against: f64,
}

#[derive(Debug)]
enum MyError {
    Io(String),
    Csv(String),
    Plotters(String),
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::Io(err) => err,
            Self::Csv(err) => err,
            Self::Plotters(err) => err,
        };
        write!(f, "{:?}", text)
    }
}

impl std::error::Error for MyError {}

impl From<io::Error> for MyError {
    fn from(value: io::Error) -> Self {
        let kind = value.kind().to_string();
        Self::Io(kind)
    }
}

impl From<csv::Error> for MyError {
    fn from(value: csv::Error) -> Self {
        let err = value.to_string();
        Self::Csv(err)
    }
}

impl<E: std::error::Error + Send + Sync> From<plotters::drawing::DrawingAreaErrorKind<E>>
    for MyError
{
    fn from(value: plotters::drawing::DrawingAreaErrorKind<E>) -> Self {
        let err = value.to_string();
        Self::Plotters(err)
    }
}

fn parse_csv<P: AsRef<Path>>(path: P) -> Result<HashMap<u32, Vec<Data>>> {
    let file = std::fs::File::open(path)?;
    let mut csv_reader = csv::Reader::from_reader(file);

    let mut records: HashMap<u32, Vec<Data>> = HashMap::new();
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
                    .filter(|s| **s != ds.team)
                    .map(|s| s.to_string())
                    .collect::<String>();

                let opp_data = data_sources.iter().find(|d| d.team == opponent).unwrap();

                let total_ck_against = opp_data.total_ck_for;
                let shots_against_from_ck = opp_data.shots_from_ck;
                let xg_against = opp_data.xg;

                Data {
                    team: ds.team.clone(),
                    game: ds.game.clone(),
                    game_week: ds.game_week,
                    opponent,
                    total_ck_for: ds.total_ck_for,
                    total_ck_against,
                    shots_from_ck: ds.shots_from_ck,
                    shots_against_from_ck,
                    xg: ds.xg,
                    xg_against,
                }
            })
            .collect::<Vec<_>>();

        records.insert(i, data_i);
    }

    Ok(records)
}

#[derive(Debug, Clone, Default)]
struct TeamData {
    team_name: String,
    total_ck_for: u32,
    total_ck_against: u32,
    shots_from_ck: u32,
    shots_against_from_ck: u32,
    xg: f64,
    xg_against: f64,
}

fn accumulate(input: HashMap<u32, Vec<Data>>) -> Vec<TeamData> {
    let mut team_names: Vec<String> = Vec::new();
    input
        .get(&0)
        .map(|vd| vd.iter().for_each(|d| team_names.push(d.team.clone())));

    let mut accumulated_data: Vec<TeamData> = Vec::new();

    for team_name in team_names {
        let team_data_iter = input
            .iter()
            .map(|(_idx, vd)| vd.iter().find(|d| d.team == team_name).unwrap())
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

        let team_data = TeamData {
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

const OUTPUT: &str = "chart_output/chart.png";

fn draw_plotters(data: Vec<TeamData>) -> Result<()> {
    let mut team_name = vec![];
    let mut total_ck_for = vec![];
    let mut total_ck_against = vec![];
    let mut shots_from_ck = vec![];
    let mut shots_against_from_ck = vec![];
    let mut xg = vec![];
    let mut xg_against = vec![];

    data.iter().for_each(|d| {
        team_name.push(d.team_name.clone());
        total_ck_for.push(d.total_ck_for);
        total_ck_against.push(d.total_ck_against);
        shots_from_ck.push(d.shots_from_ck);
        shots_against_from_ck.push(d.shots_against_from_ck);
        xg.push(d.xg);
        xg_against.push(d.xg_against);
    });

    let xg_conceded_per_shot = xg_against
        .iter()
        .cloned()
        .zip(shots_against_from_ck.clone())
        .map(|(xg, shots)| xg / shots as f64)
        .collect::<Vec<_>>();

    let shot_ratio = shots_against_from_ck
        .iter()
        .cloned()
        .zip(total_ck_against.clone())
        .map(|(shot, tck)| shot as f64 / tck as f64)
        .collect::<Vec<_>>();

    let xy_data = xg_conceded_per_shot
        .iter()
        .cloned()
        .zip(shot_ratio.clone())
        .collect::<Vec<_>>();

    let xy_data = team_name.iter().cloned().zip(xy_data).collect::<Vec<_>>();

    let x_max = xg_conceded_per_shot
        .iter()
        .map(|n| (n * 1000.) as u32)
        .max()
        .unwrap() as f64
        / 1000.;
    let x_max_margin = x_max + (x_max * 10. / 100.);

    let x_min = xg_conceded_per_shot
        .iter()
        .map(|n| (n * 1000.) as u32)
        .min()
        .unwrap() as f64
        / 1000.;
    let x_min_margin = x_min - (x_min * 10. / 100.);

    let y_max = shot_ratio.iter().map(|n| (n * 1000.) as u32).max().unwrap() as f64 / 1000.;
    let y_max_margin = y_max + (y_max * 10. / 100.);

    let y_min = shot_ratio.iter().map(|n| (n * 1000.) as u32).min().unwrap() as f64 / 1000.;
    let y_min_margin = y_min - (y_min * 10. / 100.);

    let root = BitMapBackend::new(OUTPUT, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut scatter_ctx = ChartBuilder::on(&root)
        .margin(10)
        .x_label_area_size(50)
        .y_label_area_size(55)
        .caption("Defensive Corner Proficiency", ("sans-serif", 35))
        .build_cartesian_2d(x_min_margin..x_max_margin, y_min_margin..y_max_margin)?;

    scatter_ctx
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .x_desc("xG per Shot Conceded from Corner Kick")
        .y_desc("Shots Conceded per Corner Kicks Faced")
        .axis_desc_style(("sans-serif", 20))
        .draw()?;

    scatter_ctx.draw_series(xy_data.clone().iter().map(|(name, (x, y))| {
        let color = match name.as_str() {
            "AREMA FC" => BLUE.filled(),
            "Bali United FC" => RED_700.filled(),
            "Borneo FC Samarinda" => ORANGE.filled(),
            "Dewa United FC" => YELLOW_600.filled(),
            "Madura United FC" => RED.filled(),
            "Malut United FC" => RED_700.filled(),
            "PERSEBAYA Surabaya" => GREEN_700.filled(),
            "PERSIS Solo" => RED_700.filled(),
            "PSS Sleman" => GREEN.filled(),
            "PSIS Semarang" => BLUE.filled(),
            "PERSIJA Jakarta" => RED.filled(),
            "PERSIB Bandung" => BLUE.filled(),
            "PERSITA Tangerang" => PURPLE.filled(),
            "PSBS Biak" => BLUE_200.filled(),
            "PSM Makassar" => RED_900.filled(),
            "PERSIK Kediri" => PURPLE.filled(),
            "PS Barito Putera" => YELLOW_500.filled(),
            "PERSIK KEDIRI" => PURPLE.filled(),
            _ => RED.filled(),
        };
        Circle::new((*x as f64, *y), 5, color)
    }))?;

    scatter_ctx.draw_series(xy_data.iter().cloned().map(|(name, (x, y))| {
        Text::new(
            name,
            (x as f64 + (x as f64 * 1. / 100.), y + (y * 1. / 100.)),
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
    draw_plotters(accumulated_data)?;

    Ok(())
}
