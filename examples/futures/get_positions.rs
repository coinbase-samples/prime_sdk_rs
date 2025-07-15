use prime_sdk_rs::{
    client::PrimeClient,
    services::futures::{FuturesService, GetPositionsRequest, GetPositionsResponse},
};
use std::env;

/// Usage:
///   ./run_example.sh futures/get_positions [product_id]
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
    let product_id = env::args().nth(1);

    let mut request = GetPositionsRequest::new(&entity_id);
    if let Some(pid) = product_id {
        request = request.with_product_id(pid);
    }
    println!("📋 Submitting get_positions request: {:#?}", request);
    match futures_service.get_positions(request).await {
        Ok(response) => {
            println!("✅ Positions response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to get positions: {}", e);
        }
    }
    Ok(())
}
