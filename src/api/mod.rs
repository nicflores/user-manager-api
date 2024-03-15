pub mod handlers;
pub mod routes;

use axum::routing::Router;
use std::sync::Arc;

use self::routes::get_user_routes;
use crate::service::UserService;

/// Initializes and returns the API routes as a Router.
pub fn init_api(user_service: Arc<UserService>) -> Router {
    Router::new().merge(get_user_routes(user_service))
}
