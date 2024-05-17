use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Debug, Clone)]
pub struct PostgresRepo {
    pub pool: Pool<Postgres>,
}

impl PostgresRepo {
    pub async fn new(db_url: &str) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
            .unwrap();
        Self { pool }
    }
}
