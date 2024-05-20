#![crate_name = "user_manager_api"]

mod clients;
mod errors;
mod postgres;
mod sftp;
mod shutdown;
mod utils;
mod vendors;

use crate::postgres::pool::PostgresRepo;
use crate::shutdown::shutdown_signal;

use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use std::future::IntoFuture;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use user_manager_api::config::models::AppConfig;
use user_manager_api::{config, health};

/// The main function is the entry point of the application.
/// It sets up the application configuration.
/// Additionally it setups up a tracing subscriber that only logs to stdout, currently.
/// Next it sets up the application routes and merges the various routes into a single app.
/// Then a TCP listener is created and the app is served on the listener.
/// TODO: We still need to enable the OpenTelemetry layer to send traces to Honeycomb.io.
/// TODO: make the IP and Port configurable, but for the most part I don't think it really needs to change.
#[tokio::main]
async fn main() {
    let cfg = AppConfig::new().unwrap();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .unwrap();

    let pg_pool = PostgresRepo::new(&cfg.database_url).await;

    let client_router = clients::app::router(pg_pool.clone());
    let vendor_router = vendors::app::router(pg_pool.clone());
    let sftp_router = sftp::app::router(pg_pool);
    let config_router = config::app::router(cfg);
    let health_router = health::app::router();

    let app = client_router
        .merge(vendor_router)
        .merge(sftp_router)
        .merge(config_router)
        .merge(health_router)
        .layer(OtelInResponseLayer::default())
        .layer(OtelAxumLayer::default());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .into_future()
        .await
        .unwrap()
}
