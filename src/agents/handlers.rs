use super::models::{Agent, AgentRepo, AgentUpdate};
use crate::{clients::models::Client, errors::models::AppError};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use std::collections::HashMap;

pub async fn get_agents<T: AgentRepo>(
    State(repo): State<T>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Agent>>, AppError> {
    let client_id = params.get("client_id").and_then(|id| id.parse().ok());
    let agents = repo.get_all(client_id).await?;
    Ok(Json(agents))
}

pub async fn get_agent<T: AgentRepo>(
    State(repo): State<T>,
    Path(id): Path<i64>,
) -> Result<Json<Agent>, AppError> {
    match repo.get(id).await? {
        Some(agent) => Ok(Json(agent)),
        None => Err(AppError::NotFound(format!(
            "Agent with id {} not found",
            id
        ))),
    }
}

pub async fn create_agent<T: AgentRepo>(
    State(repo): State<T>,
    Json(agent): Json<Agent>,
) -> Result<Json<i64>, AppError> {
    let agent_id = repo.create(agent).await?;
    Ok(Json(agent_id))
}

pub async fn update_agent<T: AgentRepo>(
    State(repo): State<T>,
    Path(id): Path<i64>,
    Json(agent): Json<AgentUpdate>,
) -> Result<(), AppError> {
    repo.update(id, agent).await?;
    Ok(())
}

pub async fn delete_agent<T: AgentRepo>(
    State(repo): State<T>,
    Path(id): Path<i64>,
) -> Result<(), AppError> {
    repo.delete(id).await?;
    Ok(())
}

pub async fn get_clients_for_agent<T: AgentRepo>(
    State(repo): State<T>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<Client>>, AppError> {
    let clients = repo.get_clients_for_agent(id).await?;
    Ok(Json(clients))
}

pub async fn add_client_to_agent<T: AgentRepo>(
    State(repo): State<T>,
    Path((agent_id, client_id)): Path<(i64, i64)>,
) -> Result<(), AppError> {
    repo.add_client_to_agent(agent_id, client_id).await?;
    Ok(())
}
