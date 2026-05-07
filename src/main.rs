use zero2prod_axum::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    // Use port application port from our settings

    let address = format!("127.0.0.1:{}", configuration.application_port);
    // create listener
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    run(listener).await?;
    Ok(())
}
