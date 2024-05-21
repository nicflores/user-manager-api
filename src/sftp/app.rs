use super::{
    handlers::{delete_sftp, get_sftp, get_sftp_by_id, update_sftp},
    models::SftpRepo,
};
use axum::{
    routing::{delete, get, put},
    Router,
};

pub fn router<T: SftpRepo>(repo: T) -> Router {
    Router::new()
        .route("/sftp", get(get_sftp::<T>))
        .route("/sftp/:id", get(get_sftp_by_id::<T>))
        .route("/sftp/:id", put(update_sftp::<T>))
        .route("/sftp/:id", delete(delete_sftp::<T>))
        .with_state(repo)
}
