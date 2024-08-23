use clap::Parser;
use zxcvbn::zxcvbn;

use crate::{process_genpass, CmdExector};

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub numbers: bool,

    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}

impl CmdExector for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let ret = process_genpass(
            self.length,
            self.uppercase,
            self.lowercase,
            self.numbers,
            self.symbol,
        )?;
        println!("{}", ret);

        let estimate = zxcvbn(&ret, &[]);
        eprint!("strength {}", estimate.score());

        Ok(())
    }
}
