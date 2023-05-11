use crate::{
    network::{find_available_port, port_in_range, LOCALHOST},
    qr::generate_qr_code,
    AppState,
};
use clap::{Parser, Subcommand};
use std::{path::PathBuf, process::exit};
use uuid::Uuid;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// IP Address to bind to
    #[arg(short, long, default_value_t = LOCALHOST.to_string(), global = true)]
    addr: String,
    /// Network interface to use
    #[arg(short, long, global = true)]
    interface: Option<String>,
    #[arg(short, long, value_parser = port_in_range, global = true)]
    /// Port to bind the server to (allowed user port range 1024 to 49151)
    port: Option<u16>,
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn decide(&self, server_port: &mut u16, shared_state: &mut AppState) {
        let route = match &self.command {
            Commands::Send { file } => self.send(shared_state, file.clone()),
            Commands::Receive { dest_dir, no_open } => {
                self.receive(shared_state, dest_dir, no_open)
            }
        };
        find_available_port(server_port, &self.port);
        generate_qr_code(server_port, &route);
    }

    fn send(&self, shared_state: &AppState, file: PathBuf) -> String {
        let uuid = Uuid::new_v4().to_string();
        shared_state
            .uuid_path_map
            .lock()
            .expect("shared state was poisoned")
            .insert(uuid.clone(), file);
        format!("/download/{uuid}")
    }

    fn receive(
        &self,
        shared_state: &mut AppState,
        dest_dir: &Option<PathBuf>,
        no_open: &bool,
    ) -> String {
        if let Some(path) = dest_dir {
            if !path.is_dir() {
                eprintln!(
                    "Destination directory must exist but {} does not exist",
                    path.display()
                );
                exit(1);
            }
            shared_state.destination_dir = dest_dir.clone();
        }
        shared_state.auto_open = !no_open;
        String::from("/receive")
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Send a file
    Send {
        /// File path to send
        file: PathBuf,
    },

    /// Receive a file
    Receive {
        #[arg(short, long)]
        /// Destination directory
        dest_dir: Option<PathBuf>,
        #[arg(long)]
        /// Disable opening the received file automatically using the system default program
        no_open: bool,
    },
}
