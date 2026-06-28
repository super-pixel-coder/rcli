use base64::{
    Engine,
    prelude::{BASE64_STANDARD, BASE64_URL_SAFE},
};

use crate::{Base64Format, get_reader};

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    match format {
        Base64Format::Standard => Ok(BASE64_STANDARD.encode(buf)),
        Base64Format::UrlSafe => Ok(BASE64_URL_SAFE.encode(buf)),
    }
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    match format {
        Base64Format::Standard => Ok(BASE64_STANDARD.decode(buf)?),
        Base64Format::UrlSafe => Ok(BASE64_URL_SAFE.decode(buf)?),
    }
}
