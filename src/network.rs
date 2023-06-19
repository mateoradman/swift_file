use std::{
    format,
    net::{IpAddr, SocketAddr, TcpListener},
    ops::RangeInclusive,
    process::exit,
};

use anyhow::{anyhow, Context, Ok, Result};

const PORT_RANGE: RangeInclusive<u16> = 1024..=49151; // user port range

pub fn is_port_valid(s: &str) -> Result<u16> {
    let port: u16 = s
        .parse()
        .with_context(|| format!("`{s}` isn't a port number"))?;
    Ok(port)
}

pub fn is_ip_address_valid(s: &str) -> Result<IpAddr> {
    let address: IpAddr = s
        .parse()
        .with_context(|| format!("{s} isn't a valid IP address"))?;
    let socket = SocketAddr::new(address, 0);
    TcpListener::bind(socket)
        .with_context(|| format!("cannot bind to the provided IP address `{s}`"))?;
    Ok(address)
}

pub fn is_network_interface_valid(s: &str) -> Result<default_net::Interface> {
    for interface in default_net::get_interfaces() {
        if interface.name == s {
            if interface.ipv4.is_empty() && interface.ipv6.is_empty() {
                return Err(anyhow!(
                    "interface {s} has no IPv4 or IPv6 address to bind to"
                ));
            }
            return Ok(interface);
        }
    }
    Err(anyhow!("{s} is not a valid interface name"))
}

pub fn get_socket_addr(
    ip: &Option<IpAddr>,
    interface: &Option<default_net::Interface>,
    port: &Option<u16>,
) -> SocketAddr {
    let ip_addr = match ip {
        Some(addr) => *addr,
        None => get_interface_ip(interface),
    };
    let server_port = find_available_port(&ip_addr, port);
    SocketAddr::new(ip_addr, server_port)
}

fn get_interface_ip(interface: &Option<default_net::Interface>) -> IpAddr {
    match interface {
        Some(iface) => {
            if iface.ipv4.is_empty() {
                IpAddr::V6(iface.ipv6[0].addr)
            } else {
                IpAddr::V4(iface.ipv4[0].addr)
            }
        }
        None => get_default_interface_ip(),
    }
}

fn get_default_interface_ip() -> IpAddr {
    match default_net::interface::get_local_ipaddr() {
        Some(ip) => ip,
        None => {
            eprintln!("Unable to get local IP address of a default network interface.");
            exit(1);
        }
    }
}

fn find_available_port(ip: &IpAddr, user_port: &Option<u16>) -> u16 {
    if let Some(port) = user_port {
        if can_listen_on_port(ip, port) {
            return *port;
        }
        println!("Selected port {port} is not available. Searching for another available port...");
    }
    for port in PORT_RANGE {
        if can_listen_on_port(ip, &port) {
            return port;
        }
    }
    eprintln!("Unable to listen to any port on IP address {ip}.");
    exit(1);
}

fn can_listen_on_port(ip: &IpAddr, port: &u16) -> bool {
    let addr = SocketAddr::new(*ip, *port);
    TcpListener::bind(addr).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    const DEFAULT_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

    #[test]
    fn test_is_port_valid() {
        assert!(is_port_valid("8300").is_ok());
    }

    #[test]
    fn test_is_port_valid_fails() {
        assert!(is_port_valid("blabla").is_err());
    }

    #[test]
    fn test_is_ip_valid() {
        assert!(is_ip_address_valid("0.0.0.0").is_ok());
    }

    #[test]
    fn test_is_ip_valid_fails() {
        assert!(is_ip_address_valid("blabla").is_err());
    }

    #[test]
    fn test_is_network_interface_valid_fails() {
        assert!(is_network_interface_valid("blabla").is_err());
    }

    #[test]
    fn test_find_available_port() {
        let port = find_available_port(&DEFAULT_ADDRESS, &None);
        assert!(is_port_valid(&port.to_string()).is_ok());
    }

    #[test]
    fn test_can_bind() {
        let port = find_available_port(&DEFAULT_ADDRESS, &None);
        let bindable = can_listen_on_port(&DEFAULT_ADDRESS, &port);
        assert!(bindable);
    }
}
