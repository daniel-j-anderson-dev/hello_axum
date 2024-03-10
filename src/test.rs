use std::time::Duration;

use tokio::time::sleep;

use super::*;

#[tokio::test]
async fn serve_files() {
    tracing_subscriber::fmt::init();

    tokio::spawn(server(DEFAULT_HOST_IP, DEFAULT_ROOT_DIR));
    sleep(Duration::from_millis(100)).await;

    let http_client = httpc_test::new_client(format!("http://{}", DEFAULT_HOST_IP)).unwrap();

    const FILE_PATHS: &[&str] = &[
        "/",
        "/assets/duck_drink.gif",
        "/assets/duck.jpg",
        "/assets/many_ducks.gif",
        "/assets/sleepy_duck.gif",
    ];
    for file_path in FILE_PATHS {
        info!("trying to get {}", file_path);
        let response = http_client.do_get(&file_path).await.expect("do_get failed");

        response.print().await.expect("print failed");

        assert!(response.status().is_success());
    }
}
