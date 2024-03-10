use crate::DEFAULT_ROOT_DIR;

use axum::{debug_handler, extract::Path, http::StatusCode, response::Response};
use tokio::{fs::File, io::AsyncReadExt};
use tracing::{info, warn};

#[debug_handler]
/// Try to return a file in a response body. Use [tower_http::services::ServeDir] instead
pub async fn serve_file(Path(file_path): Path<String>) -> Result<Response, StatusCode> {
    let file_path = format!("{}/{}", DEFAULT_ROOT_DIR, file_path);

    info!("{} requested", file_path);

    let mut file_data = Vec::new();

    File::open(&file_path)
        .await
        .map_err(|e| {
            warn!("Failed to open {}: {}", file_path, e);
            StatusCode::NOT_FOUND
        })?
        .read_to_end(&mut file_data)
        .await
        .map_err(|e| {
            warn!("Failed to read {}: {}", file_path, e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let response = Response::new(file_data.into());

    return Ok(response);
}
