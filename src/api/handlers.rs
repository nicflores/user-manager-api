use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
    Extension,
};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{models::User, service::UserService};

#[derive(Serialize, Deserialize)]
pub struct UserInput {
    pub username: String,
    pub bucket_name: String,
    pub public_ssh_key: String,
    pub aws_role_arn: String,
}

impl From<UserInput> for User {
    fn from(input: UserInput) -> Self {
        User {
            username: input.username,
            bucket_name: input.bucket_name,
            public_ssh_key: input.public_ssh_key,
            aws_role_arn: input.aws_role_arn,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserOutput {
    username: String,
    bucket_name: String,
    public_ssh_key: String,
    aws_role_arn: String,
}

// Inject your service layer with dependency injection or global state if necessary.
// These examples assume a simplified synchronous interaction for demonstration.

#[debug_handler]
pub async fn create_user(
    Extension(user_service): Extension<Arc<UserService>>,
    Json(payload): Json<UserInput>,
) -> impl IntoResponse {
    // Now you can use `user_service` to interact with the user service layer
    let result = user_service.create_user(payload.into()).await;

    match result {
        Ok(_) => (StatusCode::CREATED, Json("User created successfully")).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response(),
    }
}

pub async fn get_user(
    Extension(user_service): Extension<Arc<UserService>>,
    Path(username): Path<String>,
) -> impl IntoResponse {
    match user_service.get_user(username).await {
        Ok(user) => {
            // Convert your user data to the UserOutput model if necessary
            let user_output = UserOutput {
                username: user.username,
                bucket_name: user.bucket_name,
                public_ssh_key: user.public_ssh_key,
                aws_role_arn: user.aws_role_arn,
            };
            (StatusCode::OK, Json(user_output)).into_response()
        }
        Err(e) => {
            // Handle error, e.g., user not found or internal error
            // This simplistic error handling should be expanded based on your application's needs
            (StatusCode::NOT_FOUND, Json(e.to_string())).into_response()
            //.into_response()
        }
    }
}

pub async fn update_user(
    Extension(user_service): Extension<Arc<UserService>>,
    Json(payload): Json<UserInput>,
) -> impl IntoResponse {
    // Call service layer to update user
    let result = user_service.update_user(payload.into()).await;
    match result {
        Ok(_) => (StatusCode::ACCEPTED, Json("User updated successfully")).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response(),
    }
}

pub async fn delete_user(
    Extension(user_service): Extension<Arc<UserService>>,
    Path(username): Path<String>,
) -> impl IntoResponse {
    match user_service.delete_user(username).await {
        Ok(_) => (StatusCode::OK, Json("User deleted successfully")).into_response(),
        Err(e) => {
            // Handle different error types accordingly. This is a generic catch-all response.
            (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response()
        }
    }
}
