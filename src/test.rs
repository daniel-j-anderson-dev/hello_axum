use std::time::Duration;

use reqwest::Client;
use tokio::time::sleep;
use tracing::{debug, error, info, warn, Level};

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
        .with_max_level(Level::DEBUG)
        .with_writer(std::io::stdout)
        .try_init()
    {
        debug!("Failed to initialize subscriber: {}", e);
    }
}

#[tokio::test]
async fn serve_files() {
    initialize_stdout_subscriber();

    tokio::spawn(servers::host_files_with_index(
        DEFAULT_HOST_ADDRESS,
        DEFAULT_ROOT_DIR.into(),
    ));
    sleep(Duration::from_millis(100)).await;

    let http_client = httpc_test::new_client(format!("http://{}", DEFAULT_HOST_ADDRESS)).unwrap();

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

    tokio::spawn(servers::tower_serve_dir(
        DEFAULT_HOST_ADDRESS,
        DEFAULT_ROOT_DIR,
    ));
    sleep(Duration::from_millis(100)).await;

    let http_client = httpc_test::new_client(format!("http://{}", DEFAULT_HOST_ADDRESS)).unwrap();

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
async fn tiny_url() {
    initialize_stdout_subscriber();

    tokio::spawn(servers::tiny_url::TinyUrlServer::from_env_or_default().run());
    sleep(Duration::from_millis(100)).await;

    let client = Client::new();

    let response = client
        .post(format!("http://{}/create-url", DEFAULT_HOST_ADDRESS))
        .body("https://www.euclideanspace.com/maths/geometry/trig/functions/index.htm")
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    let status = response.status();
    let tiny_url = response.text().await.unwrap();

    debug!("{} {}", status, tiny_url);

    let response = client
        .get(format!("http://{}/{}", DEFAULT_HOST_ADDRESS, tiny_url))
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    let status = response.status();
    let long_url = response.text().await.unwrap();

    debug!("{} {}", status, long_url);
}
