use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;

use crate::cli::genpass::GenPassOpts;

const UPPERCHARS: &[u8] = b"ABCDEFGHJKLMNOPQRSTUVWXYZ";
const LOWERCHARS: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUM: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn generate_password(opts: GenPassOpts) -> anyhow::Result<String> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if opts.uppercase {
        chars.extend_from_slice(UPPERCHARS);
        password.push(*UPPERCHARS.choose(&mut rng).expect("not empty"));
    }

    if opts.lowercase {
        chars.extend_from_slice(LOWERCHARS);
        password.push(*LOWERCHARS.choose(&mut rng).expect("not empty"));
    }
    if opts.number {
        chars.extend_from_slice(NUM);
        password.push(*NUM.choose(&mut rng).expect("not empty"));
    }

    if opts.symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("not empty"));
    }

    for _ in 0..(opts.length as usize - password.len()) {
        let c = chars.choose(&mut rng).expect("char is not empty");
        password.push(*c);
    }

    password.shuffle(&mut rng);
    let final_pwd = String::from_utf8(password)?;

    let estimate = zxcvbn(&final_pwd, &[]).unwrap();
    eprintln!("score: {:?}", estimate.score());
    anyhow::Ok(final_pwd)
}
