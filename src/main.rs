mod config;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let loaded_config = config::Config::new().await;
    println!("{}", loaded_config.token);
    Ok(())
}
