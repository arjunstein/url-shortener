use crate::domain::repositories::UrlRepository;
use crate::infrastructure::database::db_pool;
use crate::infrastructure::repositories::PostgresUrlRepository;
use std::env;
use std::sync::Arc;
use tokio::time::{Duration, sleep};

pub fn start_cleanup_scheduler() {
    tokio::spawn(async move {
        let pool = db_pool().clone();
        let repo = Arc::new(PostgresUrlRepository::new(pool));
        let interval_secs: u64 = env::var("CLEANUP_INTERVAL_SECS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(60);

        loop {
            match repo.delete_expired_url().await {
                Ok(count) if count > 0 => {
                    tracing::info!("🧹 Auto-cleaned {count} expired short URLs");
                }
                Ok(_) => {}
                Err(e) => tracing::error!("Cleanup error: {:?}", e),
            }

            sleep(Duration::from_secs(interval_secs)).await;
        }
    });
}
