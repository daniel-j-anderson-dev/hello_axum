#[cfg(test)]
mod test;

use std::env;

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tracing::{debug, info};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();
    let ip = env::args().nth(1).unwrap_or("localhost:8000".into());
    server(&ip).await?;
    return Ok(());
}

pub async fn server(ip: &str) ->  Result<(), std::io::Error> {
    let listener = TcpListener::bind(ip).await?;
    
    let router = Router::new()
        .route("/", get(root));
    
    info!("Listening on {}", ip);
    
    axum::serve(listener, router).await?;
    
    return Ok(());
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    info!("Hello from handle root!");
    "Hello, World!"
}
