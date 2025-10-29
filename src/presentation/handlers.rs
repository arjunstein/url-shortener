use crate::application::dtos::CreateShortUrlRequest;
use crate::application::services::{UrlService, UrlServiceImpl};
use crate::infrastructure::{database::db_pool, repositories::PostgresUrlRepository};
use salvo::http::header::{HeaderName, HeaderValue};
use salvo::prelude::*;
use serde_json::json;
use std::sync::Arc;
use tracing;

#[endpoint(
    tags("URL Shortener"),
    summary = "Create short URL",
    description = "Generate a short URL from a given target URL",
    request_body(
        content = CreateShortUrlRequest,
        description = "Payload for creating a short URL"
    )
)]
pub async fn create_short_handler(req: &mut Request, res: &mut Response) {
    // Parse JSON body
    let body: CreateShortUrlRequest = match req.parse_json().await {
        Ok(b) => b,
        Err(_) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(json!({"error": "invalid request body"})));
            return;
        }
    };

    // Build repo + service from global pool (fallback approach)
    let pool = db_pool().clone();
    let repo = PostgresUrlRepository::new(pool);
    let svc = UrlServiceImpl::new(Arc::new(repo));

    // Call service
    match svc.create_short_url(body).await {
        Ok(resp) => {
            res.status_code(StatusCode::CREATED);
            res.render(Json(resp));
        }
        Err(e) => {
            tracing::error!("create_short error: {:?}", e);
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(json!({"error": format!("{}", e)})));
        }
    }
}

#[handler]
pub async fn redirect_handler(req: &mut Request, res: &mut Response) {
    let code = req.param("code").unwrap_or("").to_owned();

    if code.is_empty() {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render("code param missing");
        return;
    }

    let pool = db_pool().clone();
    let repo = PostgresUrlRepository::new(pool);
    let svc = UrlServiceImpl::new(Arc::new(repo));

    match svc.get_target_url(&code).await {
        Ok(Some(target)) => match HeaderValue::from_str(&target) {
            Ok(val) => {
                tracing::info!("Redirecting to: {}, using code: {}", target, code);
                res.status_code(StatusCode::TEMPORARY_REDIRECT);
                res.headers_mut()
                    .insert(HeaderName::from_static("location"), val);
            }
            Err(_) => {
                tracing::warn!("Invalid redirect location in DB: {}", target);
                res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                res.render("Invalid stored target URL");
            }
        },
        Ok(None) => {
            tracing::error!("Not found resource: {}", code);
            res.status_code(StatusCode::NOT_FOUND);
            res.render("Not Found");
        }
        Err(e) => {
            tracing::error!("redirect error: {:?}", e);
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render("Internal Server Error");
        }
    }
}
