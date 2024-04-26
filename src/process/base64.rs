use std::io::Read;

use base64::{engine::general_purpose, prelude::Engine};

use crate::{cli::base64::Base64Format, utils};

pub fn encode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let mut reader = utils::get_reader(input)?;

    let mut buf = Vec::new();
    let _ = reader.read_to_end(&mut buf)?;

    let encode = match format {
        Base64Format::Standard => general_purpose::STANDARD.encode(buf),
        Base64Format::UrlSafe => general_purpose::URL_SAFE_NO_PAD.encode(buf),
    };

    anyhow::Ok(encode)
}

pub fn decode(input: &str, format: Base64Format) -> anyhow::Result<Vec<u8>> {
    let mut reader = utils::get_reader(input)?;

    let mut buf = String::new();
    let _ = reader.read_to_string(&mut buf)?;
    let buf = buf.trim();

    let decode = match format {
        Base64Format::Standard => general_purpose::STANDARD.decode(buf)?,
        Base64Format::UrlSafe => general_purpose::URL_SAFE_NO_PAD.decode(buf)?,
    };

    anyhow::Ok(decode)
}
