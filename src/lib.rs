use std::io;

use plotters::style::{
    full_palette::{
        BLUE, BLUE_200, GREEN, GREEN_700, ORANGE, PURPLE, RED, RED_500, RED_700, RED_900,
        YELLOW_500, YELLOW_600,
    },
    RGBColor,
};
use serde::Deserialize;

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
