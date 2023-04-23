use std::{net::TcpListener, ops::RangeInclusive};

const PORT_RANGE: RangeInclusive<u16> = 1024..=49151; // user port range
pub const LOCALHOST: &str = "0.0.0.0";
pub const CONTENT_LENGTH_LIMIT: usize = 1024 * 1024 * 1024; /* 1gb */

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

pub fn find_available_port(server_port: &mut u16, user_port: &Option<u16>) {
    if let Some(port) = user_port {
        if is_port_available(*port) {
            *server_port = *port;
            return;
        }
    }
    for port in PORT_RANGE {
        if is_port_available(port) {
            *server_port = port;
            return;
        }
    }

    panic!("Unable to find an available port on the system.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_in_range_fails() {
        assert!(port_in_range("9999999999").is_err());
    }

    #[test]
    fn test_port_in_range() {
        assert!(port_in_range("8300").is_ok());
    }

    #[test]
    fn test_find_available_port() {
        let mut server_port: u16 = 0;
        find_available_port(&mut server_port, &None);
        assert!(port_in_range(&server_port.to_string()).is_ok());
    }
}
