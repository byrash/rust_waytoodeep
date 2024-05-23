use color_eyre::Report;
use futures::{stream::FuturesUnordered, StreamExt};
use futures_handler::FutureHandler;
use reqwest::Client;
use std::{future::Future, sync::Arc, time::Duration};
use tracing::{debug, info, instrument};
use tracing_subscriber::field::debug;
mod futures_handler;

#[tokio::main(flavor = "current_thread")]
// #[tokio::main]
#[instrument]
async fn main() -> Result<(), Report> {
    setup()?;

    // Reqwest is implemnting clone trait
    let client = Client::new();
    // For static spawned threads to work we allowing leak
    // let leaked_client = Box::leak(Box::new(client));
    // Unleaking CLient
    // let client = Arc::new(Client::new());

    // let fut = FutureHandler {};
    // fut.await;
    let google_url = "https://google.com";
    let bing_url = "https://www.bing.com/";

    // let google = fetch_url_raw(client.clone(), google_url);
    // let bing = fetch_url_raw(client.clone(), bing_url);

    // let google_thread_handle = tokio::spawn(google);
    // let bing_thread_handle = tokio::spawn(bing);

    // google_thread_handle.await.unwrap();
    // bing_thread_handle.await.unwrap();

    let mut url_requests = vec![
        fetch_url(client.clone(), google_url),
        fetch_url(client, bing_url), // Passing on client, just let it go
    ]
    .into_iter()
    .collect::<FuturesUnordered<_>>();

    while let Some(item) = url_requests.next().await {
        // propagate errors
        item?;
    }

    Ok(())
}

// Systatic Sugar
#[instrument]
async fn fetch_url(client: Client, url: &str) -> Result<(), Report> {
    info!(%url, "Requesting");
    let resp = client.get(url).send().await?.error_for_status()?;
    info!(%url,content_type = ?resp.headers().get("content-type"), "Got a response!");
    Ok(())
}

#[instrument]
#[allow(clippy::manual_async_fn)]
fn fetch_url_raw(
    client: Client,
    url: &'static str,
) -> impl Future<Output = Result<(), Report>> + 'static {
    info!(%url, "Requesting");
    async move {
        let resp = client.get(url).send().await?.error_for_status()?;
        info!(%url,content_type = ?resp.headers().get("content-type"), "Got a response!");
        Ok(())
    }
}

fn setup() -> Result<(), Report> {
    debug!("Setting things up!!");
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    if std::env::var("RUST_BACKTRACE").is_err() {
        std::env::set_var("RUST_BACKTRACE", "info")
    }
    if std::env::var("reqwest").is_err() {
        std::env::set_var("reqwest", "debug")
    }
    tracing_subscriber::fmt::fmt().init();

    Ok(())
}
