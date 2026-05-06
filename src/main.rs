use zero2prod_axum::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run().await?;
    Ok(())
}
