use crate::{clients::models::Client, errors::models::AppError, postgres::pool::PostgresRepo};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[async_trait]
pub trait AgentRepo: Send + Sync + Clone + 'static {
    async fn get_all(&self, name: Option<String>) -> Result<Vec<Agent>, AppError>;
    async fn get(&self, id: i64) -> Result<Option<Agent>, AppError>;
    async fn create(&self, agent: Agent) -> Result<i64, AppError>;
    async fn update(&self, id: i64, agent: AgentUpdate) -> Result<(), AppError>;
    async fn delete(&self, id: i64) -> Result<(), AppError>;
    async fn get_clients_for_agent(&self, id: i64) -> Result<Vec<Client>, AppError>;
    async fn add_client_to_agent(&self, agent_id: i64, client_id: i64) -> Result<(), AppError>;
}

#[async_trait]
impl AgentRepo for PostgresRepo {
    async fn get_all(&self, name: Option<String>) -> Result<Vec<Agent>, AppError> {
        let mut query = "SELECT * FROM agents".to_string();
        let mut conditions = Vec::new();

        if let Some(name) = name {
            conditions.push(format!("name LIKE '%{}%'", name));
        }

        if !conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&conditions.join(" AND "));
        }

        let agents = sqlx::query_as::<_, Agent>(&query)
            .fetch_all(&self.pool)
            .await?;

        println!("Query: {}", query);
        Ok(agents)
    }

    async fn get(&self, id: i64) -> Result<Option<Agent>, AppError> {
        let agent = sqlx::query_as!(
            Agent,
            "SELECT id, name, email FROM agents WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(agent)
    }

    async fn create(&self, agent: Agent) -> Result<i64, AppError> {
        let id = sqlx::query!(
            "INSERT INTO agents (name, email) VALUES ($1, $2) RETURNING id",
            agent.name,
            agent.email,
        )
        .fetch_one(&self.pool)
        .await?
        .id;
        Ok(id)
    }

    async fn update(&self, id: i64, agent: AgentUpdate) -> Result<(), AppError> {
        sqlx::query!(
            "UPDATE agents SET name = $1, email = $2 WHERE id = $3",
            agent.name,
            agent.email,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, id: i64) -> Result<(), AppError> {
        sqlx::query!("DELETE FROM agents WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn get_clients_for_agent(&self, id: i64) -> Result<Vec<Client>, AppError> {
        let clients = sqlx::query_as!(
            Client,
            "SELECT clients.id, clients.name, clients.email, clients.bucket
            FROM clients
            INNER JOIN agent_clients ON clients.id = agent_clients.client_id
            WHERE agent_clients.agent_id = $1",
            id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(clients)
    }

    async fn add_client_to_agent(&self, agent_id: i64, client_id: i64) -> Result<(), AppError> {
        sqlx::query!(
            "INSERT INTO agent_clients (agent_id, client_id) VALUES ($1, $2)",
            agent_id,
            client_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, FromRow)]
pub struct Agent {
    id: Option<i64>,
    name: String,
    email: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, FromRow)]
pub struct AgentUpdate {
    name: String,
    email: String,
}
