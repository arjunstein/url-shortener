use dotenvy::dotenv;
use infrastructure::database::init_db_pool;
use presentation::routes::router;
use salvo::prelude::*;
use std::env;
use tracing_subscriber;

use crate::infrastructure::scheduler::start_cleanup_scheduler;

mod application;
mod domain;
mod infrastructure;
mod presentation;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt().init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    init_db_pool(&database_url)
        .await
        .expect("Failed to init DB Pool");
    let acceptor = TcpListener::new("0.0.0.0:5800").bind().await;
    let router = router();

    start_cleanup_scheduler();

    println!("{:?}", router);
    Server::new(acceptor).serve(router).await;
}
