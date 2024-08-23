mod cli;
mod process;
mod utils;

pub use cli::{
    Base64Format, Base64SubCommand, HttpSubCommand, Opts, SubCommand, TextSignFormat,
    TextSubCommand,
};
pub use process::*;
pub use utils::*;

use anyhow::Result;

impl CmdExector for SubCommand {
    async fn execute(self) -> Result<()> {
        match self {
            SubCommand::Base64(opts) => opts.execute().await,
            SubCommand::Csv(opts) => opts.execute().await,
            SubCommand::GenPass(opts) => opts.execute().await,
            SubCommand::Http(opts) => opts.execute().await,
            SubCommand::Text(opts) => opts.execute().await,
        }
    }
}

#[allow(async_fn_in_trait)]
pub trait CmdExector {
    async fn execute(self) -> Result<()>;
}
