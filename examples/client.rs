use color_eyre::Report;

#[tokio::main]
async fn main() -> Result<(), Report> {
    let http_client = httpc_test::new_client("http://localhost:8000/assets/duck.jpg")?;
    http_client.do_get("/").await?.print().await?;
    return Ok(())
}