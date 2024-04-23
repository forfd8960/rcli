use base64::{engine::general_purpose, prelude::Engine};

pub fn encode(input: String) -> anyhow::Result<()> {
    let encode = general_purpose::URL_SAFE.encode(input);
    println!("encode: {:?}", encode);
    anyhow::Ok(())
}

pub fn decode(input: String) -> anyhow::Result<()> {
    let decode = general_purpose::URL_SAFE.decode(input)?;
    let decode_data = String::from_utf8(decode)?;
    println!("decode: {}", decode_data);
    anyhow::Ok(())
}
