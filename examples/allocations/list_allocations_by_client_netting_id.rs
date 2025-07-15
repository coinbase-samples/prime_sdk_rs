use prime_sdk_rs::{
    client::PrimeClient,
    services::allocations::{AllocationService, ListAllocationsByClientNettingIdRequest},
};
use std::env;

/// Usage:
///   ./run_example.sh allocations/list_allocations_by_client_netting_id  # Use PORTFOLIO_ID and NETTING_ID from .env
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    dotenv::dotenv().ok();

    // Get portfolio ID from environment variable (required)
    let portfolio_id = env::var("PORTFOLIO_ID").map_err(|_| {
        "PORTFOLIO_ID not found in .env file. Please set PORTFOLIO_ID in your .env file."
    })?;

    // Get netting ID from environment variable (required)
    let netting_id = env::var("NETTING_ID").map_err(|_| {
        "NETTING_ID not found in .env file. Please set NETTING_ID in your .env file."
    })?;

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let allocation_service = AllocationService::new(prime_client);
    println!(
        "📋 Fetching allocations for portfolio: {} with netting ID: {}",
        portfolio_id, netting_id
    );

    let request = ListAllocationsByClientNettingIdRequest::new(portfolio_id, netting_id);

    match allocation_service
        .list_allocations_by_client_netting_id(request)
        .await
    {
        Ok(response) => {
            println!("✅ Successfully retrieved {} allocations", response.count());
            println!("📊 Response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to get allocations: {}", e);
        }
    }

    Ok(())
}
