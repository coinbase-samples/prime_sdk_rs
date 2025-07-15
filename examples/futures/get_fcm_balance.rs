use prime_sdk_rs::{
    client::PrimeClient,
    services::futures::{FuturesService, GetFcmBalanceRequest},
};
use std::env;

/// Usage:
///   ./run_example.sh futures/get_fcm_balance
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

    let request = GetFcmBalanceRequest::new(&entity_id);
    println!("📋 Submitting get_fcm_balance request: {:#?}", request);
    match futures_service.get_fcm_balance(request).await {
        Ok(response) => {
            println!("✅ FCM Balance response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to get FCM balance: {}", e);
        }
    }
    Ok(())
}
