use std::path::PathBuf;

use clap::Parser;

use super::verify_output;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(name = "serve", about = "serve a directory over http")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, value_parser = verify_output, default_value=".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}
