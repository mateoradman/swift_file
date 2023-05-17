use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::network::get_socket_addr;

#[derive(Debug, Clone)]
pub struct GlobalConfig {
    pub uuid_path_map: Arc<Mutex<HashMap<String, PathBuf>>>,
    pub destination_dir: Option<PathBuf>,
    pub auto_open: bool,
    pub socket_addr: SocketAddr,
}

impl GlobalConfig {
    pub fn new(
        ip: &Option<IpAddr>,
        interface: &Option<default_net::Interface>,
        port: &Option<u16>,
    ) -> GlobalConfig {
        let socket_addr = get_socket_addr(ip, interface, port);
        GlobalConfig {
            destination_dir: None,
            auto_open: true,
            socket_addr,
            uuid_path_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
