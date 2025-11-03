use crate::domain::entities::ShortUrl;
use anyhow::Result;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait UrlRepository: Send + Sync {
    async fn create(
        &self,
        short_code: &str,
        target_url: &str,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<ShortUrl>;
    async fn find_by_code(&self, code: &str) -> Result<Option<ShortUrl>>;
    async fn increments_clicks(&self, id: Uuid) -> Result<()>;
    async fn get_all_url(&self) -> Result<Vec<ShortUrl>>;
    async fn delete_expired_url(&self) -> Result<u64>;
}
