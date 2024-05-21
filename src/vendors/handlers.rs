use std::collections::HashMap;

use crate::errors::models::AppError;

use super::models::{Vendor, VendorOverview, VendorRepo};
use axum::extract::{Path, Query, State};
use axum::Json;

pub async fn get_vendors<T: VendorRepo>(
    State(repo): State<T>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Vendor>>, AppError> {
    let client_id_filter = params.get("client_id").and_then(|id| id.parse().ok());
    let name_filter = params.get("name").cloned();

    let vendors = repo.get_all(client_id_filter, name_filter).await?;
    Ok(Json(vendors))
}

pub async fn get_vendor<T: VendorRepo>(
    State(repo): State<T>,
    Path(id): Path<i64>,
) -> Result<Json<VendorOverview>, AppError> {
    match repo.get(id).await? {
        Some(vendor) => Ok(Json(vendor)),
        None => Err(AppError::NotFound(format!(
            "Vendor with id {} not found",
            id
        ))),
    }
}

pub async fn update_vendor<T: VendorRepo>(
    State(repo): State<T>,
    Path(id): Path<i64>,
    Json(vendor): Json<Vendor>,
) -> Result<(), AppError> {
    repo.update(id, vendor).await?;
    Ok(())
}

pub async fn delete_vendor<T: VendorRepo>(
    State(repo): State<T>,
    Path(id): Path<i64>,
) -> Result<(), AppError> {
    repo.delete(id).await?;
    Ok(())
}
