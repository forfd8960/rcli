use rand::seq::SliceRandom;

use crate::opts::GenPassOpts;

pub fn generate_password(opts: GenPassOpts) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = String::new();
    let mut chars = Vec::new();

    if opts.uppercase {
        chars.extend_from_slice(b"ABCDEFGHJKLMNOPQRSTUVWXYZ");
    }

    if opts.lowercase {
        chars.extend_from_slice(b"abcdefghijkmnopqrstuvwxyz");
    }
    if opts.number {
        chars.extend_from_slice(b"123456789");
    }

    if opts.symbol {
        chars.extend_from_slice(b"!@#$%^&*_");
    }

    for _ in 0..opts.length {
        let c = chars.choose(&mut rng).expect("char is not empty");
        password.push(*c as char);
    }

    println!("{}", password);
    anyhow::Ok(())
}
