use std::fs;

use csv::Reader;
use serde_json::Value;

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut data = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    println!("{:?}", headers);

    for record in reader.records() {
        let r = record?;
        let json_value = headers.iter().zip(r.iter()).collect::<Value>();
        data.push(json_value);
    }

    let player_data = serde_json::to_string_pretty(&data)?;
    fs::write(output, player_data)?;

    anyhow::Ok(())
}
