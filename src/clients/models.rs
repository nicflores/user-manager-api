use crate::{
    errors::models::AppError,
    postgres::pool::PostgresRepo,
    sftp::models::{SftpResponse, SftpUpdate},
    utils::ssh::SSHKeyPair,
    vendors::models::Vendor,
};
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
    async fn add_vendor(&self, client_id: i64, vendor: Vendor) -> Result<i64, AppError>;
    async fn update_vendor(
        &self,
        client_id: i64,
        vendor_id: i64,
        vendor: Vendor,
    ) -> Result<(), AppError>;
    async fn add_sftp(&self, client_id: i64, sftp: SftpUpdate) -> Result<SftpResponse, AppError>;
    async fn reset_keys(&self, client_id: i64) -> Result<SSHKeyPair, AppError>;
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
            "INSERT INTO clients (name, email, bucket) VALUES ($1, $2, $3) RETURNING id",
            client.name,
            client.email,
            client.bucket,
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
            "UPDATE clients SET name = $1, email = $2, bucket = $3 WHERE id = $4",
            client.name,
            client.email,
            client.bucket,
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

    async fn add_vendor(&self, client_id: i64, vendor: Vendor) -> Result<i64, AppError> {
        // Check if the client exists
        let client_exists: bool = sqlx::query!("SELECT id FROM clients WHERE id = $1", client_id)
            .fetch_optional(&self.pool)
            .await?
            .is_some();

        if !client_exists {
            return Err(AppError::NotFound(format!(
                "Client with id {} not found",
                client_id
            )));
        }

        // Insert Vendor
        let vendor_id = sqlx::query!(
                "INSERT INTO vendors (client_id, name, host, port, username, password, ssh_key, ssh_key_password) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id",
                client_id,
                vendor.name,
                vendor.host,
                vendor.port,
                vendor.username,
                vendor.password,
                vendor.ssh_key,
                vendor.ssh_key_password
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
            "UPDATE vendors SET
                name = $1,
                host = $2,
                port = $3,
                username = $4,
                password = $5,
                ssh_key = $6,
                ssh_key_password = $7
             WHERE client_id = $8 AND id = $9",
            vendor.name,
            vendor.host,
            vendor.port,
            vendor.username,
            vendor.password,
            vendor.ssh_key,
            vendor.ssh_key_password,
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

    async fn add_sftp(&self, client_id: i64, sftp: SftpUpdate) -> Result<SftpResponse, AppError> {
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

        // Generate SSH key pair.
        let ssh_keys = SSHKeyPair::new();

        let sftp_id = sqlx::query!(
            "INSERT INTO sftp (client_id, username, private_key, public_key, bucket_name, aws_role_arn) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id",
            client_id,
            sftp.username,
            ssh_keys.private_key,
            ssh_keys.public_key,
            sftp.bucket_name,
            sftp.aws_role_arn,
        )
        .fetch_one(&self.pool)
        .await?
        .id;

        let response = SftpResponse {
            id: sftp_id,
            client_id,
            private_key: ssh_keys.private_key,
            public_key: ssh_keys.public_key,
        };

        Ok(response)
    }

    async fn reset_keys(&self, client_id: i64) -> Result<SSHKeyPair, AppError> {
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

        // Generate SSH key pair.
        let ssh_keys = SSHKeyPair::new();

        let rows_affected = sqlx::query!(
            "UPDATE sftp SET private_key = $1, public_key = $2 WHERE client_id = $3",
            ssh_keys.private_key,
            ssh_keys.public_key,
            client_id,
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            Err(AppError::NotFound(format!(
                "SFTP keys for client with id {} not found",
                client_id
            )))
        } else {
            Ok(ssh_keys)
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, FromRow)]
pub struct Client {
    pub id: Option<i64>,
    pub name: String,
    pub email: String,
    pub bucket: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, FromRow)]
pub struct ClientBucket {
    pub id: i64,
    pub bucket: String,
}
