use std::{
    collections::{hash_map::Entry, HashMap},
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{
    debug_handler,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Redirect, Response},
};
use serde::Deserialize;
use tracing::{debug, error, trace};
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
    long_to_tiny_map: HashMap<Url, String>,
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
        let listener = TcpListener::bind(&self.host_address).await?;

        let router = Router::new()
            .route("/create-url/", post(create_entry))
            .route("/*tiny-url", get(redirect_tiny_url))
            .with_state(AppData::from(self));

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

pub fn generate_suffix() -> String {
    let charset = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let suffix = random_string::generate(5, charset);
    return suffix;
}

#[derive(Debug, Clone)]
pub struct AppData(Arc<Mutex<HashMap<Url, String>>>, Arc<SocketAddr>);
impl From<TinyUrlServer> for AppData {
    fn from(value: TinyUrlServer) -> Self {
        return Self(
            Arc::new(Mutex::new(value.long_to_tiny_map)),
            Arc::new(value.host_address),
        );
    }
}

#[derive(Debug, Deserialize)]
pub struct Params {
    #[serde(alias = "long-url")]
    long_url: Url,
}

#[debug_handler]
pub async fn create_entry(
    State(AppData(long_to_tiny_map, host_addr)): State<AppData>,
    Query(Params { long_url }): Query<Params>,
) -> Result<Response<String>, StatusCode> {
    debug!("POST /create-url {}", long_url);

    // let long_url = long_url.parse().map_err(|e| {
    //     error!("Failed to parse long_url: {}", e);
    //     StatusCode::BAD_REQUEST
    // })?;

    let suffix = generate_suffix();

    let mut long_to_tiny_map = long_to_tiny_map.try_lock().map_err(|e| {
        error!("Failed to to lock big_to_tiny_map: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let tiny_url = format!("http://{}/{}", host_addr, suffix)
        .parse::<Url>()
        .map_err(|e| {
            error!("Failed to parse tiny_url: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    debug!("Added {} -> {}", tiny_url, long_url);

    match long_to_tiny_map.entry(long_url) {
        Entry::Occupied(o_e) => {
            debug!("Entry already made suffix: \"{}\"", o_e.get());
        }
        Entry::Vacant(v_e) => {
            v_e.insert(suffix);
        }
    };

    let mut response = Response::new(tiny_url.to_string());
    *response.status_mut() = StatusCode::CREATED;

    trace!("{:?}", response);

    return Ok(response);
}

#[debug_handler]
pub async fn redirect_tiny_url(
    State(AppData(long_to_tiny_map, _)): State<AppData>,
    Path(tiny_url): Path<String>,
) -> Result<Redirect, StatusCode> {
    let long_to_tiny_map = match long_to_tiny_map.try_lock() {
        Ok(lock) => lock,
        Err(e) => {
            error!("Failed to to lock big_to_tiny_map: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let long_url = long_to_tiny_map
        .iter()
        .find_map(|(long_url, suffix)| {
            if suffix.as_str() == tiny_url.as_str() {
                Some(long_url)
            } else {
                None
            }
        })
        .ok_or(StatusCode::NOT_FOUND)?;

    let redirect = Redirect::permanent(long_url.to_string().as_str());

    return Ok(redirect);
}
