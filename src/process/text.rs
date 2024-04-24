use std::{fs, io::Read};

use base64::{engine::general_purpose, Engine};

use crate::{
    cli::text::{TextSignFormat, TextSignOpts, TextVerifyOpts},
    utils,
};

trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>>;
}

trait TextVerify {
    fn verify(&self, reader: impl Read, sig: &[u8]) -> anyhow::Result<bool>;
}

pub struct Blake3 {
    key: [u8; 32],
}

// pub struct ED25519Signer {
//     key: [u8; 32],
// }

// pub struct ED25519Verify {
//     key: [u8; 32],
// }

pub fn process_sign(sign_opts: TextSignOpts) -> anyhow::Result<()> {
    let mut reader = utils::get_reader(&sign_opts.input)?;
    let mut buf = Vec::new();
    let _ = reader.read_to_end(&mut buf);

    let sign = match sign_opts.format {
        TextSignFormat::Blake3 => {
            let key = fs::read(sign_opts.key)?;
            let key = &key[..32];
            let key = key.try_into()?;

            let signer = Blake3 { key };
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            todo!()
        }
    };

    let signed = general_purpose::URL_SAFE_NO_PAD.encode(sign);
    println!("{:?}", signed);
    anyhow::Ok(())
}

pub fn process_verify(_verify_opts: TextVerifyOpts) -> anyhow::Result<()> {
    anyhow::Ok(())
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        let _ = reader.read_to_end(&mut buf);

        anyhow::Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        let _ = reader.read_to_end(&mut buf);

        let hash = blake3::hash(&buf);
        let hash = hash.as_bytes();
        anyhow::Ok(hash == sig)
    }
}
