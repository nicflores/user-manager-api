use crate::{errors::models::AppError, postgres::pool::PostgresRepo, vendors::models::Vendor};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Interface to the client database table.
/// Supports all CRUD operations.
/// ```
/// get_all: Get all clients.
/// create: Create a new client.
/// get: Get a client by id.
/// update: Update a client by id.
/// delete: Delete a client by id.
/// ```
#[async_trait]
pub trait ClientRepo: Send + Sync + Clone + 'static {
    async fn get_all(
        &self,
        name_filter: Option<String>,
        email_filter: Option<String>,
    ) -> Result<Vec<Client>, AppError>;
    async fn create(&self, client: Client) -> Result<i64, AppError>;
    async fn get(&self, id: i64) -> Result<Option<Client>, AppError>;
    async fn update(&self, id: i64, client: Client) -> Result<(), AppError>;
    async fn delete(&self, id: i64) -> Result<(), AppError>;
    async fn add_vendor_to_client(&self, client_id: i64, vendor: Vendor) -> Result<i64, AppError>;
    async fn update_vendor(
        &self,
        client_id: i64,
        vendor_id: i64,
        vendor: Vendor,
    ) -> Result<(), AppError>;
}

#[async_trait]
impl ClientRepo for PostgresRepo {
    async fn get_all(
        &self,
        name_filter: Option<String>,
        email_filter: Option<String>,
    ) -> Result<Vec<Client>, AppError> {
        let mut clients = sqlx::query_as!(Client, "SELECT * FROM clients")
            .fetch_all(&self.pool)
            .await?;

        if let Some(name) = name_filter {
            clients.retain(|client| client.name.contains(&name));
        }

        if let Some(email) = email_filter {
            clients.retain(|client| client.email.contains(&email));
        }

        Ok(clients)
    }

    async fn create(&self, client: Client) -> Result<i64, AppError> {
        let client_id = sqlx::query!(
            "INSERT INTO clients (name, email) VALUES ($1, $2) RETURNING id",
            client.name,
            client.email,
        )
        .fetch_one(&self.pool)
        .await?
        .id;

        Ok(client_id)
    }

    async fn get(&self, id: i64) -> Result<Option<Client>, AppError> {
        let client = sqlx::query_as!(Client, "SELECT * FROM clients WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await?;

        match client {
            Some(client) => Ok(Some(client)),
            None => Err(AppError::NotFound(format!(
                "Client with id {} not found",
                id
            ))),
        }
    }

    async fn update(&self, id: i64, client: Client) -> Result<(), AppError> {
        let rows_affected = sqlx::query!(
            "UPDATE clients SET name = $1, email = $2 WHERE id = $3",
            client.name,
            client.email,
            id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            Err(AppError::NotFound(format!(
                "Client with id {} not found",
                id
            )))
        } else {
            Ok(())
        }
    }

    async fn delete(&self, id: i64) -> Result<(), AppError> {
        let rows_affected = sqlx::query!("DELETE FROM clients WHERE id = $1", id)
            .execute(&self.pool)
            .await?
            .rows_affected();

        if rows_affected == 0 {
            Err(AppError::NotFound(format!(
                "Client with id {} not found",
                id
            )))
        } else {
            Ok(())
        }
    }

    async fn add_vendor_to_client(&self, client_id: i64, vendor: Vendor) -> Result<i64, AppError> {
        // For some reason sqlx doesn't like SELECT 1.
        let client_exists: bool = sqlx::query!("SELECT * FROM clients WHERE id = $1", client_id)
            .fetch_optional(&self.pool)
            .await?
            .is_some();

        if !client_exists {
            return Err(AppError::NotFound(format!(
                "Client with id {} not found",
                client_id
            )));
        }

        let vendor_id = sqlx::query!(
            "INSERT INTO vendors (client_id, name, email) VALUES ($1, $2, $3) RETURNING id",
            client_id,
            vendor.name,
            vendor.email,
        )
        .fetch_one(&self.pool)
        .await?
        .id;

        Ok(vendor_id)
    }

    async fn update_vendor(
        &self,
        client_id: i64,
        vendor_id: i64,
        vendor: Vendor,
    ) -> Result<(), AppError> {
        let rows_affected = sqlx::query!(
            "UPDATE vendors SET name = $1, email = $2 WHERE client_id = $3 AND id = $4",
            vendor.name,
            vendor.email,
            client_id,
            vendor_id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            Err(AppError::NotFound(format!(
                "Vendor with id {} for client with id {} not found",
                vendor_id, client_id
            )))
        } else {
            Ok(())
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, FromRow)]
pub struct Client {
    pub id: i64, // Use i64 to match Postgres BIGSERIAL
    pub name: String,
    pub email: String,
}
