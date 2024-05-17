use axum::{
    routing::{delete, get, post, put},
    Router,
};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};

use super::{
    handlers::{create_client, delete_client, get_client, get_clients, update_client},
    models::ClientRepo,
};

pub fn router<T: ClientRepo>(client_repo: T) -> Router {
    Router::new()
        .route("/clients", get(get_clients::<T>))
        .route("/clients", post(create_client::<T>))
        .route("/clients/:id", get(get_client::<T>))
        .route("/clients/:id", put(update_client::<T>))
        .route("/clients/:id", delete(delete_client::<T>))
        .with_state(client_repo)
        .layer(OtelInResponseLayer::default())
        .layer(OtelAxumLayer::default())
}
