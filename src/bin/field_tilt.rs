use std::path::Path;

use tsg2425::Result;

fn parse_csv<P: AsRef<Path>>(path: P) -> Result<()> {
    let file = std::fs::File::open(path)?;
    let mut reader = csv::Reader::from_reader(file);

    let header = reader.headers()?;
    println!("{:?}", header);

    for rc in reader.records() {
        let record = rc?;
        println!("{:?}", record);
    }

    Ok(())
}

fn main() -> Result<()> {
    parse_csv("dataset/touchfinal3rd.csv")?;
    Ok(())
}
