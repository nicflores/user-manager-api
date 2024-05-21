use super::{
    handlers::{
        add_client_to_agent, create_agent, delete_agent, get_agent, get_agents,
        get_clients_for_agent, update_agent,
    },
    models::AgentRepo,
};
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn router<T: AgentRepo>(repo: T) -> Router {
    Router::new()
        .route("/agents", get(get_agents::<T>))
        .route("/agents/:id", get(get_agent::<T>))
        .route("/agents", post(create_agent::<T>))
        .route("/agents/:id", put(update_agent::<T>))
        .route("/agents/:id", delete(delete_agent::<T>))
        .route("/agents/:id/clients", get(get_clients_for_agent::<T>))
        .route("/agents/:id/clients/:id", put(add_client_to_agent::<T>))
        .with_state(repo)
}
