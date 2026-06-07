mod base64;
mod csv;
mod genpass;

pub use self::base64::{Base64Format, Base64SubCommand};
use self::csv::CsvOpts;
pub use self::csv::OutputFormat;
use clap::Parser;
use genpass::GenPassOpts;
use std::path::Path;

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
}

fn verify_input_file(filename: &str) -> anyhow::Result<String> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err(anyhow::anyhow!("File does not exists"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-").unwrap(), Into::<String>::into("-"));
        assert_eq!(
            verify_input_file("*").unwrap_err().to_string(),
            "File does not exists".to_owned()
        );
        assert_eq!(
            verify_input_file("Cargo.toml").unwrap(),
            "Cargo.toml".to_owned()
        );
        assert_eq!(
            verify_input_file("file-not-exists")
                .unwrap_err()
                .to_string(),
            "File does not exists".to_owned()
        );
    }
}
