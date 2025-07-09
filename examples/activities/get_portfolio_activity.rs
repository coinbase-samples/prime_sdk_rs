use prime_rs_sdk::{
    client::PrimeClient,
    services::activities::{ActivitiesService, GetPortfolioActivityRequest},
};
use std::env;

/// Usage:
///   ./run_example.sh activities/get_portfolio_activity -- <activity_id>  # Use command line argument
///   ./run_example.sh activities/get_portfolio_activity                    # Use PORTFOLIO_ID and ACTIVITY_ID from .env
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    // Load .env file
    dotenv::dotenv().ok();

    // Get portfolio ID from environment variable (required)
    let portfolio_id = env::var("PORTFOLIO_ID").map_err(|_| {
        "PORTFOLIO_ID environment variable is required. Please set it in your .env file."
    })?;

    // Get activity ID from command line argument or environment variable (required)
    let activity_id = if let Some(arg) = env::args().nth(1) {
        // Use command line argument if provided
        arg
    } else {
        // Fall back to environment variable
        env::var("ACTIVITY_ID").map_err(|_| {
            "ACTIVITY_ID not provided as command line argument or found in .env file. \
            Usage: ./run_example.sh activities/get_portfolio_activity -- <activity_id> \
            or set ACTIVITY_ID in your .env file."
        })?
    };

    println!("🚀 Creating PrimeClient...");

    // Create a new PrimeClient with default configuration
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    // Create the activities service
    let activities_service = ActivitiesService::new(prime_client);

    println!("📋 Fetching activity: {}", activity_id);

    // Create request object
    let request = GetPortfolioActivityRequest::new(portfolio_id, activity_id);

    // Example: Get a specific activity by ID
    match activities_service.get_portfolio_activity(request).await {
        Ok(response) => {
            println!("✅ Successfully retrieved activity");

            let activity = response.activity;
            println!("\n📋 Activity Details: {:#?}", activity);
        }
        Err(e) => {
            eprintln!("❌ Failed to get activity: {}", e);
        }
    }

    Ok(())
}
