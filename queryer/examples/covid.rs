use anyhow::Result;
use polars::prelude::*;
use std::io::Cursor;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let url = "https://datahub.io/core/pharmaceutical-drug-spending/_r/-/data/data.csv";
    let data = reqwest::get(url).await?.text().await?;

    let df = CsvReader::new(Cursor::new(data))
            .finish()?;

    let filtered = df.filter(&df["PC_HEALTHXP

"].gt(13).unwrap())?;
    println!(
        "{:?}",
        filtered.select([
            "LOCATION",
            "TIME",
            "PC_HEALTHXP",
            "PC_GDP",
            "USD_CAP",
            "FLAG_CODES",
            "TOTAL_SPEND"
        ])
    );
    Ok(())
}