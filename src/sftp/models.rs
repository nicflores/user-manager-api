use crate::{errors::models::AppError, postgres::pool::PostgresRepo};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Postgres, Transaction};

#[async_trait]
pub trait SftpRepo: Send + Sync + Clone + 'static {
    async fn get_all(&self, client_id: Option<i64>) -> Result<Vec<SftpOverview>, AppError>;
    async fn get(&self, id: i64) -> Result<Option<SftpOverview>, AppError>;
    async fn update(&self, id: i64, sftp: SftpUpdate) -> Result<(), AppError>;
    async fn delete(&self, id: i64) -> Result<(), AppError>;
}

#[async_trait]
impl SftpRepo for PostgresRepo {
    async fn get_all(&self, client_id: Option<i64>) -> Result<Vec<SftpOverview>, AppError> {
        let sftps = match client_id {
            Some(client_id) => {
                sqlx::query_as!(
                    SftpOverview,
                    "SELECT id, client_id, username, bucket_name, aws_role_arn FROM sftp WHERE client_id = $1",
                    client_id
                )
                .fetch_all(&self.pool)
                .await?
            },
            None => {
                sqlx::query_as!(
                    SftpOverview,
                    "SELECT id, client_id, username, bucket_name, aws_role_arn FROM sftp"
                )
                .fetch_all(&self.pool)
                .await?
            }
        };
        Ok(sftps)
    }

    async fn get(&self, id: i64) -> Result<Option<SftpOverview>, AppError> {
        let sftp = sqlx::query_as!(
            SftpOverview,
            "SELECT id, client_id, username, bucket_name, aws_role_arn FROM sftp WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(sftp)
    }

    async fn update(&self, id: i64, sftp: SftpUpdate) -> Result<(), AppError> {
        let mut tx: Transaction<'_, Postgres> = self.pool.begin().await?;

        if let Some(username) = sftp.username {
            sqlx::query!("UPDATE sftp SET username = $1 WHERE id = $2", username, id)
                .execute(&mut *tx)
                .await?;
        }

        if let Some(bucket_name) = sftp.bucket_name {
            sqlx::query!(
                "UPDATE sftp SET bucket_name = $1 WHERE id = $2",
                bucket_name,
                id
            )
            .execute(&mut *tx)
            .await?;
        }

        if let Some(aws_role_arn) = sftp.aws_role_arn {
            sqlx::query!(
                "UPDATE sftp SET aws_role_arn = $1 WHERE id = $2",
                aws_role_arn,
                id
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    async fn delete(&self, id: i64) -> Result<(), AppError> {
        let rows_affected = sqlx::query!("DELETE FROM sftp WHERE id = $1", id)
            .execute(&self.pool)
            .await?
            .rows_affected();

        if rows_affected == 0 {
            Err(AppError::NotFound(format!("Sftp with id {} not found", id)))
        } else {
            Ok(())
        }
    }
}

/// This struct represents the SFTP details for a client.
/// These detaisl are for the multiuser SFTP server we stood up in AWS.
/// It's the service that connects clients to their S3 buckets.
/// This information will more likely be needed by the SFTP transfer service
/// identity provider lambda.
///
/// Currently the sftp server is located at:
/// ```bash
/// sftp.prep.api.tpv.ntrs.com
/// ```
/// Each SFTP object is associated with a client vi the client_id field.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, FromRow)]
struct Sftp {
    pub id: Option<i64>,
    pub client_id: i64,
    pub username: String,
    pub private_key: Option<String>,
    pub public_key: Option<String>,
    pub bucket_name: String,
    pub aws_role_arn: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SftpOverview {
    pub id: i64,
    pub client_id: i64,
    pub username: String,
    pub bucket_name: String,
    pub aws_role_arn: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SftpUpdate {
    pub username: Option<String>,
    pub bucket_name: Option<String>,
    pub aws_role_arn: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SftpResponse {
    pub id: i64,
    pub client_id: i64,
    pub private_key: String,
    pub public_key: String,
}
