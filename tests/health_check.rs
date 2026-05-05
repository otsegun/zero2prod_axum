#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app().await.expect("Failed to spawn our app");

    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute requst.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// launch our application in the background ~somehow~
async fn spawn_app() -> Result<(), std::io::Error> {
    let server = zero2prod_axum::run().await.expect("Failed to bind server");
    let _ = tokio::spawn(server);
}
