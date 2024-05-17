use super::{
    handlers::{create_client, delete_client, get_client, get_clients, update_client},
    models::ClientRepo,
};
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn router<T: ClientRepo>(client_repo: T) -> Router {
    Router::new()
        .route("/clients", get(get_clients::<T>))
        .route("/clients", post(create_client::<T>))
        .route("/clients/:id", get(get_client::<T>))
        .route("/clients/:id", put(update_client::<T>))
        .route("/clients/:id", delete(delete_client::<T>))
        .route(path: "/clients/:id/vendor", post(add_vendor_to_client::<T>))
        .with_state(client_repo)
}
