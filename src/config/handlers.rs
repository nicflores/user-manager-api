use std::sync::Arc;

use axum::{Extension, Json};

use super::models::AppConfig;

/// The config endpoint handler.
pub async fn config(Extension(config): Extension<Arc<AppConfig>>) -> Json<AppConfig> {
    Json(config.as_ref().clone())
}
