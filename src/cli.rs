use crate::{
    network::{is_ip_address_valid, is_network_interface_valid, is_port_valid},
    qr::generate_qr_code,
    GlobalConfig,
};
use clap::{Parser, Subcommand};
use std::{net::IpAddr, path::PathBuf, process::exit};
use uuid::Uuid;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// IP Address to bind to
    #[arg(long, value_parser = is_ip_address_valid, global = true)]
    ip: Option<IpAddr>,
    /// Network interface to use (ignored if --ip provided)
    #[arg(short, long, value_parser = is_network_interface_valid, global = true)]
    interface: Option<default_net::Interface>,
    #[arg(short, long, value_parser = is_port_valid, global = true)]
    /// Server port
    port: Option<u16>,
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn into_config(self) -> GlobalConfig {
        let mut global_config = GlobalConfig::new(&self.ip, &self.interface, &self.port);
        let route = match &self.command {
            Commands::Send { file } => self.send(&global_config, file.clone()),
            Commands::Receive { dest_dir, no_open } => {
                self.receive(&mut global_config, dest_dir, no_open)
            }
        };
        generate_qr_code(&global_config.socket_addr, &route);
        global_config
    }

    fn send(&self, global_config: &GlobalConfig, file: PathBuf) -> String {
        let uuid = Uuid::new_v4().to_string();
        global_config
            .uuid_path_map
            .lock()
            .expect("global state already locked in the same thread")
            .insert(uuid.clone(), file);
        format!("/download/{uuid}")
    }

    fn receive(
        &self,
        global_config: &mut GlobalConfig,
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
            global_config.destination_dir = dest_dir.clone();
        }
        global_config.auto_open = !no_open;
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
        #[arg(long, default_value_t = false)]
        /// Disable opening the received file automatically using the system default program
        no_open: bool,
    },
}
