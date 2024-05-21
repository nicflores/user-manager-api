#![crate_name = "user_manager_api"]

mod agents;
mod clients;
mod errors;
mod postgres;
mod sftp;
mod shutdown;
mod utils;
mod vendors;

use crate::postgres::pool::PostgresRepo;
use crate::shutdown::shutdown_signal;
use crate::utils::auth::auth;

use axum::middleware;
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use std::future::IntoFuture;
use std::sync::Arc;
use tower::ServiceBuilder;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use user_manager_api::config::models::AppConfig;
use user_manager_api::{config, health};

/// The main function is the entry point of the application.
/// TODO: We still need to enable the OpenTelemetry layer to send traces to Honeycomb.io.
#[tokio::main]
async fn main() {
    // Load the application configuration.
    let cfg = AppConfig::new().unwrap();

    // Setup the tracing subscriber to log to stdout.
    // TODO: Enable OpenTelemetry tracing to Honeycomb.io.
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .unwrap();

    // Setup the Postgres connection pool.
    let pg_pool = PostgresRepo::new(&cfg.database_url).await;

    // Setup the routers for the various parts of the application.
    let client_router = clients::app::router(pg_pool.clone());
    let vendor_router = vendors::app::router(pg_pool.clone());
    let sftp_router = sftp::app::router(pg_pool.clone());
    let agent_router = agents::app::router(pg_pool);
    let config_router = config::app::router(cfg.clone());
    let health_router = health::app::router();

    // Setup the auth layer.
    let token = Arc::new(cfg.api_key.clone());
    let auth_layer = ServiceBuilder::new()
        .layer(middleware::from_fn(move |req, next| {
            let token = token.clone();
            async move { auth(req, next, token).await }
        }))
        .into_inner();

    // Merge all the routers into a single app.
    let app = client_router
        .merge(vendor_router)
        .merge(sftp_router)
        .merge(agent_router)
        .merge(config_router)
        .merge(health_router)
        .layer(OtelInResponseLayer::default())
        .layer(OtelAxumLayer::default())
        .layer(auth_layer);

    // Create a TCP listener and serve the app on the listener.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    // This is the main event loop that listens for incoming requests.
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .into_future()
        .await
        .unwrap()
}
