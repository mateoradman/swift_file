#![forbid(unsafe_code)]

mod cli;
mod config;
mod files;
mod network;
mod qr;
mod receive;
mod send;
mod server;

use crate::{cli::Cli, config::GlobalConfig, server::start_server};
use clap::Parser;

#[tokio::main]
async fn main() {
    let cli_args = Cli::parse();
    let config = cli_args.into_config();
    start_server(config).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
