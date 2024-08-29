use anyhow::Ok;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::{fmt, path::PathBuf, str::FromStr};
use tokio::fs;

use crate::{
    get_content, get_reader, process_text_generate, process_text_sign, process_text_verify,
    CmdExector,
};

use super::{verify_file, verify_path};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum TextSubCommand {
    #[command(name = "sign", about = "sign a message with a private key")]
    Sign(TextSignOpts),
    #[command(name = "verify", about = "verify a message with a public key")]
    Verify(TextVerifyOpts),
    #[command(name = "generate", about = "generate a key pair")]
    Generate(TextKeyGenerateOpts),
}
#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(short, long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path, default_value = "blake3.txt")]
    pub output: PathBuf,
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
    #[arg(short, long)]
    pub sig: String,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl CmdExector for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;

        let key = get_content(&self.key)?;

        let sig = process_text_sign(&mut reader, &key, self.format)?;

        let encoded = URL_SAFE_NO_PAD.encode(sig);
        println!("{}", encoded);
        Ok(())
    }
}

impl CmdExector for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;

        let key = get_content(&self.key)?;

        let decoded = URL_SAFE_NO_PAD.decode(self.sig)?;

        let verified = process_text_verify(&mut reader, &key, &decoded, self.format)?;

        if verified {
            println!("Verify success");
        } else {
            println!("Verify failed");
        }

        Ok(())
    }
}

impl CmdExector for TextKeyGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = process_text_generate(self.format)?;
        for (k, v) in key {
            fs::write(self.output.join(k), v).await?;
        }

        Ok(())
    }
}
