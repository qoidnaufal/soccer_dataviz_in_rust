use std::{collections::HashMap, path::Path};

use plotters::{
    chart::ChartBuilder,
    coord::ranged1d::{NoDefaultFormatting, ValueFormatter},
    prelude::{BitMapBackend, DiscreteRanged, IntoDrawingArea, IntoSegmentedCoord, Ranged},
    series::Histogram,
    style::{
        full_palette::{BLACK, BLUE, RED},
        Color, ShapeStyle, WHITE,
    },
};
use serde::Deserialize;
use tsg2425::{Result, Teams};

const FIELD_TILT_OUTPUT: &str = "chart_output/field_tilt.png";

#[derive(Debug, Deserialize)]
struct DataSource {
    #[serde(rename = "Team")]
    team: Teams,
    #[serde(rename = "Match")]
    game: String,
    #[serde(rename = "Winning")]
    winning: f64,
    #[serde(rename = "Drawing")]
    drawing: f64,
    #[serde(rename = "Losing")]
    losing: f64,
}

#[derive(Debug, Clone)]
struct FieldTilt {
    team: Teams,
    _opponent: Teams,
    winning: f64,
    drawing: f64,
    losing: f64,
}

struct Data {
    teams: Vec<Teams>,
    field_tilt: Vec<FieldTilt>,
}

fn parse_csv<P: AsRef<Path>>(path: P) -> Result<Data> {
    let file = std::fs::File::open(path)?;
    let mut reader = csv::Reader::from_reader(file);

    let mut data_sources = Vec::new();
    let mut games = Vec::new();
    let mut teams = Vec::new();
    let mut field_tilt = Vec::new();

    for ds in reader.deserialize() {
        let datasource: DataSource = ds?;
        if !games.contains(&datasource.game) {
            games.push(datasource.game.clone());
        }
        if !teams.contains(&datasource.team) {
            teams.push(datasource.team);
        }
        data_sources.push(datasource);
    }

    for game in games {
        data_sources
            .iter()
            .filter(|ds| ds.game == game)
            .for_each(|ds| {
                let teams_involved = ds.game.split("vs").map(|s| s.trim()).collect::<Vec<_>>();
                let opponent = teams_involved
                    .iter()
                    .find(|t| Teams::from(**t) != ds.team)
                    .map(|o| Teams::from(*o))
                    .unwrap();
                let opp_data = data_sources
                    .iter()
                    .find(|d| d.team == opponent && d.game == game)
                    .unwrap();

                let winning = ds.winning / (ds.winning + opp_data.losing);
                let drawing = ds.drawing / (ds.drawing + opp_data.drawing);
                let losing = ds.losing / (ds.losing + opp_data.winning);

                let winning = if winning.is_nan() { 0. } else { winning };
                let drawing = if drawing.is_nan() { 0. } else { drawing };
                let losing = if losing.is_nan() { 0. } else { losing };

                field_tilt.push(FieldTilt {
                    team: ds.team,
                    _opponent: opponent,
                    winning,
                    drawing,
                    losing,
                });
            });
    }

    Ok(Data { teams, field_tilt })
}

struct TeamFieldTiltData {
    team_name: Teams,
    winning: f64,
    drawing: f64,
    losing: f64,
}

#[derive(Debug, Default)]
struct DataFrame {
    teams: Vec<Teams>,
    winning: HashMap<Teams, f64>,
    drawing: HashMap<Teams, f64>,
    losing: HashMap<Teams, f64>,
}

fn create_dataframe(data: Data) -> Result<DataFrame> {
    let mut df = DataFrame::default();
    df.teams = data.teams;
    let team_data = df
        .teams
        .iter()
        .map(|team_name| {
            let team_data_iter = data
                .field_tilt
                .iter()
                .filter(|ft| ft.team == *team_name)
                .collect::<Vec<_>>();
            let len = team_data_iter.len() as f64;

            let winning = team_data_iter.iter().map(|ft| ft.winning).sum::<f64>() / len;
            let drawing = team_data_iter.iter().map(|ft| ft.drawing).sum::<f64>() / len;
            let losing = team_data_iter.iter().map(|ft| ft.losing).sum::<f64>() / len;

            TeamFieldTiltData {
                team_name: *team_name,
                winning,
                drawing,
                losing,
            }
        })
        .collect::<Vec<_>>();

    team_data.iter().for_each(|td| {
        df.winning.insert(td.team_name, td.winning);
        df.drawing.insert(td.team_name, td.drawing);
        df.losing.insert(td.team_name, td.losing);
    });

    println!("{:#?}", df);

    Ok(df)
}

// fn draw_plot(input: DataFrame) -> Result<()> {
//     let root = BitMapBackend::new(FIELD_TILT_OUTPUT, (1800, 768)).into_drawing_area();
//     root.fill(&WHITE)?;

//     let y_max = input
//         .winning
//         .values()
//         .chain(input.drawing.values().into_iter())
//         .chain(input.losing.values().into_iter())
//         .map(|n| (*n * 1000.) as u64)
//         .max()
//         .unwrap() as f64
//         / 1000.;
//     let y_range = 0.0..y_max + (y_max * 0.1);

//     // let a = input.winning.iter().zip(&input.drawing).zip(&input.losing).map(|(((t, w), (_, d)), (_, l))| (t, (w, d, l))).collect::<HashMap<_, _>>();

//     let idx = input.teams.len() as u32;

//     Ok(())
// }

fn main() -> Result<()> {
    let data = parse_csv("dataset/touchfinal3rd.csv")?;
    create_dataframe(data)?;
    // draw_plot(df)?;

    Ok(())
}
