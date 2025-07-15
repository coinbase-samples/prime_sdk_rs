use prime_sdk_rs::services::wallets::{ListWalletAddressesRequest, WalletsService};
use prime_sdk_rs::PrimeClient;
use std::env;

/// Usage:
///   ./run_example.sh examples/wallets/list_wallet_addresses  # Use PORTFOLIO_ID and WALLET_ID from .env, network_id defaults to base-mainnet
///   ./run_example.sh examples/wallets/list_wallet_addresses -- <wallet_id> [network_id] [cursor]  # Use wallet_id and optionally network_id/cursor from CLI
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
    let cursor = args.next();
    println!("🔑 Using wallet_id: {}", wallet_id);
    println!("🌐 Using network_id: {}", network_id);
    if let Some(ref c) = cursor {
        println!("➡️  Using cursor: {}", c);
    }

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let service = WalletsService::new(prime_client);
    println!(
        "📋 Fetching wallet addresses for wallet {} in portfolio: {}",
        wallet_id, portfolio_id
    );

    let mut request = ListWalletAddressesRequest::new(portfolio_id, wallet_id, network_id);
    if let Some(c) = cursor {
        request = request.with_cursor(c);
    }
    let response = service.list_wallet_addresses(request).await;
    match response {
        Ok(resp) => {
            println!(
                "✅ Successfully retrieved {} wallet addresses",
                resp.count()
            );
            for (i, address) in resp.addresses().iter().enumerate() {
                println!("\n🏦 Address {}: {:#?}", i + 1, address);
            }
            let pagination = resp.pagination();
            println!("\n📄 Pagination Info:");
            println!("   Has more pages: {}", resp.has_more());
            if let Some(next) = resp.next_cursor() {
                println!("   Next cursor: {}", next);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to list wallet addresses: {}", e);
        }
    }
    Ok(())
}
