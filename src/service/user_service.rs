use crate::models::User;
use crate::{api::handlers::UserInput, dal::UserDal};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct CreateUserInput {
    pub username: String,
    pub bucket_name: String,
    pub public_ssh_key: String,
    pub aws_role_arn: String,
}

impl From<UserInput> for CreateUserInput {
    fn from(input: UserInput) -> Self {
        CreateUserInput {
            username: input.username,
            bucket_name: input.bucket_name,
            public_ssh_key: input.public_ssh_key,
            aws_role_arn: input.aws_role_arn,
        }
    }
}

pub struct UserService {
    user_dal: Arc<dyn UserDal>, // Use dynamic dispatch for flexibility
}

impl UserService {
    // Dependency injection for the DAL, allowing for easy swapping
    pub fn new(user_dal: Arc<dyn UserDal>) -> Self {
        UserService { user_dal }
    }

    pub async fn create_user(&self, input: CreateUserInput) -> Result<()> {
        // Validate input, generate any additional required data (e.g., SSH keys), and call the DAL
        let user = User {
            username: input.username,
            bucket_name: input.bucket_name,
            public_ssh_key: input.public_ssh_key,
            aws_role_arn: input.aws_role_arn,
        };

        self.user_dal.create_user(user).await?;
        Ok(())
    }

    pub async fn get_user(&self, username: String) -> Result<User> {
        self.user_dal.get_user(&username).await
    }

    pub async fn update_user(&self, user: User) -> Result<()> {
        self.user_dal.update_user(user).await
    }

    pub async fn delete_user(&self, username: String) -> Result<()> {
        self.user_dal.delete_user(&username).await
    }
}
