use prime_sdk_rs::{
    client::PrimeClient,
    services::allocations::{AllocationService, ListPortfolioAllocationsRequest},
};
use std::env;

/// Usage:
///   ./run_example.sh allocations/list_portfolio_allocations  # Use PORTFOLIO_ID from .env
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    dotenv::dotenv().ok();

    // Get portfolio ID from environment variable (required)
    let portfolio_id = env::var("PORTFOLIO_ID").map_err(|_| {
        "PORTFOLIO_ID not found in .env file. Please set PORTFOLIO_ID in your .env file."
    })?;

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let allocation_service = AllocationService::new(prime_client);
    println!("📋 Fetching allocations for portfolio: {}", portfolio_id);

    // Use a default start date
    let start_date = "2025-01-01T00:00:00Z";
    let request = ListPortfolioAllocationsRequest::new(portfolio_id, start_date);

    match allocation_service.list_portfolio_allocations(request).await {
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
