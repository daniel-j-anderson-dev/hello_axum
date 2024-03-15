mod server;
#[cfg(test)]
mod test;

use std::env;

use tracing::{info, Level};

const DEFAULT_HOST_IP: &str = "localhost:8080";
const DEFAULT_ROOT_DIR: &str = "web";

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    let host_ip = env::args().nth(1).unwrap_or(DEFAULT_HOST_IP.into());
    let root_directory = env::args().nth(2).unwrap_or(DEFAULT_ROOT_DIR.into());

    info!("Listening on serving {} on {}", root_directory, host_ip);

    server::tower_serve_dir(&host_ip, &root_directory).await?;

    return Ok(());
}
