use crate::postgres::pool::PostgresRepo;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[async_trait]
pub trait VendorRepo: Send + Sync + Clone + 'static {
    async fn get_all(&self) -> Vec<Vendor>;
    async fn create(&self, client: Vendor) -> i64;
    async fn get(&self, id: i64) -> Option<Vendor>;
    async fn update(&self, id: i64, vendor: Vendor) -> ();
    async fn delete(&self, id: i64) -> ();
}

#[async_trait]
impl VendorRepo for PostgresRepo {
    async fn get_all(&self) -> Vec<Vendor> {
        let vendors = sqlx::query_as!(Vendor, "SELECT * FROM vendors")
            .fetch_all(&self.pool)
            .await
            .unwrap();
        vendors
    }

    async fn create(&self, vendor: Vendor) -> i64 {
        let vendor = sqlx::query!(
            "INSERT INTO vendors (client_id, name, email) VALUES ($1, $2, $3) RETURNING id",
        )
        .bind(&vendor.client_id)
        .bind(&vendor.name)
        .bind(&vendor.email)
        .fetch_one(&self.pool)
        .await
        .unwrap();
        vendor.id
    }

    async fn get(&self, id: i64) -> Option<Vendor> {
        let vendor = sqlx::query_as!(Vendor, "SELECT * FROM vendors WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
            .unwrap();
        vendor
    }

    async fn update(&self, id: i64, vendor: Vendor) {
        sqlx::query!(
            "UPDATE vendors SET name = $1, email = $2 WHERE id = $3",
            vendor.name,
            vendor.email,
            id
        )
        .execute(&self.pool)
        .await
        .unwrap();
    }

    async fn delete(&self, id: i64) {
        sqlx::query!("DELETE FROM vendors WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .unwrap();
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, FromRow)]
pub struct Vendor {
    pub id: Option<i64>, // Use i64 to match Postgres BIGSERIAL
    pub client_id: i64,
    pub name: String,
    pub email: String,
}
