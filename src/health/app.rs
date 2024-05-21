use super::handlers::health;
use axum::{routing::get, Router};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};

/// The router for the health check endpoint.
/// ```
/// GET /health
/// ```
/// Returns:
/// ```json
/// {"status": "OK"}
///```
///
pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
        .layer(OtelInResponseLayer::default())
        .layer(OtelAxumLayer::default())
}
