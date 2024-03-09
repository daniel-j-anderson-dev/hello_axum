use super::*;

#[tokio::test]
async fn root() {
    tracing_subscriber::fmt::init();
    tokio::spawn(server("localhost:8000"));
    let http_client = httpc_test::new_client("http://localhost:8000").unwrap();
    http_client.do_get("/").await.unwrap().print().await.unwrap();
}