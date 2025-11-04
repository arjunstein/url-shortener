use crate::application::dtos::CreateShortUrlRequest;
use crate::application::services::{UrlService, UrlServiceImpl};
use crate::domain::validators::url_validator::normalize_url;
use crate::infrastructure::{database::db_pool, repositories::PostgresUrlRepository};
use salvo::http::header::{HeaderName, HeaderValue};
use salvo::prelude::*;
use serde_json::json;
use std::sync::Arc;
use tracing;

#[handler]
pub async fn create_short_handler(req: &mut Request, res: &mut Response) {
    // Parse JSON body
    let mut body: CreateShortUrlRequest = match req.parse_json().await {
        Ok(b) => b,
        Err(_) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(json!({"error": "invalid request body"})));
            return;
        }
    };

    let url = match normalize_url(&body.target_url) {
        Ok(u) => u,
        Err(msg) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(json!({ "error": msg })));
            return;
        }
    };

    // Normalized URL
    body.target_url = url.to_string();

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
            let msg = e.to_string();
            if msg.starts_with("EXPIRED:") {
                tracing::error!("Expired URL: {}", code);
                let exp = msg.trim_start_matches("EXPIRED:");
                res.status_code(StatusCode::GONE);
                res.render(Json(json!({
                    "error": "url expired",
                    "expired_at": exp
                })));
            } else {
                tracing::error!("internal server error: {}", msg);
                res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                res.render(Json(json!({
                    "error": "internal server error"
                })));
            }
        }
    }
}

#[handler]
pub async fn get_all_handler(_req: &mut Request, res: &mut Response) {
    let pool = db_pool().clone();
    let repo = PostgresUrlRepository::new(pool);
    let svc = UrlServiceImpl::new(Arc::new(repo));

    match svc.get_all_urls().await {
        Ok(list) => {
            res.status_code(StatusCode::OK);
            res.render(Json(list));
        }
        Err(e) => {
            tracing::error!("get_all error: {:?}", e);
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(json!({"error": "internal server error"})));
        }
    }
}
