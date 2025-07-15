use prime_sdk_rs::{
    client::PrimeClient,
    services::futures::{FuturesService, GetFuturesSweepsRequest, GetFuturesSweepsResponse},
};
use std::env;

/// Usage:
///   ./run_example.sh futures/get_futures_sweeps
///
/// Requires ENTITY_ID environment variable.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let futures_service = FuturesService::new(prime_client);
    let entity_id = env::var("ENTITY_ID").expect("ENTITY_ID env var required");

    let request = GetFuturesSweepsRequest::new(&entity_id);
    println!("📋 Submitting get_futures_sweeps request: {:#?}", request);
    match futures_service.get_futures_sweeps(request).await {
        Ok(response) => {
            println!("✅ Futures sweeps response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to get futures sweeps: {}", e);
        }
    }
    Ok(())
}
