use std::{
    collections::{hash_map::Entry, HashMap},
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{debug_handler, extract::{Path, State}, http::StatusCode, response::Response, Json};
use serde::Deserialize;
use tracing::{debug, error};
use url::Url;

use crate::{servers::*, DEFAULT_HOST_ADDRESS};

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
    pub fn from_env_or_default() -> Self {
        return Self {
            host_address: std::env::args()
                .nth(1)
                .and_then(|s| {
                    return s.parse().ok();
                })
                .unwrap_or(DEFAULT_HOST_ADDRESS),
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

    pub async fn run(self) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(self.host_address).await?;

        let long_to_tiny_map = Arc::new(Mutex::new(self.long_to_tiny_map));

        let router = Router::new()
            .route("/create-url", post(create_url))
            .route("/*tiny-url", get(redirect_tiny_url))
            .with_state(long_to_tiny_map);

        axum::serve(listener, router).await?;

        return Ok(());
    }
}
impl Default for TinyUrlServer {
    fn default() -> Self {
        return Self {
            host_address: DEFAULT_HOST_ADDRESS,
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
    Json(Params { long_url }): Json<Params>,
) -> Result<Response<String>, StatusCode> {
    debug!("POST /create-url {}", long_url);

    let tiny_url = minify_url(&long_url);

    let mut long_to_tiny_map = match long_to_tiny_map.try_lock() {
        Ok(lock) => lock,
        Err(e) => {
            error!("Failed to to lock big_to_tiny_map: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    match long_to_tiny_map.entry(long_url) {
        Entry::Occupied(o_e) => {
            debug!("Entry already made shortened url: \"{}\"", o_e.get());
        }
        Entry::Vacant(v_e) => {
            debug!("Added {{ \"{}\": \"{}\" }}", v_e.key(), tiny_url);
            v_e.insert(tiny_url.clone());
        }
    };

    let mut response = Response::new(tiny_url.to_string());
    *response.status_mut() = StatusCode::CREATED;

    debug!("{:?}", response);

    return Ok(response);
}

#[debug_handler]
pub async fn redirect_tiny_url(
    State(long_to_tiny_map): State<TinyUrlMap>,
    Path(tiny_url): Path<String>,
) -> Result<Response<String>, StatusCode> {
    let long_to_tiny_map = match long_to_tiny_map.try_lock() {
        Ok(lock) => lock,
        Err(e) => {
            error!("Failed to to lock big_to_tiny_map: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let long_url = long_to_tiny_map
        .iter()
        .find_map(|(k, v)| if v.to_string() == tiny_url {Some(k)} else {None})
        .ok_or(StatusCode::NOT_FOUND)?;

    let mut response = Response::new(long_url.to_string());
    *response.status_mut() = StatusCode::PERMANENT_REDIRECT;

    debug!("{:?}", response);

    return Ok(response);
}