use std::net::SocketAddr;

use axum::{Router, http::StatusCode, routing::get};

async fn greet() -> &'static str {
    "Hello, World!\n"
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
pub async fn run() -> SocketAddr {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(greet))
        .route("/health_check", get(health_check));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    // get addr
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    addr
}
