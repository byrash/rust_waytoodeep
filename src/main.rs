use color_eyre::Report;
use reqwest::{Client, Url};
use tracing::{info, instrument};

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup()?;
    info!("Hello from a (so far completely unnecessary) async runtime");

    let url = "https://google.com";
    call_api(url).await?;

    Ok(())
}

#[instrument]
async fn call_api(url: &str) -> Result<(), Report> {
    let client = Client::new();
    let resp = client.get(url).send().await?.error_for_status()?;
    info!(%url,content_type = ?resp.headers().get("content-type"), "Got a response!");
    Ok(())
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt().init();

    Ok(())
}
