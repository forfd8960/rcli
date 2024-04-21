use std::fs;

use anyhow::{self};
use clap::Parser;
use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

// rcli csv -i input.csv -o output.json --header -d ','
#[derive(Debug, Parser)]
#[command(name="rcli", version, author, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "convert csv to json")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, value_parser = verify_input)]
    input: String,
    #[arg(short, long, default_value = "output.json")]
    output: String,
    #[arg(short, long, default_value_t = ',')]
    delimiter: char,
    #[arg(long, default_value_t = true)]
    header: bool,
}

// clap value validation
fn verify_input(f: &str) -> std::result::Result<String, String> {
    if std::path::Path::new(f).exists() {
        Ok(f.into())
    } else {
        Err("File is not exists".into())
    }
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    println!("{:?}", opts);
    handle_opts(opts)
}

fn handle_opts(opts: Opts) -> anyhow::Result<()> {
    match opts.cmd {
        SubCommand::Csv(csv_opts) => {
            let mut reader = Reader::from_path(csv_opts.input)?;
            let mut players = Vec::with_capacity(128);
            for record in reader.deserialize() {
                let r: Player = record?;
                players.push(r);
                // println!("{:?}", r);
            }

            let player_data = serde_json::to_string_pretty(&players)?;
            fs::write(csv_opts.output, player_data)?;

            anyhow::Ok(())
        }
    }
}
