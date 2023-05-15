use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::network::{determine_ip, find_available_port};

#[derive(Debug, Clone)]
pub struct GlobalConfig {
    pub uuid_path_map: Arc<Mutex<HashMap<String, PathBuf>>>,
    pub destination_dir: Option<PathBuf>,
    pub auto_open: bool,
    pub socket_addr: SocketAddr,
}

impl GlobalConfig {
    pub fn new(ip_addr: String, port: Option<u16>) -> GlobalConfig {
        let ip = determine_ip(ip_addr);
        let server_port = find_available_port(&ip, port);
        let socket_addr = SocketAddr::new(ip.parse().unwrap(), server_port);
        GlobalConfig {
            destination_dir: None,
            auto_open: true,
            socket_addr,
            uuid_path_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
