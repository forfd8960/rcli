use clap::Parser;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long)]
    pub input: String,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long)]
    pub input: String,
}
