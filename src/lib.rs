mod cka;
mod ckd;

use std::{collections::HashMap, io, path::Path};

use plotters::style::{
    full_palette::{
        BLUE, BLUE_200, GREEN, GREEN_700, ORANGE, PURPLE, RED, RED_500, RED_700, RED_900,
        YELLOW_500, YELLOW_600,
    },
    RGBColor,
};
use serde::Deserialize;

pub use cka::plot_cka;
pub use ckd::plot_ckd;

pub type Result<T> = std::result::Result<T, MyError>;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
pub enum Teams {
    #[serde(rename = "AREMA FC")]
    AremaFC,
    #[serde(rename = "Bali United FC")]
    BaliUnitedFC,
    #[serde(rename = "Borneo FC Samarinda")]
    BorneoFCSamarinda,
    #[serde(rename = "Dewa United FC")]
    DewaUnitedFC,
    #[serde(rename = "Madura United FC")]
    MaduraUnitedFC,
    #[serde(rename = "Malut United FC")]
    MalutUnitedFC,
    #[serde(rename = "PERSEBAYA Surabaya")]
    PersebayaSurabaya,
    #[serde(rename = "PERSIS Solo")]
    PersisSolo,
    #[serde(rename = "PSS Sleman")]
    PssSleman,
    #[serde(rename = "PSIS Semarang")]
    PsisSemarang,
    #[serde(rename = "PERSIJA Jakarta")]
    PersijaJakarta,
    #[serde(rename = "PERSIB Bandung")]
    PersibBandung,
    #[serde(rename = "PERSITA Tangerang")]
    PersitaTangerang,
    #[serde(rename = "PSBS Biak")]
    PsbsBiak,
    #[serde(rename = "PSM Makassar")]
    PsmMakassar,
    #[serde(rename = "PERSIK Kediri")]
    PersikKediri,
    #[serde(rename = "PS Barito Putera")]
    PSBaritoPutera,
    #[serde(rename = "Semen Padang FC")]
    SemenPadangFC,
}

impl std::fmt::Display for Teams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Teams::AremaFC => "AREMA FC",
            Teams::BaliUnitedFC => "Bali United FC",
            Teams::BorneoFCSamarinda => "Borneo FC Samarinda",
            Teams::DewaUnitedFC => "Dewa United FC",
            Teams::MaduraUnitedFC => "Madura United FC",
            Teams::MalutUnitedFC => "Malut United FC",
            Teams::PersebayaSurabaya => "PERSEBAYA Surabaya",
            Teams::PersisSolo => "PERSIS Solo",
            Teams::PssSleman => "PSS Sleman",
            Teams::PsisSemarang => "PSIS Semarang",
            Teams::PersijaJakarta => "PERSIJA Jakarta",
            Teams::PersibBandung => "PERSIB Bandung",
            Teams::PersitaTangerang => "PERSITA Tangerang",
            Teams::PsbsBiak => "PSBS Biak",
            Teams::PsmMakassar => "PSM Makassar",
            Teams::PersikKediri => "PERSIK Kediri",
            Teams::PSBaritoPutera => "PS Barito Putera",
            Teams::SemenPadangFC => "Semen Padang FC",
        };
        write!(f, "{name}")
    }
}

impl From<Teams> for RGBColor {
    fn from(value: Teams) -> Self {
        match value {
            Teams::AremaFC => BLUE,
            Teams::BaliUnitedFC => RED_700,
            Teams::BorneoFCSamarinda => ORANGE,
            Teams::DewaUnitedFC => YELLOW_600,
            Teams::MaduraUnitedFC => RED,
            Teams::MalutUnitedFC => RED_700,
            Teams::PersebayaSurabaya => GREEN_700,
            Teams::PersisSolo => RED_700,
            Teams::PssSleman => GREEN,
            Teams::PsisSemarang => BLUE,
            Teams::PersijaJakarta => RED,
            Teams::PersibBandung => BLUE,
            Teams::PersitaTangerang => PURPLE,
            Teams::PsbsBiak => BLUE_200,
            Teams::PsmMakassar => RED_900,
            Teams::PersikKediri => PURPLE,
            Teams::PSBaritoPutera => YELLOW_500,
            Teams::SemenPadangFC => RED_500,
        }
    }
}

impl From<&Teams> for RGBColor {
    fn from(value: &Teams) -> Self {
        match value {
            Teams::AremaFC => BLUE,
            Teams::BaliUnitedFC => RED_700,
            Teams::BorneoFCSamarinda => ORANGE,
            Teams::DewaUnitedFC => YELLOW_600,
            Teams::MaduraUnitedFC => RED,
            Teams::MalutUnitedFC => RED_700,
            Teams::PersebayaSurabaya => GREEN_700,
            Teams::PersisSolo => RED_700,
            Teams::PssSleman => GREEN,
            Teams::PsisSemarang => BLUE,
            Teams::PersijaJakarta => RED,
            Teams::PersibBandung => BLUE,
            Teams::PersitaTangerang => PURPLE,
            Teams::PsbsBiak => BLUE_200,
            Teams::PsmMakassar => RED_900,
            Teams::PersikKediri => PURPLE,
            Teams::PSBaritoPutera => YELLOW_500,
            Teams::SemenPadangFC => RED_500,
        }
    }
}

impl From<String> for Teams {
    fn from(value: String) -> Self {
        match value.as_str() {
            "AREMA FC" => Self::AremaFC,
            "Bali United FC" => Self::BaliUnitedFC,
            "Borneo FC Samarinda" => Self::BorneoFCSamarinda,
            "Dewa United FC" => Self::DewaUnitedFC,
            "Madura United FC" => Self::MaduraUnitedFC,
            "Malut United FC" => Self::MalutUnitedFC,
            "PERSEBAYA Surabaya" => Self::PersebayaSurabaya,
            "PERSIS Solo" => Self::PersisSolo,
            "PSS Sleman" => Self::PssSleman,
            "PSIS Semarang" => Self::PsisSemarang,
            "PERSIJA Jakarta" => Self::PersijaJakarta,
            "PERSIB Bandung" => Self::PersibBandung,
            "PERSITA Tangerang" => Self::PersitaTangerang,
            "PSBS Biak" => Self::PsbsBiak,
            "PSM Makassar" => Self::PsmMakassar,
            "PERSIK Kediri" => Self::PersikKediri,
            "PS Barito Putera" => Self::PSBaritoPutera,
            "Semen Padang FC" => Self::SemenPadangFC,
            _ => unreachable!("Invalid Team Name"),
        }
    }
}

impl From<&String> for Teams {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "AREMA FC" => Self::AremaFC,
            "Bali United FC" => Self::BaliUnitedFC,
            "Borneo FC Samarinda" => Self::BorneoFCSamarinda,
            "Dewa United FC" => Self::DewaUnitedFC,
            "Madura United FC" => Self::MaduraUnitedFC,
            "Malut United FC" => Self::MalutUnitedFC,
            "PERSEBAYA Surabaya" => Self::PersebayaSurabaya,
            "PERSIS Solo" => Self::PersisSolo,
            "PSS Sleman" => Self::PssSleman,
            "PSIS Semarang" => Self::PsisSemarang,
            "PERSIJA Jakarta" => Self::PersijaJakarta,
            "PERSIB Bandung" => Self::PersibBandung,
            "PERSITA Tangerang" => Self::PersitaTangerang,
            "PSBS Biak" => Self::PsbsBiak,
            "PSM Makassar" => Self::PsmMakassar,
            "PERSIK Kediri" => Self::PersikKediri,
            "PS Barito Putera" => Self::PSBaritoPutera,
            "Semen Padang FC" => Self::SemenPadangFC,
            _ => unreachable!("Invalid Team Name"),
        }
    }
}

impl From<&str> for Teams {
    fn from(value: &str) -> Self {
        match value {
            "AREMA FC" => Self::AremaFC,
            "Bali United FC" => Self::BaliUnitedFC,
            "Borneo FC Samarinda" => Self::BorneoFCSamarinda,
            "Dewa United FC" => Self::DewaUnitedFC,
            "Madura United FC" => Self::MaduraUnitedFC,
            "Malut United FC" => Self::MalutUnitedFC,
            "PERSEBAYA Surabaya" => Self::PersebayaSurabaya,
            "PERSIS Solo" => Self::PersisSolo,
            "PSS Sleman" => Self::PssSleman,
            "PSIS Semarang" => Self::PsisSemarang,
            "PERSIJA Jakarta" => Self::PersijaJakarta,
            "PERSIB Bandung" => Self::PersibBandung,
            "PERSITA Tangerang" => Self::PersitaTangerang,
            "PSBS Biak" => Self::PsbsBiak,
            "PSM Makassar" => Self::PsmMakassar,
            "PERSIK Kediri" => Self::PersikKediri,
            "PS Barito Putera" => Self::PSBaritoPutera,
            "Semen Padang FC" => Self::SemenPadangFC,
            _ => unreachable!("Invalid Team Name"),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DataSource {
    team: Teams,
    game: String,
    game_week: u32,
    total_ck_for: u32,
    shots_from_ck: u32,
    xg: f64,
}

pub struct Data {
    team: Teams,
    game: String,
    game_week: u32,
    opponent: Teams,
    total_ck_for: u32,
    total_ck_against: u32,
    shots_from_ck: u32,
    shots_against_from_ck: u32,
    xg: f64,
    xg_against: f64,
}

#[derive(Debug)]
pub enum MyError {
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
        write!(f, "{text}")
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

pub fn parse_csv<P: AsRef<Path>>(path: P) -> Result<HashMap<u32, Vec<Data>>> {
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
                    .find(|s| Teams::from(**s) != ds.team)
                    .map(|s| Teams::from(*s))
                    .unwrap();

                let opp_data = data_sources.iter().find(|d| d.team == opponent).unwrap();

                let total_ck_against = opp_data.total_ck_for;
                let shots_against_from_ck = opp_data.shots_from_ck;
                let xg_against = opp_data.xg;

                Data {
                    team: ds.team,
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

#[derive(Debug, Clone)]
pub struct TeamData {
    team_name: Teams,
    total_ck_for: u32,
    total_ck_against: u32,
    shots_from_ck: u32,
    shots_against_from_ck: u32,
    xg: f64,
    xg_against: f64,
}

pub fn accumulate(input: HashMap<u32, Vec<Data>>) -> Vec<TeamData> {
    let mut team_names: Vec<Teams> = Vec::new();
    if let Some(vd) = input.get(&0) {
        vd.iter().for_each(|d| team_names.push(d.team))
    }

    let mut accumulated_data: Vec<TeamData> = Vec::new();

    for team_name in team_names {
        let team_data_iter = input
            .values()
            .map(|vd| vd.iter().find(|d| d.team == team_name).unwrap())
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

#[derive(Debug, Clone)]
pub struct DataFrame {
    team_name: Vec<Teams>,
    total_ck_for: Vec<u32>,
    total_ck_against: Vec<u32>,
    shots_from_ck: Vec<u32>,
    shots_against_from_ck: Vec<u32>,
    xg: Vec<f64>,
    xg_against: Vec<f64>,
}

pub fn create_dataframe(data: Vec<TeamData>) -> DataFrame {
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
