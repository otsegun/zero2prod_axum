use axum::{
    Router,
    routing::{get, post},
};

use crate::routes::{health_check, subscribe};

// routes
pub fn app() -> Router {
    Router::new()
        // .route("/", axum::routing::get(greet))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
}

// entry point for running app normally, to be called by main
pub async fn run() -> Result<(), std::io::Error> {
    // create listener
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app()).await
}
