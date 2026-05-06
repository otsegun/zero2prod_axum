use std::net::SocketAddr;

use axum::{
    Form, Router,
    http::StatusCode,
    routing::{get, post},
};

// data types
#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

// routes
pub fn app() -> Router {
    Router::new()
        .route("/", get(greet))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
}

// handlers
async fn greet() -> &'static str {
    "Hello, World!\n"
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn subscribe(Form(form_data): Form<FormData>) -> StatusCode {
    StatusCode::OK
}

// entry point for tests, to be called by tests
pub async fn spawn_for_test() -> SocketAddr {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app()).await.unwrap();
    });
    addr
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
