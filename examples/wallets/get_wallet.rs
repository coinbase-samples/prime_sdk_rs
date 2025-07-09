use prime_rs_sdk::services::{GetWalletRequest, WalletsService};
use prime_rs_sdk::PrimeClient;
use std::env;

/// Usage:
///   ./run_example.sh examples/wallets/get_wallet  # Use PORTFOLIO_ID and WALLET_ID from .env
///   ./run_example.sh examples/wallets/get_wallet -- <wallet_id>  # Use wallet_id from CLI
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    let portfolio_id = env::var("PORTFOLIO_ID").map_err(|_| {
        "PORTFOLIO_ID not found in .env file. Please set PORTFOLIO_ID in your .env file."
    })?;

    // Accept wallet_id as a command-line argument, fallback to env
    let wallet_id = env::args()
        .nth(1)
        .or_else(|| env::var("WALLET_ID").ok())
        .ok_or("WALLET_ID not provided as CLI arg or in .env file.")?;
    println!("🔑 Using wallet_id: {}", wallet_id);

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let service = WalletsService::new(prime_client);
    println!(
        "📋 Fetching wallet {} for portfolio: {}",
        wallet_id, portfolio_id
    );

    let request = GetWalletRequest::new(portfolio_id, wallet_id);
    match service.get_wallet(request).await {
        Ok(response) => {
            println!("✅ Successfully retrieved wallet");
            println!("\n👛 Wallet: {:#?}", response.wallet);
        }
        Err(e) => {
            eprintln!("❌ Failed to get wallet: {}", e);
        }
    }
    Ok(())
}
