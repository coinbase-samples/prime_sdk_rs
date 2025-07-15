use prime_sdk_rs::client::PrimeClient;
use prime_sdk_rs::services::futures::{CancelFuturesSweepRequest, FuturesService};
use std::env;

/// Usage:
///   ./run_example.sh futures/cancel_futures_sweep
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

    let request = CancelFuturesSweepRequest::new(&entity_id);
    println!("📋 Submitting cancel_futures_sweep request: {:#?}", request);
    match futures_service.cancel_futures_sweep(request).await {
        Ok(response) => {
            println!("✅ Cancel futures sweep response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to cancel futures sweep: {}", e);
        }
    }
    Ok(())
}
