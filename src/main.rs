use anyhow::{self};
use clap::Parser;
use rcli::{
    opts,
    process::{csv_convert, gen_pass},
};

fn main() -> anyhow::Result<()> {
    let opts = opts::Opts::parse();
    println!("{:?}", opts);
    handle_opts(opts)
}

fn handle_opts(opts: opts::Opts) -> anyhow::Result<()> {
    match opts.cmd {
        opts::SubCommand::Csv(csv_opts) => {
            let output = if let Some(output) = csv_opts.output {
                output.clone()
            } else {
                format!("output.{}", csv_opts.format)
            };

            csv_convert::process_csv(&csv_opts.input, output, csv_opts.format)
        }
        opts::SubCommand::GenPass(opts) => {
            println!("generate pwd opts: {:?}", opts);
            gen_pass::generate_password(opts)
        }
    }
}
