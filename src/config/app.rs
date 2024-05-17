use std::sync::Arc;

use axum::{routing::get, Extension, Router};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};

use super::{handlers::config, models::AppConfig};

/// The router for the health check endpoint.
/// ```
/// GET /config
/// ```
/// Returns:
/// ```json
/// {"database_url": ""postgresql://postgres:postgres@localhost:5432/postgres"",
/// "api_key": "*****",
/// "log_level": "info"}
/// ```
///
pub fn router(app_config: AppConfig) -> Router {
    let app_config = Arc::new(app_config);
    Router::new()
        .route("/config", get(config))
        .layer(Extension(app_config))
        .layer(OtelInResponseLayer::default())
        .layer(OtelAxumLayer::default())
}
