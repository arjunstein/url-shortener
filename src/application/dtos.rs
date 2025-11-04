use crate::domain::utils::utilities::{
    deserialize_option_datetime, serialize_datetime, serialize_option_datetime,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Deserialize)]
pub struct CreateShortUrlRequest {
    pub target_url: String,
    #[serde(deserialize_with = "deserialize_option_datetime")]
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct CreateUrlResponse {
    pub id: Uuid,
    pub short_code: String,
    pub target_url: String,
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    pub clicks: i64,
    #[serde(serialize_with = "serialize_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(serialize_with = "serialize_option_datetime")]
    pub expires_at: Option<DateTime<Utc>>,
}
