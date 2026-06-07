use std::{fs::File, io::Read};

use base64::{
    Engine,
    prelude::{BASE64_STANDARD, BASE64_URL_SAFE},
};

use crate::Base64Format;

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let data = read_data(input)?;

    match format {
        Base64Format::Standard => {
            println!("{}", BASE64_STANDARD.encode(data));
        }
        Base64Format::UrlSafe => {
            println!("{}", BASE64_URL_SAFE.encode(data));
        }
    }
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let data = read_data(input)?;

    let decode = match format {
        Base64Format::Standard => BASE64_STANDARD.decode(data)?,
        Base64Format::UrlSafe => BASE64_URL_SAFE.decode(data)?,
    };
    println!("{}", String::from_utf8_lossy(&decode));
    Ok(())
}

pub fn read_data(input: &str) -> anyhow::Result<Vec<u8>> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}
