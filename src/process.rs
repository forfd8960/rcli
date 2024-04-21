use std::fs;

use serde::{Deserialize, Serialize};

use csv::Reader;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut players = Vec::with_capacity(128);
    for record in reader.deserialize() {
        let r: Player = record?;
        players.push(r);
        // println!("{:?}", r);
    }

    let player_data = serde_json::to_string_pretty(&players)?;
    fs::write(output, player_data)?;

    anyhow::Ok(())
}
