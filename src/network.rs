use std::{net::TcpListener, ops::RangeInclusive};

const PORT_RANGE: RangeInclusive<u16> = 1..=65535;
pub const LOCALHOST: &str = "0.0.0.0";

pub fn port_in_range(s: &str) -> Result<u16, String> {
    let port: u16 = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a port number"))?;
    if PORT_RANGE.contains(&port) {
        Ok(port)
    } else {
        Err(format!(
            "port not in range {}-{}",
            PORT_RANGE.start(),
            PORT_RANGE.end()
        ))
    }
}

pub fn is_port_available(port: u16) -> bool {
    let addr = format!("{LOCALHOST}:{port}");
    TcpListener::bind(addr).is_ok()
}

pub fn find_available_port() -> u16 {
    for port in PORT_RANGE {
        if is_port_available(port) {
            return port;
        }
    }
    panic!("Unable to find an available port on the system.")
}
