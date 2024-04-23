use std::fs;

use csv::Reader;
use serde_json::Value;
use serde_yaml;

use crate::cli::csv::OutputFormat;

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut data = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    println!("{:?}", headers);

    for record in reader.records() {
        let r = record?;
        let value = headers.iter().zip(r.iter()).collect::<Value>();
        data.push(value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&data)?,
        OutputFormat::Yaml => serde_yaml::to_string(&data)?,
    };

    fs::write(output, content)?;

    anyhow::Ok(())
}
