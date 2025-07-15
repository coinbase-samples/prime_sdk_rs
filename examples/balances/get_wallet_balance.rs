use prime_sdk_rs::{
    client::PrimeClient,
    services::balances::{BalancesService, GetWalletBalanceRequest},
};
use std::env;

/// Usage:
///   ./run_example.sh balances/get_wallet_balance -- <wallet_id>  # Use command line argument for wallet_id
///   ./run_example.sh balances/get_wallet_balance                  # Use WALLET_ID from .env
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    // Load .env file
    dotenv::dotenv().ok();

    // Get portfolio ID from environment variable (required)
    let portfolio_id = env::var("PORTFOLIO_ID").map_err(|_| {
        "PORTFOLIO_ID not found in .env file. Please set PORTFOLIO_ID in your .env file."
    })?;

    // Get wallet ID from command line argument or environment variable (required)
    let wallet_id = if let Some(arg) = env::args().nth(1) {
        // Use command line argument if provided
        arg
    } else {
        // Fall back to environment variable
        env::var("WALLET_ID").map_err(|_| {
            "WALLET_ID not provided as command line argument or found in .env file. \
            Usage: ./run_example.sh balances/get_wallet_balance -- <wallet_id> \
            or set WALLET_ID in your .env file."
        })?
    };

    println!("🚀 Creating PrimeClient...");

    // Create a new PrimeClient with default configuration
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    // Create the balances service
    let balances_service = BalancesService::new(prime_client);

    println!(
        "📋 Fetching wallet balance for portfolio: {} wallet: {}",
        portfolio_id, wallet_id
    );

    // Create the request
    let request = GetWalletBalanceRequest::new(portfolio_id, wallet_id);

    // Example: Get wallet balance
    match balances_service.get_wallet_balance(request).await {
        Ok(response) => {
            println!("✅ Successfully retrieved wallet balance");
            println!("\n📋 Wallet Balance: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to get wallet balance: {}", e);
        }
    }

    Ok(())
}
