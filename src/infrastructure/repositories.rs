use crate::domain::entities::ShortUrl;
use crate::domain::repositories::UrlRepository;
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PostgresUrlRepository {
    pub pool: PgPool,
}

impl PostgresUrlRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UrlRepository for PostgresUrlRepository {
    async fn create(
        &self,
        short_code: &str,
        target_url: &str,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<ShortUrl> {
        let record = sqlx::query_as!(ShortUrl, "INSERT INTO short_urls (short_code, target_url, expires_at) VALUES ($1, $2, $3) RETURNING *", short_code, target_url, expires_at)
            .fetch_one(&self.pool)
            .await?;
        Ok(record)
    }

    async fn find_by_code(&self, code: &str) -> Result<Option<ShortUrl>> {
        let record = sqlx::query_as!(
            ShortUrl,
            "SELECT * FROM short_urls WHERE short_code = $1",
            code
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(record)
    }

    async fn increments_clicks(&self, id: Uuid) -> Result<()> {
        sqlx::query!(
            "UPDATE short_urls SET clicks = clicks + 1 WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
