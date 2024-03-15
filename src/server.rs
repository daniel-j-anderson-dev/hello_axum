//! This module is a collection of functions that call [axum::serve] with a variety of contracts

mod handlers;

use axum::{
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;

/// A simple server only using the [tower_http::services::ServeDir] middleware as a nest_service
pub async fn tower_serve_dir(host_ip: &str, root_dir: &str) -> Result<(), std::io::Error> {
    use tower_http::services::ServeDir;

    let listener = TcpListener::bind(host_ip).await?;

    let router = Router::new().nest_service("/", ServeDir::new(root_dir));

    axum::serve(listener, router).await?;

    return Ok(());
}

/// A simple server with similar functionality to [tower_http::services::ServeDir] using my [handlers] module instead
pub async fn host_files_with_index(host_ip: &str, root_dir: String) -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(host_ip).await?;

    let router = Router::new()
        .route("/", get(handlers::root_index))
        .route("/*file_path", get(handlers::serve_file))
        .with_state(root_dir);

    axum::serve(listener, router).await?;

    return Ok(());
}

/// A REST API server to minimize urls
/// # Endpoints
/// - `POST /create-url`
///   - Params: long-url
///   - Status code: 201 Accepted
/// - `GET /{short-url}`
///   - Status code: 301 Permanent Redirect
pub async fn tiny_url(host_ip: &str) -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(host_ip).await?;

    let router = Router::new().route("/create-url", post(handlers::create_url));

    axum::serve(listener, router).await?;

    return Ok(());
}
