mod handlers;
#[cfg(test)]
mod test;

use std::env;

use axum::Router;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::{info, Level};

const DEFAULT_HOST_IP: &str = "localhost:8080";
const DEFAULT_ROOT_DIR: &str = "web";

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt().with_max_level(Level::TRACE).init();
    
    let host_ip = env::args().nth(1).unwrap_or(DEFAULT_HOST_IP.into());
    let root_directory = env::args().nth(2).unwrap_or(DEFAULT_ROOT_DIR.into());
    
    info!("Listening on serving {} on {}", root_directory, host_ip);
    
    server(&host_ip, &root_directory).await?;

    return Ok(());
}

pub async fn server(host_ip: &str, root_dir: &str) -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(host_ip).await?;

    let router = Router::new().nest_service("/", ServeDir::new(root_dir));

    axum::serve(listener, router).await?;

    return Ok(());
}
