use crate::domain::utils::utilities::{
    deserialize_option_datetime, serialize_datetime, serialize_option_datetime,
};
use chrono::{DateTime, Utc};
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Deserialize, ToSchema)]
#[salvo(schema(example = json!({
    "expires_at": "2025-11-05 14:20:30",
    "target_url": "github.com"
})))]
pub struct CreateShortUrlRequest {
    pub target_url: String,
    #[serde(deserialize_with = "deserialize_option_datetime")]
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateUrlResponse {
    pub id: Uuid,
    pub short_code: String,
    pub target_url: String,
    pub clicks: i64,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(serialize_with = "serialize_option_datetime")]
    pub expires_at: Option<DateTime<Utc>>,
}
