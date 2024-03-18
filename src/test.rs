use std::time::Duration;

use reqwest::Client;
use tokio::time::sleep;
use tracing::{debug, error, info, warn, Level};
use url::Url;

use super::*;

const FILE_PATHS: &[&str] = &[
    "/",
    "/assets/duck_drink.gif",
    "/assets/duck.jpg",
    "/assets/many_ducks.gif",
    "/assets/sleepy_duck.gif",
];

#[tokio::test]
async fn serve_files() {
    initialize_stdout_subscriber(Level::DEBUG);

    tokio::spawn(servers::host_files_with_index(
        LOCAL_HOST_8080,
        DEFAULT_ROOT_DIR.into(),
    ));
    sleep(Duration::from_millis(100)).await;

    let http_client = httpc_test::new_client(format!("http://{}", LOCAL_HOST_8080)).unwrap();

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
    initialize_stdout_subscriber(Level::DEBUG);

    tokio::spawn(servers::tower_serve_dir(LOCAL_HOST_8080, DEFAULT_ROOT_DIR));
    sleep(Duration::from_millis(100)).await;

    let http_client = httpc_test::new_client(format!("http://{}", LOCAL_HOST_8080)).unwrap();

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
    initialize_stdout_subscriber(Level::TRACE);

    tokio::spawn(async {
        info!("starting tiny_url server");
        servers::tiny_url::TinyUrlServer::from_env_or_default()
            .run()
            .await
            .unwrap()
    });
    sleep(Duration::from_millis(100)).await;

    let client = Client::new();

    let create_endpoint = format!("http://{LOCAL_HOST_8080}/create-ur/?long-url={EXAMPLE_URL}").parse::<Url>().unwrap();
    let response = client
        .post(create_endpoint)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    let status = response.status();
    let tiny_url = response.text().await.unwrap();

    debug!("{} {}", status, tiny_url);

    let redirect_endpoint = format!("http://{LOCAL_HOST_8080}/{tiny_url}").parse::<Url>().unwrap();
    let response = client
        .get(redirect_endpoint)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    let status = response.status();
    let long_url = response
        .headers()
        .get("location")
        .unwrap()
        .to_str()
        .unwrap();

    debug!("{} {}", status, long_url);
}
