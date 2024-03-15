use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
};

use axum::{debug_handler, http::StatusCode, response::Response, Json};
use serde::Deserialize;
use tracing::{debug, error};
use url::Url;

use super::*;

/// A REST API server to minimize urls
/// # Endpoints
/// - `POST /create-url`
///   - Params: long-url
///   - Status code: 201 Accepted
/// - `GET /{short-url}`
///   - Status code: 301 Permanent Redirect
pub struct TinyUrlServer {
    host_address: SocketAddr,
    big_to_tiny_map: HashMap<Url, Url>,
}
impl TinyUrlServer {
    pub const DEFAULT_HOST_ADDRESS: &'static str = "localhost::8080";

    pub fn from_env_or_default() -> Result<Self, std::net::AddrParseError> {
        let host_address = std::env::args()
            .nth(1)
            .unwrap_or_else(|| Self::DEFAULT_HOST_ADDRESS.into());
        return Ok(TinyUrlServer::new(&host_address)?);
    }

    pub fn new(host_address: &str) -> Result<Self, std::net::AddrParseError> {
        let host_address = match host_address.parse() {
            Ok(parsed_address) => parsed_address,
            Err(e) => {
                error!("Could not parse host_address ({}): {}", host_address, e);
                return Err(e);
            },
        };

        return Ok(TinyUrlServer {
            host_address,
            big_to_tiny_map: HashMap::new(),
        });
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(self.host_address).await?;

        let router = Router::new().route("/create-url", post(create_url));

        axum::serve(listener, router).await?;

        return Ok(());
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateUrlParams(pub Url);

#[debug_handler]
pub async fn create_url(
    Json(CreateUrlParams(long_url)): Json<CreateUrlParams>,
) -> Result<Response, StatusCode> {
    debug!("POST /create-url ({})", long_url);

    todo!();
}


