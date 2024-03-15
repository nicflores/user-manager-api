use std::sync::Arc;

use crate::{
    api::handlers::{create_user, delete_user, get_user, update_user},
    service::UserService,
};
use axum::{
    routing::{delete, get, post, put},
    Extension, Router,
};

/// Setup and return user-related routes.
pub fn get_user_routes(user_service: Arc<UserService>) -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route("/users/:username", get(get_user))
        .route("/users/:username", put(update_user))
        .route("/users/:username", delete(delete_user))
        .layer(Extension(user_service))
}
