use axum::{
    routing::{delete, get, post, put},
    Router,
};

use super::{
    handlers::{create_vendor, delete_vendor, get_vendor, get_vendors, update_vendor},
    models::VendorRepo,
};

pub fn outer<T: VendorRepo>(vendor_repo: T) -> Router {
    Router::new()
        .route("/vendors", get(get_vendors::<T>))
        .route("/vendors", post(create_vendor::<T>))
        .route("/vendors/:id", get(get_vendor::<T>))
        .route("/vendors/:id", put(update_vendor::<T>))
        .route("/vendors/:id", delete(delete_vendor::<T>))
        .with_state(vendor_repo)
}
