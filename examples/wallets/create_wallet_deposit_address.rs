use prime_sdk_rs::services::wallets::{CreateWalletDepositAddressRequest, WalletsService};
use prime_sdk_rs::PrimeClient;
use std::env;

/// Usage:
///   ./run_example.sh examples/wallets/create_wallet_deposit_address  # Use PORTFOLIO_ID and WALLET_ID from .env, network_id defaults to base-mainnet
///   ./run_example.sh examples/wallets/create_wallet_deposit_address -- <wallet_id> [network_id]  # Use wallet_id and optionally network_id from CLI
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    let portfolio_id = env::var("PORTFOLIO_ID").map_err(|_| {
        "PORTFOLIO_ID not found in .env file. Please set PORTFOLIO_ID in your .env file."
    })?;

    let mut args = env::args().skip(1);
    let wallet_id = args
        .next()
        .or_else(|| env::var("WALLET_ID").ok())
        .ok_or("WALLET_ID not provided as CLI arg or in .env file.")?;
    let network_id = args.next().unwrap_or_else(|| "base-mainnet".to_string());
    println!("🔑 Using wallet_id: {}", wallet_id);
    println!("🌐 Using network_id: {}", network_id);

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let service = WalletsService::new(prime_client);
    println!(
        "📋 Creating deposit address for wallet {} in portfolio: {}",
        wallet_id, portfolio_id
    );

    let request = CreateWalletDepositAddressRequest::new(portfolio_id, wallet_id, network_id);
    let response = service.create_wallet_deposit_address(request).await;
    match response {
        Ok(resp) => {
            println!("✅ Successfully created wallet deposit address");
            println!("\n🏦 Deposit Address: {:#?}", resp);
        }
        Err(e) => {
            eprintln!("❌ Failed to create wallet deposit address: {}", e);
        }
    }
    Ok(())
}
