use std::io::Error;

use axum::{Router, http::StatusCode, routing::get, serve::Serve};

async fn greet() -> &'static str {
    "Hello, World!\n"
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
pub async fn run() -> Result<Serve<L, M, S>, Error> {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(greet))
        .route("/health_check", get(health_check));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    let serve = axum::serve(listener, app);
    Ok(serve)
}
