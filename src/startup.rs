use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;
use tokio::net::TcpListener;

use crate::routes::{health_check, subscribe};

// routes
pub fn app(db_pool: PgPool) -> Router {
    Router::new()
        // .route("/", axum::routing::get(greet))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(db_pool)
}

// entry point for running app normally, to be called by main
pub async fn run(listener: TcpListener, db_pool: PgPool) -> Result<(), std::io::Error> {
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app(db_pool)).await
}
