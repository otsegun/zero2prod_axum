use axum::{
    Router,
    routing::{get, post},
};
use tokio::net::TcpListener;

use crate::routes::{health_check, subscribe};

// routes
pub fn app() -> Router {
    Router::new()
        // .route("/", axum::routing::get(greet))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
}

// entry point for running app normally, to be called by main
pub async fn run(listener: TcpListener) -> Result<(), std::io::Error> {
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app()).await
}
