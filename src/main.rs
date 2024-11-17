use tsg2425::{accumulate, create_dataframe, parse_csv, plot_cka, plot_ckd, Result};

fn main() -> Result<()> {
    let path = "dataset/xg_corner.csv";
    let parsed_data = parse_csv(path)?;

    let accumulated_data = accumulate(parsed_data);
    let df = create_dataframe(accumulated_data);

    plot_ckd(&df)?;
    plot_cka(&df)?;

    Ok(())
}
