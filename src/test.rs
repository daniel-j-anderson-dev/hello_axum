use std::time::Duration;

use tokio::time::sleep;
use tracing::{debug, error, warn};

use super::*;

const FILE_PATHS: &[&str] = &[
    "/",
    "/assets/duck_drink.gif",
    "/assets/duck.jpg",
    "/assets/many_ducks.gif",
    "/assets/sleepy_duck.gif",
];

pub fn initialize_stdout_subscriber() {
    if let Err(e) = tracing_subscriber::fmt::Subscriber::builder()
        .with_max_level(Level::TRACE)
        .with_writer(std::io::stdout)
        .try_init()
    {
        debug!("Failed to initialize subscriber: {}", e);
    }
}

#[tokio::test]
async fn serve_files() {
    initialize_stdout_subscriber();

    tokio::spawn(server::host_files_with_index(
        DEFAULT_HOST_IP,
        DEFAULT_ROOT_DIR.into(),
    ));
    sleep(Duration::from_millis(100)).await;

    let http_client = httpc_test::new_client(format!("http://{}", DEFAULT_HOST_IP)).unwrap();

    for file_path in FILE_PATHS {
        info!("trying to get {}", file_path);
        let response = http_client.do_get(&file_path).await.expect("do_get failed");

        if let Err(e) = response.print().await {
            warn!("failed to print response: {}", e);
        }

        if !response.status().is_success() {
            error!("Response failed!");
        }

        debug!("response status: {}", response.status());
    }
}

#[tokio::test]
async fn tower_serve_dir() {
    initialize_stdout_subscriber();

    tokio::spawn(server::tower_serve_dir(DEFAULT_HOST_IP, DEFAULT_ROOT_DIR));
    sleep(Duration::from_millis(100)).await;

    let http_client = httpc_test::new_client(format!("http://{}", DEFAULT_HOST_IP)).unwrap();

    for file_path in FILE_PATHS {
        info!("trying to get {}", file_path);
        let response = http_client.do_get(&file_path).await.expect("do_get failed");

        if let Err(e) = response.print().await {
            warn!("failed to print response: {}", e);
        }

        if !response.status().is_success() {
            error!("Response failed!");
        }

        debug!("response status: {}", response.status());
    }
}
