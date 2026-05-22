use env_logger::Env;
use sqlx::PgPool;
use zero2prod_axum::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Setup env_logger: `init` does call `set_logger`, so this is all we need to do.
    // We are falling back to printing all logs at info-level or above
    // if the RUST_LOG environment variable has not been set.
    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();

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
