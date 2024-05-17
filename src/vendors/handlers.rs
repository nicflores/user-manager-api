use super::models::{Vendor, VendorRepo};
use axum::extract::{Path, State};
use axum::Json;

pub async fn get_vendors<T: VendorRepo>(State(state): State<T>) -> Json<Vec<Vendor>> {
    let vendors = state.get_all().await;
    Json(vendors)
}

pub async fn create_vendor<T: VendorRepo>(
    State(state): State<T>,
    Json(vendor): Json<Vendor>,
) -> Json<i64> {
    let id = state.create(vendor).await;
    Json(id)
}

pub async fn get_vendor<T: VendorRepo>(
    Path(id): Path<i64>,
    State(state): State<T>,
) -> Json<Option<Vendor>> {
    let vendor = state.get(id).await;
    Json(vendor)
}

pub async fn delete_vendor<T: VendorRepo>(Path(id): Path<i64>, State(state): State<T>) -> Json<()> {
    state.delete(id).await;
    Json(())
}

pub async fn update_vendor<T: VendorRepo>(
    State(state): State<T>,
    Path(id): Path<i64>,
    Json(vendor): Json<Vendor>,
) -> Json<()> {
    state.update(id, vendor).await;
    Json(())
}
