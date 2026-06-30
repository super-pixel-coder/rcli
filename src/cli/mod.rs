mod base64;
mod csv;
mod genpass;
mod http;
mod text;

pub use self::base64::{Base64Format, Base64SubCommand};
use self::csv::CsvOpts;
pub use self::csv::OutputFormat;
pub use crate::cli::text::{TextSignFormat, TextSignOpts, TextSubCommand, TextVerifyOpts};
use clap::Parser;
use genpass::GenPassOpts;
pub use http::{HttpServeOpts, HttpSubCommand};
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV or Convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
    #[command(subcommand)]
    Text(TextSubCommand),
    #[command(subcommand)]
    Http(HttpSubCommand),
}

fn verify_file(filename: &str) -> anyhow::Result<String> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err(anyhow::anyhow!("File does not exists"))
    }
}

fn verify_path(filepath: &str) -> anyhow::Result<PathBuf> {
    let p = Path::new(filepath);
    if p.exists() && p.is_dir() {
        Ok(p.into())
    } else {
        Err(anyhow::anyhow!("Path: {} does not exist", filepath))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-").unwrap(), Into::<String>::into("-"));
        assert_eq!(
            verify_file("*").unwrap_err().to_string(),
            "File does not exists".to_owned()
        );
        assert_eq!(verify_file("Cargo.toml").unwrap(), "Cargo.toml".to_owned());
        assert_eq!(
            verify_file("file-not-exists").unwrap_err().to_string(),
            "File does not exists".to_owned()
        );
    }
}
