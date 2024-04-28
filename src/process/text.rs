use std::{fs, io::Read, path::Path};

use base64::{
    engine::general_purpose::{self, URL_SAFE_NO_PAD},
    Engine,
};
use ed25519_dalek::{ed25519::signature::Signer, Signature, SigningKey, VerifyingKey};
use rand::rngs::OsRng;

use crate::{
    cli::genpass,
    cli::text::{TextSignFormat, TextSignOpts, TextVerifyOpts},
    utils,
};

use super::gen_pass::generate_password;

trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self>
    where
        Self: Sized;
}

trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>>;
}

trait TextVerify {
    fn verify(&self, reader: impl Read, sig: &[u8]) -> anyhow::Result<bool>;
}

pub trait KeyGenerator {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>>;
}

pub struct Blake3 {
    key: [u8; 32],
}

impl Blake3 {
    fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    #[allow(dead_code)]
    fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        anyhow::Ok(Blake3::new(key))
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        let key = &key[..32];
        let key = key.try_into()?;
        anyhow::Ok(Blake3::new(key))
    }
}

pub struct ED25519Signer {
    key: SigningKey,
}

impl ED25519Signer {
    fn new(key: SigningKey) -> Self {
        ED25519Signer { key }
    }

    fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let sec_bytes = key.try_into()?;
        let key: SigningKey = SigningKey::from_bytes(sec_bytes);
        anyhow::Ok(ED25519Signer::new(key))
    }
}

impl KeyLoader for ED25519Signer {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        ED25519Signer::try_new(&key)
    }
}

pub struct ED25519Verify {
    key: VerifyingKey,
}

impl ED25519Verify {
    fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
        anyhow::Ok(ED25519Verify::new(key))
    }
}

impl KeyLoader for ED25519Verify {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        ED25519Verify::try_new(&key)
    }
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

        let hash = blake3::keyed_hash(&self.key, &buf);
        let hash = hash.as_bytes();
        anyhow::Ok(hash == sig)
    }
}

impl TextSign for ED25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        let _ = reader.read_to_end(&mut buf);

        let sig = self.key.sign(&buf);
        anyhow::Ok(sig.to_bytes().to_vec())
    }
}
impl TextVerify for ED25519Verify {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        let _ = reader.read_to_end(&mut buf);

        let signature = Signature::from_bytes(sig.try_into()?);
        let verify_result = self.key.verify_strict(&buf, &signature).is_ok();
        anyhow::Ok(verify_result)
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>> {
        let key = generate_password(genpass::GenPassOpts {
            length: 32,
            uppercase: true,
            lowercase: true,
            number: true,
            symbol: true,
        })?;
        anyhow::Ok(vec![key.into_bytes()])
    }
}

impl KeyGenerator for ED25519Signer {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);

        let sk = signing_key.to_bytes().to_vec();
        let vk = signing_key.verifying_key();

        anyhow::Ok(vec![sk, vk.to_bytes().to_vec()])
    }
}

pub fn process_sign(sign_opts: TextSignOpts) -> anyhow::Result<String> {
    let mut reader = utils::get_reader(&sign_opts.input)?;
    let mut buf = Vec::new();
    let _ = reader.read_to_end(&mut buf);

    let sign = match sign_opts.format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(&sign_opts.key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let signer = ED25519Signer::load(&sign_opts.key)?;
            signer.sign(&mut reader)?
        }
    };

    let signed = general_purpose::URL_SAFE_NO_PAD.encode(sign);
    anyhow::Ok(signed)
}

pub fn process_verify(verify_opts: TextVerifyOpts) -> anyhow::Result<bool> {
    let mut reader = utils::get_reader(&verify_opts.input)?;
    let sig = URL_SAFE_NO_PAD.decode(verify_opts.sig)?;

    let verified = match verify_opts.format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(verify_opts.key)?;
            verifier.verify(&mut reader, &sig)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = ED25519Verify::load(verify_opts.key)?;
            verifier.verify(&mut reader, &sig)?
        }
    };

    anyhow::Ok(verified)
}

pub fn process_generate(format: TextSignFormat) -> anyhow::Result<Vec<Vec<u8>>> {
    let keys = match format {
        TextSignFormat::Blake3 => Blake3::generate()?,
        TextSignFormat::Ed25519 => ED25519Signer::generate()?,
    };

    anyhow::Ok(keys)
}
