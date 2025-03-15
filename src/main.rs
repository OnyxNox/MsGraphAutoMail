use std::error;

/// Application entry point.
#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let ip_address = reqwest::get("https://api.ipify.org").await?.text().await?;

    println!("Hello, world! My IP address is {}", ip_address);

    Ok(())
}
