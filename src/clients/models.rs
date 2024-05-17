use std::env;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, FromRow, Pool, Postgres};

#[derive(Debug, Clone)]
pub struct ClientRepoPostgres {
    pool: Pool<Postgres>,
}

impl ClientRepoPostgres {
    pub async fn new() -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();
        Self { pool }
    }
}

/// Interface to the client database table.
/// Supports all CRUD operations.
/// ```
/// get_all: Get all clients.
/// create: Create a new client.
/// get: Get a client by id.
/// update: Update a client by id.
/// delete: Delete a client by id.
/// ```
///
#[async_trait]
pub trait ClientRepo: Send + Sync + Clone + 'static {
    async fn get_all(&self) -> Vec<Client>;
    async fn create(&self, client: Client) -> i64;
    async fn get(&self, id: i64) -> Option<Client>;
    async fn update(&self, id: i64, client: Client) -> ();
    async fn delete(&self, id: i64) -> ();
}

#[async_trait]
impl ClientRepo for ClientRepoPostgres {
    async fn get_all(&self) -> Vec<Client> {
        let clients = sqlx::query_as!(Client, "SELECT * FROM clients")
            .fetch_all(&self.pool)
            .await
            .unwrap();
        clients
    }

    async fn create(&self, client: Client) -> i64 {
        let client = sqlx::query_as::<_, Client>(
            "INSERT INTO clients (name, email, vendors, sftp) VALUES ($1, $2, $3, $4) RETURNING id",
        )
        .bind(&client.name)
        .bind(&client.email)
        .bind(&client.vendors)
        .bind(&client.sftp)
        .fetch_one(&self.pool)
        .await
        .unwrap();
        client.id
    }

    async fn get(&self, id: i64) -> Option<Client> {
        let client = sqlx::query_as!(Client, "SELECT * FROM clients WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
            .unwrap();
        client
    }

    async fn update(&self, id: i64, client: Client) {
        sqlx::query(
            "UPDATE clients SET name = $1, email = $2, vendors = $3, sftp = $4 WHERE id = $5",
        )
        .bind(&client.name)
        .bind(&client.email)
        .bind(&client.vendors)
        .bind(&client.sftp)
        .bind(id)
        .execute(&self.pool)
        .await
        .unwrap();
    }

    async fn delete(&self, id: i64) {
        sqlx::query("DELETE FROM clients WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .unwrap();
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, FromRow)]
pub struct Client {
    pub id: i64, // Use i64 to match Postgres BIGSERIAL
    pub vendors: Vec<String>,
    pub sftp: String,
    pub name: String,
    pub email: String,
}
