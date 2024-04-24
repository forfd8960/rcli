use std::io::Read;

use base64::{engine::general_purpose, prelude::Engine};

use crate::cli::base64::Base64Format;

pub fn encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;

    let mut buf = Vec::new();
    let _ = reader.read_to_end(&mut buf)?;

    let encode = match format {
        Base64Format::Standard => general_purpose::STANDARD.encode(buf),
        Base64Format::UrlSafe => general_purpose::URL_SAFE_NO_PAD.encode(buf),
    };

    println!("{}", encode);
    anyhow::Ok(())
}

pub fn decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;

    let mut buf = String::new();
    let _ = reader.read_to_string(&mut buf)?;
    let buf = buf.trim();

    let decode = match format {
        Base64Format::Standard => general_purpose::STANDARD.decode(buf)?,
        Base64Format::UrlSafe => general_purpose::URL_SAFE_NO_PAD.decode(buf)?,
    };

    let decode_data = String::from_utf8(decode)?;
    println!("{}", decode_data);
    anyhow::Ok(())
}

fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>, anyhow::Error> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin()) // use box::new convert to trait object
    } else {
        Box::new(std::fs::File::open(input)?)
    };

    anyhow::Ok(reader)
}
