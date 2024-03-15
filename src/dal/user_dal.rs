use crate::dal::UserDal;
use crate::models::User;

use anyhow::Result;
use async_trait::async_trait;
use aws_config::BehaviorVersion;
use aws_sdk_secretsmanager::client::Client;
use aws_sdk_secretsmanager::error::SdkError;
use aws_sdk_secretsmanager::Error;

pub struct AwsSecretsManagerUserDal {
    client: Client,
}

impl AwsSecretsManagerUserDal {
    pub async fn new() -> Result<Self, SdkError<Error>> {
        let config = aws_config::load_defaults(BehaviorVersion::v2023_11_09()).await;
        let client = Client::new(&config);
        Ok(AwsSecretsManagerUserDal { client })
    }
}

#[async_trait]
impl UserDal for AwsSecretsManagerUserDal {
    async fn create_user(&self, user: User) -> Result<()> {
        let secret_name = format!("user/{}", user.username);
        let secret_value = serde_json::to_string(&user)?;
        self.client
            .create_secret()
            .name(secret_name)
            .secret_string(secret_value)
            .send()
            .await?;
        Ok(())
    }

    async fn get_user(&self, username: &String) -> Result<User> {
        let secret_name = format!("user/{}", username);
        let resp = self
            .client
            .get_secret_value()
            .secret_id(secret_name)
            .send()
            .await?
            .secret_string
            .ok_or_else(|| anyhow::Error::msg("Secret not found or has no string value"))?;
        let user: User = serde_json::from_str(&resp)?;
        Ok(user)
    }

    async fn update_user(&self, user: User) -> Result<()> {
        let secret_name = format!("user/{}", user.username);
        let secret_value = serde_json::to_string(&user)?;
        self.client
            .update_secret()
            .secret_id(secret_name)
            .secret_string(secret_value)
            .send()
            .await?;
        Ok(())
    }

    async fn delete_user(&self, username: &String) -> Result<()> {
        let secret_name = format!("user/{}", username);
        self.client
            .delete_secret()
            .secret_id(secret_name)
            .force_delete_without_recovery(true)
            .send()
            .await?;
        Ok(())
    }
}
