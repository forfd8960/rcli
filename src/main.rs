use anyhow::{self};
use base64::{engine::general_purpose, Engine};
use clap::Parser;
use rcli::{
    cli::{base64::Base64SubCommand, opts},
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
        opts::SubCommand::Base64(sub_cmd) => match sub_cmd {
            Base64SubCommand::Encode(encode_opts) => {
                let encode = general_purpose::STANDARD.encode(encode_opts.input);
                println!("encode: {:?}", encode);
                anyhow::Ok(())
            }
            Base64SubCommand::Decode(decode_opts) => {
                let decode = general_purpose::STANDARD.decode(decode_opts.input);
                println!("decode: {:?}", decode);
                anyhow::Ok(())
            }
        },
    }
}
