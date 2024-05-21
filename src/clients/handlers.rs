use super::models::{Client, ClientRepo};
use crate::{
    errors::models::AppError,
    sftp::models::{SftpResponse, SftpUpdate},
    vendors::models::Vendor,
};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use std::collections::HashMap;

/// The get clients endpoint handler. Returns a list of all clients as JSON.
pub async fn get_clients<T: ClientRepo>(
    State(repo): State<T>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Client>>, AppError> {
    let name_filter = params.get("name").cloned();
    let email_filter = params.get("email").cloned();

    let clients = repo.get_all(name_filter, email_filter).await?;
    Ok(Json(clients))
}

pub async fn create_client<T: ClientRepo>(
    State(repo): State<T>,
    Json(client): Json<Client>,
) -> Result<Json<i64>, AppError> {
    let client_id = repo.create(client).await?;
    Ok(Json(client_id))
}

pub async fn get_client<T: ClientRepo>(
    State(repo): State<T>,
    Path(id): Path<i64>,
) -> Result<Json<Client>, AppError> {
    match repo.get(id).await? {
        Some(client) => Ok(Json(client)),
        None => Err(AppError::NotFound(format!(
            "Client with id {} not found",
            id
        ))),
    }
}

pub async fn update_client<T: ClientRepo>(
    State(repo): State<T>,
    Path(id): Path<i64>,
    Json(client): Json<Client>,
) -> Result<(), AppError> {
    repo.update(id, client).await?;
    Ok(())
}

pub async fn delete_client<T: ClientRepo>(
    State(repo): State<T>,
    Path(id): Path<i64>,
) -> Result<(), AppError> {
    repo.delete(id).await?;
    Ok(())
}

pub async fn add_vendor_to_client<T: ClientRepo>(
    State(repo): State<T>,
    Path(client_id): Path<i64>,
    Json(vendor): Json<Vendor>,
) -> Result<Json<i64>, AppError> {
    let vendor_id = repo.add_vendor(client_id, vendor).await?;
    Ok(Json(vendor_id))
}

pub async fn update_vendor<T: ClientRepo>(
    State(repo): State<T>,
    Path((client_id, vendor_id)): Path<(i64, i64)>,
    Json(vendor): Json<Vendor>,
) -> Result<(), AppError> {
    repo.update_vendor(client_id, vendor_id, vendor).await?;
    Ok(())
}

pub async fn add_sftp<T: ClientRepo>(
    State(repo): State<T>,
    Path(client_id): Path<i64>,
    Json(sftp): Json<SftpUpdate>,
) -> Result<Json<SftpResponse>, AppError> {
    let sftp_response = repo.add_sftp(client_id, sftp).await?;
    Ok(Json(sftp_response))
}

pub async fn reset_keys<T: ClientRepo>(
    State(repo): State<T>,
    Path(client_id): Path<i64>,
) -> Result<(), AppError> {
    repo.reset_keys(client_id).await?;
    Ok(())
}
