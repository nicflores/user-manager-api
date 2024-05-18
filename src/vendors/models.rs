use crate::{errors::models::AppError, postgres::pool::PostgresRepo};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[async_trait]
pub trait VendorRepo: Send + Sync + Clone + 'static {
    async fn get_all(
        &self,
        client_id: Option<i64>,
        name: Option<String>,
    ) -> Result<Vec<Vendor>, AppError>;
    async fn get(&self, id: i64) -> Result<Option<Vendor>, AppError>;
    async fn update(&self, id: i64, vendor: Vendor) -> Result<(), AppError>;
    async fn delete(&self, id: i64) -> Result<(), AppError>;
}

#[async_trait]
impl VendorRepo for PostgresRepo {
    async fn get_all(
        &self,
        client_id: Option<i64>,
        name: Option<String>,
    ) -> Result<Vec<Vendor>, AppError> {
        let mut query = "SELECT id, client_id, name, url FROM vendors".to_string();
        let mut conditions = Vec::new();

        if let Some(client_id) = client_id {
            conditions.push(format!("client_id = {}", client_id));
        }

        if let Some(name) = name {
            conditions.push(format!("name LIKE '%{}%'", name));
        }

        if !conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&conditions.join(" AND "));
        }

        let vendors = sqlx::query_as::<_, Vendor>(&query)
            .fetch_all(&self.pool)
            .await?;

        Ok(vendors)
    }

    async fn get(&self, id: i64) -> Result<Option<Vendor>, AppError> {
        let vendor = sqlx::query_as!(
            Vendor,
            "SELECT id, client_id, name, email FROM vendors WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(vendor)
    }

    async fn update(&self, id: i64, vendor: Vendor) -> Result<(), AppError> {
        let rows_affected = sqlx::query!(
            "UPDATE vendors SET client_id = $1, name = $2, email = $3 WHERE id = $4",
            vendor.client_id,
            vendor.name,
            vendor.email,
            id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            Err(AppError::NotFound(format!(
                "Vendor with id {} not found",
                id
            )))
        } else {
            Ok(())
        }
    }

    async fn delete(&self, id: i64) -> Result<(), AppError> {
        let rows_affected = sqlx::query!("DELETE FROM vendors WHERE id = $1", id)
            .execute(&self.pool)
            .await?
            .rows_affected();

        if rows_affected == 0 {
            Err(AppError::NotFound(format!(
                "Vendor with id {} not found",
                id
            )))
        } else {
            Ok(())
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, FromRow)]
pub struct Vendor {
    pub id: i64, // Use i64 to match Postgres BIGSERIAL
    pub client_id: i64,
    pub name: String,
    pub email: String,
}
