use crate::application::dtos::{CreateShortUrlRequest, CreateUrlResponse};
use crate::domain::repositories::UrlRepository;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use chrono::Utc;
use rand::Rng;
use rand::distributions::Alphanumeric;
use std::sync::Arc;

#[async_trait]
pub trait UrlService: Send + Sync {
    async fn create_short_url(&self, req: CreateShortUrlRequest) -> Result<CreateUrlResponse>;
    async fn get_target_url(&self, short_code: &str) -> Result<Option<String>>;
    async fn get_all_urls(&self) -> Result<Vec<CreateUrlResponse>>;
    async fn delete_url(&self, code: &str) -> Result<(), anyhow::Error>;
}

pub struct UrlServiceImpl<R: UrlRepository> {
    repo: Arc<R>,
}

impl<R: UrlRepository> UrlServiceImpl<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    fn generate_short_code(&self) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect()
    }
}

#[async_trait]
impl<R: UrlRepository> UrlService for UrlServiceImpl<R> {
    async fn create_short_url(&self, req: CreateShortUrlRequest) -> Result<CreateUrlResponse> {
        let code = self.generate_short_code();

        let entity = self
            .repo
            .create(&code, &req.target_url, req.expires_at)
            .await?;

        Ok(CreateUrlResponse {
            id: entity.id,
            short_code: entity.short_code,
            target_url: entity.target_url,
            clicks: entity.clicks,
            created_at: entity.created_at,
            expires_at: entity.expires_at,
        })
    }

    async fn get_target_url(&self, short_code: &str) -> Result<Option<String>> {
        if let Some(url) = self.repo.find_by_code(short_code).await? {
            // check expired url
            if let Some(exp) = url.expires_at {
                if Utc::now() > exp {
                    let expired_local = exp
                        .with_timezone(&chrono::Local)
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string();

                    return Err(anyhow!(format!("EXPIRED:{}", expired_local)));
                }
            }

            self.repo.increments_clicks(url.id).await?;
            return Ok(Some(url.target_url));
        }

        Ok(None)
    }

    async fn get_all_urls(&self) -> Result<Vec<CreateUrlResponse>> {
        let urls = self.repo.get_all_url().await?;
        Ok(urls
            .into_iter()
            .map(|url| CreateUrlResponse {
                id: url.id,
                short_code: url.short_code,
                target_url: url.target_url,
                clicks: url.clicks,
                created_at: url.created_at,
                expires_at: url.expires_at,
            })
            .collect())
    }

    async fn delete_url(&self, code: &str) -> Result<(), anyhow::Error> {
        self.repo.delete_by_code(code).await?;
        Ok(())
    }
}
