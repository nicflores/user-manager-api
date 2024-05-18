use super::{
    handlers::{
        add_vendor_to_client, create_client, delete_client, get_client, get_clients, update_client,
        update_vendor,
    },
    models::ClientRepo,
};
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn router<T: ClientRepo>(repo: T) -> Router {
    Router::new()
        .route("/clients", post(create_client::<T>))
        .route("/clients", get(get_clients::<T>))
        .route("/clients/:id", get(get_client::<T>))
        .route("/clients/:id", put(update_client::<T>))
        .route("/clients/:id", delete(delete_client::<T>))
        .route("/clients/:id/vendor", post(add_vendor_to_client::<T>))
        .route("/clients/:id/vendor/:id", put(update_vendor::<T>))
        .with_state(repo)
}
