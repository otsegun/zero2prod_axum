use sqlx::PgPool;
use zero2prod_axum::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    // Use port application port from our settings
    let address = format!("127.0.0.1:{}", configuration.application_port);
    // create listener
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    run(listener, connection_pool).await?;
    Ok(())
}
