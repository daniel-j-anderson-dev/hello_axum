use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use tracing::{debug, trace, Level};

#[cfg(test)]
mod test;

pub mod servers;

pub const LOCAL_HOST_8080: SocketAddr =
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 135)), 8080);
pub const DEFAULT_ROOT_DIR: &str = "web";
pub const EXAMPLE_URL: &str = "https://www.euclideanspace.com/maths/geometry/trig/functions/index.htm";

pub fn initialize_stdout_subscriber(level: Level) {
    if let Err(e) = tracing_subscriber::fmt::Subscriber::builder()
        .with_max_level(level)
        .with_writer(std::io::stdout)
        .try_init()
    {
        trace!("Failed to initialize subscriber: {}", e);
    }
}
