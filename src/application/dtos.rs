use chrono::{DateTime, Utc};
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateShortUrlRequest {
    pub target_url: String,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateUrlResponse {
    pub id: Uuid,
    pub short_code: String,
    pub target_url: String,
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    pub clicks: i64,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}
