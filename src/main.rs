use axum::{body::Bytes, debug_handler, extract::Path, http::{StatusCode, Uri}, response::Html, routing::get, Extension, Router};
use log::{info, trace};
use tokio::{fs::OpenOptions, io::AsyncReadExt, net::TcpListener};

pub const INDEX_HTML: Html<&'static str> = Html(include_str!("../web/index.html"));
pub const DUCK_JPG: Bytes = Bytes::from_static(include_bytes!("../web/assets/duck.jpg"));

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Trace)?;

    let host_address = std::env::var("HOST_ADDRESS").unwrap_or("localhost:8000".into());
    let listener = TcpListener::bind(&host_address).await?;
    let router = Router::new()
        .route("/", get(INDEX_HTML))
        .route("/:uri", get(handle_get_file));

    info!("Serving on {}", host_address);

    axum::serve(listener, router).await?;

    return Ok(());
}

#[debug_handler]
pub async fn handle_get_file(Path(file_path): Path<String>) -> Result<Bytes, StatusCode> {
    // let file_path = format!("web/assets/{}", uri.path());
    trace!("in handle_get_file: file path is {}", file_path);

    let mut file_data = Vec::new();

    OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&file_path)
        .await
        .map_err(|error| {
            info!("Failed to open {}; {}", file_path, error);
            StatusCode::NOT_FOUND
        })?
        .read_to_end(&mut file_data)
        .await
        .map_err(|error| {
            info!("Failed to read {}; {}", file_path, error);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    return Ok(file_data.into());
}
