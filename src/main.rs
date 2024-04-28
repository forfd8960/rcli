use std::fs;

use anyhow::{self};
use clap::Parser;
use rcli::{
    cli::{
        base64::Base64SubCommand,
        opts,
        text::{TextSignFormat, TextSubCommand},
    },
    process::{self, csv_convert, gen_pass, text},
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
            let pwd = gen_pass::generate_password(opts)?;
            println!("{}", pwd);
            anyhow::Ok(())
        }
        opts::SubCommand::Base64(sub_cmd) => match sub_cmd {
            Base64SubCommand::Encode(encode_opts) => {
                let encoded = process::base64::encode(&encode_opts.input, encode_opts.format)?;
                println!("{:?}", encoded);
                anyhow::Ok(())
            }
            Base64SubCommand::Decode(decode_opts) => {
                let decoded = process::base64::decode(&decode_opts.input, decode_opts.format)?;
                let decode_data: String = String::from_utf8(decoded)?;
                println!("{}", decode_data);
                anyhow::Ok(())
            }
        },
        opts::SubCommand::Text(sub_cmd) => {
            println!("opts: {:?}", sub_cmd);
            match sub_cmd {
                TextSubCommand::Sign(opts) => {
                    let signed = text::process_sign(opts)?;
                    println!("{:?}", signed);
                }
                TextSubCommand::Verify(opts) => {
                    let verified = text::process_verify(opts)?;
                    println!("{}", verified);
                }
                TextSubCommand::GenerateKey(opts) => {
                    println!("{:?}", opts);
                    let keys = text::process_generate(opts.format)?;
                    match opts.format {
                        TextSignFormat::Blake3 => {
                            let name = opts.output.join("blake3.txt");
                            let _ = fs::write(name, &keys[0]);
                        }
                        TextSignFormat::Ed25519 => {
                            let name = &opts.output;
                            let _ = fs::write(name.join("ed25519.sk"), &keys[0]);
                            let _ = fs::write(name.join("ed25519.pk"), &keys[1]);
                        }
                    }
                    println!("generate keys done~~~");
                }
            }
            anyhow::Ok(())
        }
    }
}
