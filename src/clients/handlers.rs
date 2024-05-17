use std::collections::HashMap;

use crate::{errors::models::AppError, vendors::models::Vendor};

use super::models::{Client, ClientRepo};
use axum::{
    extract::{Path, Query, State},
    Json,
};

/// The get clients endpoint handler. Returns a list of all clients as JSON.
pub async fn get_clients<T: ClientRepo>(State(state): State<T>) -> Json<Vec<Client>> {
    let clients = state.get_all().await;
    Json(clients)
}

pub async fn create_client<T: ClientRepo>(
    State(state): State<T>,
    Json(client): Json<Client>,
) -> Json<i64> {
    let id = state.create(client).await;
    Json(id)
}

pub async fn get_client<T: ClientRepo>(
    Path(id): Path<i64>,
    State(state): State<T>,
) -> Json<Option<Client>> {
    let client = state.get(id).await;
    client
}

pub async fn delete_client<T: ClientRepo>(Path(id): Path<i64>, State(state): State<T>) -> Json<()> {
    state.delete(id).await;
    Json(())
}

pub async fn update_client<T: ClientRepo>(
    State(state): State<T>,
    Path(id): Path<i64>,
    Json(client): Json<Client>,
) -> Json<()> {
    state.update(id, client).await;
    Json(())
}

async fn get_all_clients<T: ClientRepo>(
    State(repo): State<T>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Client>>, AppError> {
    let name_filter = params.get("name").cloned();
    let email_filter = params.get("email").cloned();

    let clients = repo.get_all(name_filter, email_filter).await?;
    Ok(Json(clients))
}

async fn update_vendor<T: ClientRepo>(
    State(repo): State<T>,
    Path((client_id, vendor_id)): Path<(i64, i64)>,
    Json(vendor): Json<Vendor>,
) -> Result<(), AppError> {
    repo.update_vendor(client_id, vendor_id, vendor).await?;
    Ok(())
}
