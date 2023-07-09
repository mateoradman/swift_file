use crate::{
    files,
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
    pub async fn into_config(self) -> GlobalConfig {
        let mut global_config = GlobalConfig::new(&self.ip, &self.interface, &self.port);
        let route = match &self.command {
            Commands::Send { path, zip } => self.send(&mut global_config, path.clone(), zip).await,
            Commands::Receive { dest_dir, no_open } => {
                self.receive(&mut global_config, dest_dir, no_open).await
            }
        };
        generate_qr_code(&global_config.socket_addr, &route).await;
        global_config
    }

    async fn send(&self, global_config: &mut GlobalConfig, path: PathBuf, zip: &bool) -> String {
        if !path.exists() {
            eprintln!("Path {} does not exist on disk", path.to_str().unwrap());
            exit(1)
        }
        if path.is_dir() && !zip {
            eprintln!("Unable to send a directory without creating a zip. Tip: use --zip");
            exit(1)
        }

        let mut file_path = path.clone();
        if *zip {
            let file_stem = match path.file_stem() {
                Some(val) => val.to_str().unwrap_or("file"),
                None => {
                    eprintln!("Unable to determine file stem from the provided file path");
                    exit(1)
                }
            };
            file_path = match files::create_archive(file_stem, &path).await {
                Ok(path) => path,
                Err(err) => {
                    eprintln!("Unable to create a zip file from {file_stem} due to {err}");
                    exit(1)
                }
            };
        }

        let uuid = Uuid::new_v4().to_string();
        global_config
            .uuid_path_map
            .lock()
            .expect("global state already locked in the same thread")
            .insert(uuid.clone(), file_path);
        global_config.zip = *zip;
        format!("/download/{uuid}")
    }

    async fn receive(
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
    /// Send a file or directory
    Send {
        /// Path to a file (or directory if using --zip)
        path: PathBuf,
        #[arg(long, default_value_t = false)]
        /// ZIP file or directory before transferring
        zip: bool,
    },

    /// Receive files
    Receive {
        #[arg(short, long)]
        /// Destination directory
        dest_dir: Option<PathBuf>,
        #[arg(long, default_value_t = false)]
        /// Disable opening the received file automatically using the system default program
        no_open: bool,
    },
}
