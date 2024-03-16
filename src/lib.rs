use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[cfg(test)]
mod test;

pub mod servers;

pub const DEFAULT_HOST_ADDRESS: SocketAddr =
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
pub const DEFAULT_ROOT_DIR: &str = "web";
