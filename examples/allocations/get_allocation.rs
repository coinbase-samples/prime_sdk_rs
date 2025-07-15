use prime_sdk_rs::{
    client::PrimeClient,
    services::allocations::{AllocationService, GetAllocationRequest},
};
use std::env;

/// Usage:
///   ./run_example.sh allocations/get_allocation -- <allocation_id>  # Use CLI argument for allocation_id
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    dotenv::dotenv().ok();

    // Get portfolio ID from environment variable (required)
    let portfolio_id = env::var("PORTFOLIO_ID").map_err(|_| {
        "PORTFOLIO_ID not found in .env file. Please set PORTFOLIO_ID in your .env file."
    })?;

    // Get allocation ID from command line argument (required)
    let allocation_id = env::args().nth(1).ok_or(
        "ALLOCATION_ID not provided. Usage: ./run_example.sh allocations/get_allocation -- <allocation_id>"
    )?;

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let allocation_service = AllocationService::new(prime_client);
    println!(
        "📋 Fetching allocation: {} for portfolio: {}",
        allocation_id, portfolio_id
    );

    let request = GetAllocationRequest::new(portfolio_id, allocation_id);

    match allocation_service.get_allocation(request).await {
        Ok(response) => {
            if response.is_some() {
                println!("✅ Successfully retrieved allocation");
                println!("📊 Response: {:#?}", response);
            } else {
                println!("📭 No allocation found");
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to get allocation: {}", e);
        }
    }

    Ok(())
}
