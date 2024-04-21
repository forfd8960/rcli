use anyhow::{self};
use clap::Parser;
use rcli::{opts, process};

fn main() -> anyhow::Result<()> {
    let opts = opts::Opts::parse();
    println!("{:?}", opts);
    handle_opts(opts)
}

fn handle_opts(opts: opts::Opts) -> anyhow::Result<()> {
    match opts.cmd {
        opts::SubCommand::Csv(csv_opts) => process::process_csv(&csv_opts.input, &csv_opts.output),
    }
}
