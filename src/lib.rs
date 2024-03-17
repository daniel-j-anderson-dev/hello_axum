use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use tracing::{debug, Level};

#[cfg(test)]
mod test;

pub mod servers;

pub const LOCAL_HOST_8080: SocketAddr =
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
pub const DEFAULT_ROOT_DIR: &str = "web";
pub const EXAMPLE_URL: &str = "https://www.euclideanspace.com/maths/geometry/trig/functions/index.htm";

pub fn initialize_stdout_subscriber(level: Level) {
    if let Err(e) = tracing_subscriber::fmt::Subscriber::builder()
        .with_max_level(level)
        .with_writer(std::io::stdout)
        .try_init()
    {
        debug!("Failed to initialize subscriber: {}", e);
    }
}
