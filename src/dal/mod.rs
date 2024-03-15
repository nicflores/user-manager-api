mod user_dal;

// Trait defining operations for managing users.
use async_trait::async_trait;
pub use user_dal::AwsSecretsManagerUserDal;

#[async_trait]
pub trait UserDal: Sync + Send {
    async fn create_user(&self, user: crate::models::User) -> anyhow::Result<()>;
    async fn get_user(&self, username: &String) -> anyhow::Result<crate::models::User>;
    async fn update_user(&self, user: crate::models::User) -> anyhow::Result<()>;
    async fn delete_user(&self, username: &String) -> anyhow::Result<()>;
}
