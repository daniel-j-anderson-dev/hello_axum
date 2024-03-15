use std::{sync::Once, time::Duration};

use tokio::time::sleep;
use tracing::{error, warn};

use super::*;

const FILE_PATHS: &[&str] = &[
    "/",
    "/assets/duck_drink.gif",
    "/assets/duck.jpg",
    "/assets/many_ducks.gif",
    "/assets/sleepy_duck.gif",
];

pub fn initialize_stdout_subscriber() {
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        tracing_subscriber::fmt::Subscriber::builder()
            .with_writer(std::io::stdout)
            .init();
    });
}

#[tokio::test]
async fn serve_files() {
    initialize_stdout_subscriber();

    let root_dir = format!("{}/assets", DEFAULT_ROOT_DIR);

    tokio::spawn(server::host_files_with_index(DEFAULT_HOST_IP, root_dir.clone()));
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
    }
}
