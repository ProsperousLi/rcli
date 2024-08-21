// rcli csv -i input.csv -o output.json --header -d ','

use std::fs;

use clap::Parser;
use template::{
    process_csv, process_decode, process_encode, process_genpass, process_http_server,
    process_text_generate, process_text_sign, process_text_verify, Base64SubCommand,
    HttpSubCommand, Opts, SubCommand, TextSubCommand,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
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
            let pass = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.numbers,
                opts.symbol,
            )?;
            println!("{}", pass);
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                let encode = process_encode(&opts.input, opts.format)?;
                println!("{}", encode)
            }
            Base64SubCommand::Decode(opts) => {
                let decode = process_decode(&opts.input, opts.format)?;
                println!("{:?}", String::from_utf8(decode))
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                let sign = process_text_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", sign)
            }
            TextSubCommand::Verify(opts) => {
                let result = process_text_verify(&opts.input, &opts.key, opts.format, &opts.sig)?;
                println!("{}", result)
            }
            TextSubCommand::Generate(opts) => {
                let key = process_text_generate(opts.format)?;
                match opts.format {
                    template::TextSignFormat::Blake3 => {
                        let name = opts.output.join("blake3.txt");
                        fs::write(name, &key[0])?;
                    }
                    template::TextSignFormat::Ed25519 => {
                        let name = &opts.output;
                        fs::write(name.join("ed25519.sk"), &key[0])?;
                        fs::write(name.join("ed25519.pk"), &key[1])?;
                    }
                }
            }
        },
        SubCommand::Http(cmd) => match cmd {
            HttpSubCommand::Server(opts) => process_http_server(opts.dir, opts.port).await?,
        },
    }

    Ok(())
}
