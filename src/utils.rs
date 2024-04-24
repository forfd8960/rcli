use anyhow;
use std::io::Read;

pub fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>, anyhow::Error> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin()) // use box::new convert to trait object
    } else {
        Box::new(std::fs::File::open(input)?)
    };

    anyhow::Ok(reader)
}
