use crate::{
    network::{find_available_port, port_in_range},
    qr::generate_qr_code,
    AppState,
};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn decide(&self, server_port: &mut u16, shared_state: &mut AppState) {
        match &self.command {
            Commands::Send { file, port } => {
                self.send(server_port, shared_state, file.clone(), port)
            }
            Commands::Receive {
                dest_dir,
                port,
                no_open,
            } => self.receive(server_port, shared_state, dest_dir, port, no_open),
        };
    }

    fn send(
        &self,
        server_port: &mut u16,
        shared_state: &AppState,
        file: PathBuf,
        port: &Option<u16>,
    ) {
        find_available_port(server_port, port);
        let uuid = Uuid::new_v4().to_string();
        shared_state
            .uuid_path_map
            .lock()
            .expect("shared state was poisoned")
            .insert(uuid.clone(), file);
        let route = format!("/download/{uuid}");
        generate_qr_code(server_port, &route);
    }

    fn receive(
        &self,
        server_port: &mut u16,
        shared_state: &mut AppState,
        dest_dir: &Option<PathBuf>,
        port: &Option<u16>,
        no_open: &bool,
    ) {
        find_available_port(server_port, port);
        shared_state.destination_dir = dest_dir.clone();
        shared_state.auto_open = !no_open;
        generate_qr_code(server_port, "/receive");
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Send a file
    Send {
        /// File path to send
        file: PathBuf,
        #[arg(short, long, value_parser = port_in_range)]
        /// Port to bind the server to (allowed user port range 1024 to 49151)
        port: Option<u16>,
    },

    /// Receive a file
    Receive {
        #[arg(short, long)]
        /// Destination directory
        dest_dir: Option<PathBuf>,
        #[arg(short, long, value_parser = port_in_range)]
        /// Port to bind the server to (allowed user port range 1024 to 49151)
        port: Option<u16>,
        #[arg(long)]
        /// Disable opening the received file automatically using the system default program
        no_open: bool,
    },
}
