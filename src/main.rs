use clap::Parser;
use rcli::{
    Base64SubCommand, Opts, SubCommand, TextSubCommand, process_csv, process_decode,
    process_encode, process_genpass, process_text_generate, process_text_sign, process_text_verify,
};
use zxcvbn::zxcvbn;

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            let password = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            println!("{}", password);
            let estimate = zxcvbn(&password, &[]);
            eprintln!("Password strength: {}", estimate.score());
        }
        SubCommand::Base64(cmd) => match cmd {
            Base64SubCommand::Encode(opts) => {
                let res = process_encode(&opts.input, opts.format)?;
                println!("{}", res);
            }
            Base64SubCommand::Decode(opts) => {
                let res = String::from_utf8(process_decode(&opts.input, opts.format)?)?;
                println!("{}", res);
            }
        },
        SubCommand::Text(cmd) => match cmd {
            TextSubCommand::Sign(opts) => {
                let signed = process_text_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", signed);
            }
            TextSubCommand::Verify(opts) => {
                let verified = process_text_verify(&opts.input, &opts.key, opts.format, &opts.sig)?;
                println!("{}", verified);
            }
            TextSubCommand::Generate(opts) => {
                let key = process_text_generate(opts.format)?;
                match opts.format {
                    rcli::TextSignFormat::Blake3 => {
                        let name = opts.output.join("blake3.txt");
                        std::fs::write(name, &key[0])?;
                    }
                    rcli::TextSignFormat::Ed25519 => {
                        let name = &opts.output;
                        std::fs::write(name.join("ed25519.sk"), &key[0])?;
                        std::fs::write(name.join("ed25519.pk"), &key[1])?;
                    }
                }
            }
        },
    }
    Ok(())
}
