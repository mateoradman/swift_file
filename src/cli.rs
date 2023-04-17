use crate::network::port_in_range;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Send a file
    Send {
        /// File to send
        file: PathBuf,
        #[arg(short, long, value_parser = port_in_range)]
        port: Option<u16>,
    },
    /// Receive a file
    Receive {
        #[arg(short, long, value_parser = port_in_range)]
        port: Option<u16>,
    },
}
