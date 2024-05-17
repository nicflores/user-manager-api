use super::models::Health;
use axum::Json;

/// The health check endpoint handler.
/// Currently it simply returns a Health struct with a status of "OK".
pub async fn health() -> Json<Health> {
    let health_stats = Health {
        status: "OK".to_string(),
    };
    Json(health_stats)
}
