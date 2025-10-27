use anyhow::Result;
use once_cell::sync::OnceCell;
use sqlx::postgres::{PgPool, PgPoolOptions};

pub static DB_POOL: OnceCell<PgPool> = OnceCell::new();

pub async fn init_db_pool(database_url: &str) -> Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    DB_POOL.set(pool).expect("DB_POOL is already initialized");
    Ok(())
}

pub fn db_pool() -> &'static PgPool {
    DB_POOL.get().expect("DB_POOL is not initialized")
}
