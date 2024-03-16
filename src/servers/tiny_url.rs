use std::{
    collections::{hash_map::Entry, HashMap},
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::{Arc, Mutex},
};

use axum::{body::Body, debug_handler, extract::State, http::StatusCode, response::Response, Json};
use serde::Deserialize;
use tracing::{debug, error, Level};
use url::Url;

use super::*;

/// A REST API server to minimize urls
/// # Endpoints
/// - `POST /create-url`
///   - body json:
///     ```json
///     {
///         "long-url": "your/long/url/that/needs/shortening"
///     }
///     ```
///   - Returns
///     - Status code: 201 Accepted
///     - Status code:
/// - `GET /{short-url}`
///   - Status code: 301 Permanent Redirect
pub struct TinyUrlServer {
    host_address: SocketAddr,
    long_to_tiny_map: HashMap<Url, Url>,
}
impl TinyUrlServer {
    pub const DEFAULT_HOST_ADDRESS: SocketAddr =
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

    pub fn from_env_or_default() -> Self {
        return Self {
            host_address: std::env::args()
                .nth(1)
                .and_then(|s| {
                    return s.parse().ok();
                })
                .unwrap_or(Self::DEFAULT_HOST_ADDRESS),
            long_to_tiny_map: HashMap::new(),
        };
    }

    pub fn new(host_address: &str) -> Result<Self, std::net::AddrParseError> {
        debug!("Creating new TinyUrlServer");

        let host_address = match host_address.parse() {
            Ok(parsed_address) => parsed_address,
            Err(e) => {
                error!("Could not parse host_address ({}): {}", host_address, e);
                return Err(e);
            }
        };

        return Ok(TinyUrlServer {
            host_address,
            long_to_tiny_map: HashMap::new(),
        });
    }

    async fn listener(&self) -> Result<TcpListener, std::io::Error> {
        return TcpListener::bind(self.host_address).await;
    }

    fn router(self) -> Router {
        let long_to_tiny_map = Arc::new(Mutex::new(self.long_to_tiny_map));
        return Router::new()
            .route("/create-url", post(create_url))
            .with_state(long_to_tiny_map);
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        axum::serve(self.listener().await?, self.router()).await?;

        return Ok(());
    }
}
impl Default for TinyUrlServer {
    fn default() -> Self {
        return Self {
            host_address: Self::DEFAULT_HOST_ADDRESS,
            long_to_tiny_map: HashMap::new(),
        };
    }
}

pub fn minify_url(long_url: &Url) -> Url {
    // TODO: FIXME
    return long_url.clone();
}

pub type TinyUrlMap = Arc<Mutex<HashMap<Url, Url>>>;

#[derive(Debug, Deserialize)]
pub struct Params {
    #[serde(alias = "long-url")]
    pub long_url: Url,
}

#[debug_handler]
pub async fn create_url(
    State(long_to_tiny_map): State<TinyUrlMap>,
    Json(Params{ long_url }): Json<Params>,
) -> Result<StatusCode, StatusCode> {
    debug!("POST /create-url ({})", long_url);

    let mut long_to_tiny_map = match long_to_tiny_map.try_lock() {
        Ok(lock) => lock,
        Err(e) => {
            error!("Failed to to lock big_to_tiny_map: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    match long_to_tiny_map.entry(long_url) {
        Entry::Occupied(o_e) => {
            debug!("Entry already made \"{}\": \"{}\"", o_e.key(), o_e.get());
        }
        Entry::Vacant(v_e) => {
            debug!("Creating tiny-url");
            let tiny_url = minify_url(v_e.key());

            debug!("Added {{ \"{}\": \"{}\" }}", v_e.key(), tiny_url);
            v_e.insert(tiny_url);
        }
    };

    return Ok(StatusCode::CREATED);
}
