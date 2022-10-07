#![feature(async_closure)]
use std::net::SocketAddr;

use axum::{routing::any, Router};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod proxy;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "frontman=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config_file =
        std::fs::read_to_string("frontman.toml").expect("Should have been able to read the file");
    let config: config::Config = toml::from_str(&config_file).unwrap();

    tracing::debug!("Starting server with configuration: {:?}", config);

    let proxy = proxy::Proxy::new(config);

    let handler = async move |path| match proxy.handler(path).await {
        Ok(response) => response,
        Err(_) => "error".to_owned(),
    };

    let app = Router::new()
        .route("/*path", any(handler))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("Server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
