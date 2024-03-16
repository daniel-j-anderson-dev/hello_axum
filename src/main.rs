use std::env;

use hello_axum::{servers, DEFAULT_HOST_ADDRESS, DEFAULT_ROOT_DIR};
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let host_addr = env::args().nth(1).unwrap_or(DEFAULT_HOST_ADDRESS.to_string()).parse()?;
    let root_directory = env::args().nth(2).unwrap_or(DEFAULT_ROOT_DIR.into());

    info!("Listening on serving {} on {}", root_directory, host_addr);

    servers::host_files_with_index(host_addr, root_directory).await?;
    // server::tower_serve_dir(&host_ip, &root_directory).await?;

    return Ok(());
}
