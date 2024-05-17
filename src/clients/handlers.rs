use super::models::{Client, ClientRepo};
use axum::{
    extract::{Path, State},
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
    Json(client)
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
