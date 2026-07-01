use crate::{CmdExecutor, process_decode, process_encode};

use super::verify_file;
use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Decode a base64 to string")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = parse_format, default_value = "standard")]
    pub format: Base64Format,
}

impl CmdExecutor for Base64EncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let res = process_encode(&self.input, self.format)?;
        println!("{}", res);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = parse_format, default_value = "standard")]
    pub format: Base64Format,
}

impl CmdExecutor for Base64DecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let res = String::from_utf8(process_decode(&self.input, self.format)?)?;
        println!("{}", res);
        Ok(())
    }
}

#[derive(Debug, Parser, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

impl From<Base64Format> for &str {
    fn from(value: Base64Format) -> Self {
        match value {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "url_safe",
        }
    }
}

impl TryFrom<&str> for Base64Format {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "standard" => Ok(Base64Format::Standard),
            "url_safe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("无法将 {} 转译为 Base64Format 格式", value)),
        }
    }
}

impl std::fmt::Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

fn parse_format(value: &str) -> anyhow::Result<Base64Format> {
    value.try_into()
}
