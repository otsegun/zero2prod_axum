use zero2prod_axum::{app, run};

#[tokio::main]
// async fn main() -> () {
//     // TODO(segun): main doesn't block anymore, fix this.
//     let _addr = run().await;
//     // format!("http://{}", addr)
// }

async fn main() -> Result<(), std::io::Error> {
    run().await?;
    Ok(())
}
