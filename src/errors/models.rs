use axum::response::IntoResponse;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Unknown error")]
    Unknown,
}

// Implement `IntoResponse` for `AppError` to convert it into a proper HTTP response
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(_) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                self.to_string(),
            ),
            AppError::NotFound(_) => (axum::http::StatusCode::NOT_FOUND, self.to_string()),
            AppError::InvalidInput(_) => (axum::http::StatusCode::BAD_REQUEST, self.to_string()),
            AppError::Unknown => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                self.to_string(),
            ),
        };

        (status, error_message).into_response()
    }
}
