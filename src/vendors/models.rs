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

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, FromRow)]
pub struct Vendor {
    pub id: i64, // Use i64 to match Postgres BIGSERIAL
    pub vendors: Vec<String>,
    pub sftp: String,
    pub name: String,
    pub email: String,
}
