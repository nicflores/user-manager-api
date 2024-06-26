use super::{
    handlers::{delete_vendor, get_vendor, get_vendors, update_vendor},
    models::VendorRepo,
};
use axum::{
    routing::{delete, get, put},
    Router,
};

pub fn router<T: VendorRepo>(repo: T) -> Router {
    Router::new()
        .route("/vendors", get(get_vendors::<T>))
        .route("/vendors/:id", get(get_vendor::<T>))
        .route("/vendors/:id", put(update_vendor::<T>))
        .route("/vendors/:id", delete(delete_vendor::<T>))
        .with_state(repo)
}
