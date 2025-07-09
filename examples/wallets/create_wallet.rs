use prime_rs_sdk::services::{CreateWalletRequest, WalletsService};
use prime_rs_sdk::types::generated::generated::wallet_type::WalletType;
use prime_rs_sdk::PrimeClient;
use std::env;
use uuid::Uuid;

/// Usage:
///   ./run_example.sh examples/wallets/create_wallet  # Use PORTFOLIO_ID from .env
///   ./run_example.sh examples/wallets/create_wallet -- ETH  # Use ETH as symbol
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    let portfolio_id = env::var("PORTFOLIO_ID").map_err(|_| {
        "PORTFOLIO_ID not found in .env file. Please set PORTFOLIO_ID in your .env file."
    })?;

    // Accept symbol as a command-line argument, default to ETH
    let symbol = env::args().nth(1).unwrap_or_else(|| "ETH".to_string());
    println!("🪙 Using symbol: {}", symbol);

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let service = WalletsService::new(prime_client);
    println!("📝 Creating a new wallet in portfolio: {}", portfolio_id);

    let request = CreateWalletRequest::new(format!("My {} Wallet", symbol), symbol.clone())
        .with_wallet_type(WalletType::Vault);

    match service.create_wallet(portfolio_id, request).await {
        Ok(response) => {
            println!("✅ Successfully created wallet");
            println!("\n👛 Wallet creation response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to create wallet: {}", e);
        }
    }
    Ok(())
}
