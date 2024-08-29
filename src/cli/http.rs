use std::path::PathBuf;

use crate::{process_http_server, CmdExector};

use super::verify_path;
use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum HttpSubCommand {
    #[command(about = "Server a directory over HTTP")]
    Server(HttpServerOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServerOpts {
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,

    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

impl CmdExector for HttpServerOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_http_server(self.dir, self.port).await
    }
}
