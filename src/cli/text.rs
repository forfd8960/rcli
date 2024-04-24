use anyhow::anyhow;
use clap::Parser;
use std::fmt;
use std::str::FromStr;

use super::verify_input;

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(name = "sign", about = "")]
    Sign(TextSignOpts),
    #[command(name = "verify", about = "")]
    Verify(TextVerifyOpts),
    #[command(name = "generate-key", about = "")]
    GenerateKey,
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    // value_parser = verify_input, default_value="-"
    #[arg(short, long, value_parser = verify_input, default_value="-")]
    pub input: String,
    #[arg(long, value_parser = verify_input)]
    pub key: String,
    #[arg(long, value_parser=parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    // value_parser = verify_input, default_value="-"
    #[arg(short, long, value_parser = verify_input, default_value="-")]
    pub input: String,
    #[arg(long, value_parser = verify_input)]
    pub key: String,
    #[arg(long, value_parser=parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
    #[arg(long)]
    pub sig: String,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_format(format: &str) -> std::result::Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl From<TextSignFormat> for &'static str {
    fn from(value: TextSignFormat) -> Self {
        match value {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow!("Not supported text sign format")),
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
