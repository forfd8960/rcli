use anyhow::{self};
use clap::Parser;
use rcli::{
    cli::{base64::Base64SubCommand, opts},
    process::{self, csv_convert, gen_pass},
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
        opts::SubCommand::Base64(sub_cmd) => match sub_cmd {
            Base64SubCommand::Encode(encode_opts) => process::base64::encode(encode_opts.input),
            Base64SubCommand::Decode(decode_opts) => process::base64::decode(decode_opts.input),
        },
    }
}
