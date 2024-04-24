use std::io::Read;

use base64::{engine::general_purpose, prelude::Engine};

use crate::cli::base64::Base64Format;

pub fn encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let buf = read_input(input)?;

    let encode = match format {
        Base64Format::Standard => general_purpose::STANDARD.encode(buf),
        Base64Format::UrlSafe => general_purpose::URL_SAFE_NO_PAD.encode(buf),
    };

    println!("encode: {}", encode);
    anyhow::Ok(())
}

pub fn decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let buf = read_input(input)?;
    let buf = buf.trim();

    let decode = match format {
        Base64Format::Standard => general_purpose::STANDARD.decode(buf)?,
        Base64Format::UrlSafe => general_purpose::URL_SAFE_NO_PAD.decode(buf)?,
    };

    println!("decode: {:?}", decode);

    let decode_data = String::from_utf8(decode)?;
    println!("decode_data: {}", decode_data);
    anyhow::Ok(())
}

fn read_input(input: &str) -> anyhow::Result<String, anyhow::Error> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin()) // use box::new convert to trait object
    } else {
        Box::new(std::fs::File::open(input)?)
    };

    let mut buf = String::new();
    let _ = reader.read_to_string(&mut buf)?;

    anyhow::Ok(buf)
}
