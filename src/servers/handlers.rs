//! This module contains some general axum route handlers and helpers

use axum::{debug_handler, extract::Path, http::StatusCode, response::Response, Extension};
use tokio::{fs::File, io::AsyncReadExt};
use tracing::{debug, error};

/// Return the file data or a status code.
/// - This function logs any status code returned with [warn]
pub async fn open_file(file_path: &str) -> Result<Vec<u8>, StatusCode> {
    let mut file_data = Vec::new();

    File::open(&file_path)
        .await
        .map_err(|e| {
            error!("Failed to open {}: {}", file_path, e);
            StatusCode::NOT_FOUND
        })?
        .read_to_end(&mut file_data)
        .await
        .map_err(|e| {
            error!("Failed to read {}: {}", file_path, e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    return Ok(file_data);
}

#[debug_handler]
/// Try to return a file in a response body. Use [tower_http::services::ServeDir] instead
pub async fn serve_file(
    Extension(root_dir): Extension<String>,
    Path(file_path): Path<String>,
) -> Result<Response, StatusCode> {
    let file_path = format!("{}/{}", root_dir, file_path);

    debug!("File requested ({})", file_path);

    let file_data = open_file(&file_path).await?;

    let response = Response::new(file_data.into());

    debug!("Returning file ({})", file_path);

    return Ok(response);
}

#[debug_handler]
pub async fn root_index(Extension(root_dir): Extension<String>) -> Result<Response, StatusCode> {
    let file_path = format!("{}/index.html", root_dir);

    debug!("Root index requested ({})", file_path);

    let file_data = open_file(&file_path).await?;

    let response = Response::new(file_data.into());

    debug!("Returning root_index ({})", file_path);

    return Ok(response);
}
