use super::models::{SftpOverview, SftpRepo, SftpUpdate};
use crate::errors::models::AppError;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use std::collections::HashMap;

pub async fn get_sftp<T: SftpRepo>(
    State(repo): State<T>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<SftpOverview>>, AppError> {
    let client_id = params.get("client_id").and_then(|id| id.parse().ok());
    let sftps = repo.get_all(client_id).await?;
    Ok(Json(sftps))
}

pub async fn get_sftp_by_id<T: SftpRepo>(
    State(repo): State<T>,
    Path(id): Path<i64>,
) -> Result<Json<SftpOverview>, AppError> {
    match repo.get(id).await? {
        Some(sftp) => Ok(Json(SftpOverview {
            id: sftp.id,
            client_id: sftp.client_id,
            username: sftp.username,
            bucket_name: sftp.bucket_name,
            aws_role_arn: sftp.aws_role_arn,
        })),
        None => Err(AppError::NotFound(format!("Sftp with id {} not found", id))),
    }
}

pub async fn update_sftp<T: SftpRepo>(
    State(repo): State<T>,
    Path(id): Path<i64>,
    Json(update): Json<SftpUpdate>,
) -> Result<(), AppError> {
    repo.update(id, update).await?;
    Ok(())
}

pub async fn delete_sftp<T: SftpRepo>(
    State(repo): State<T>,
    Path(id): Path<i64>,
) -> Result<(), AppError> {
    repo.delete(id).await?;
    Ok(())
}
