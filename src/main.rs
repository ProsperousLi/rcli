// rcli csv -i input.csv -o output.json --header -d ','

use clap::Parser;
use template::{CmdExector, Opts};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts = Opts::parse();
    opts.cmd.execute().await
}
