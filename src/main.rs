use tsg2425::{accumulate, create_dataframe, draw_plotters, parse_csv, Result};

fn main() -> Result<()> {
    let path = "dataset/xg_corner.csv";
    let parsed_data = parse_csv(path)?;

    let accumulated_data = accumulate(parsed_data);
    let df = create_dataframe(accumulated_data);

    draw_plotters(&df)?;

    Ok(())
}
