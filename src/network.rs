use std::{
    net::{IpAddr, TcpListener},
    ops::RangeInclusive,
    process::exit,
};

const PORT_RANGE: RangeInclusive<u16> = 1024..=49151; // user port range
pub const DEFAULT_ADDRESS: &str = "0.0.0.0";

pub fn is_port_valid(s: &str) -> Result<u16, String> {
    let port: u16 = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a port number"))?;
    Ok(port)
}

pub fn is_ip_address_valid(s: &str) -> Result<String, String> {
    let address: IpAddr = s
        .parse()
        .map_err(|_| format!("{s} is not a valid IP address"))?;
    Ok(address.to_string())
}

pub fn can_bind_to_port(port: u16) -> bool {
    let addr = format!("{DEFAULT_ADDRESS}:{port}");
    TcpListener::bind(addr).is_ok()
}

pub fn find_available_port(user_port: Option<u16>) -> u16 {
    if let Some(port) = user_port {
        if can_bind_to_port(port) {
            return port;
        }
        println!("Selected port {port} is not available. Searching for another available port...");
    }
    for port in PORT_RANGE {
        if can_bind_to_port(port) {
            return port;
        }
    }
    eprintln!("Unable to find an available port on the system.");
    exit(1);
}

pub fn determine_ip(ip: String) -> String {
    if ip != DEFAULT_ADDRESS {
        return ip;
    }
    match default_net::interface::get_local_ipaddr() {
        Some(ip) => ip.to_string(),
        None => {
            eprintln!("unable to determine local IP address of the default network interface. Please provide a network interface or IP address");
            exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_port_valid() {
        assert!(is_port_valid("8300").is_ok());
    }

    #[test]
    fn test_is_port_valid_fails() {
        assert!(is_port_valid("blabla").is_err());
    }

    #[test]
    fn test_find_available_port() {
        let port = find_available_port(None);
        assert!(is_port_valid(&port.to_string()).is_ok());
    }

    #[test]
    fn test_can_bind() {
        let port = find_available_port(None);
        let bindable = can_bind_to_port(port);
        assert!(bindable);
    }
}
