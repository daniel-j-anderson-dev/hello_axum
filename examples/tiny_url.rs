use std::env;

use tracing::{info, trace, Level};

use hello_axum::{initialize_stdout_subscriber, servers::tiny_url::TinyUrlServer, LOCAL_HOST_8080};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    initialize_stdout_subscriber(Level::TRACE);

    let host_addr = env::args().nth(1).unwrap_or(LOCAL_HOST_8080.to_string());

    let server = TinyUrlServer::new(&host_addr)?;

    info!("Listening on {}", host_addr);
    server.run().await?;

    return Ok(());
}
