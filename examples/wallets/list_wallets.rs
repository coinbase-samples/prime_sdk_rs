use prime_sdk_rs::services::{ListWalletsRequest, WalletsService};
use prime_sdk_rs::PrimeClient;
use std::env;

/// Usage:
///   ./run_example.sh examples/wallets/list_wallets  # Use PORTFOLIO_ID from .env
///   ./run_example.sh examples/wallets/list_wallets -- <cursor>  # Use cursor for pagination
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    let portfolio_id = env::var("PORTFOLIO_ID").map_err(|_| {
        "PORTFOLIO_ID not found in .env file. Please set PORTFOLIO_ID in your .env file."
    })?;

    // Accept optional cursor as a command-line argument
    let cursor = env::args().nth(1);
    if let Some(ref c) = cursor {
        println!("🔎 Using cursor: {}", c);
    }

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let service = WalletsService::new(prime_client);
    println!("📋 Fetching wallets for portfolio: {}", portfolio_id);

    let mut request = ListWalletsRequest::new(portfolio_id).with_limit(100);
    if let Some(c) = cursor {
        request = request.with_cursor(c);
    }

    match service.list_wallets(request).await {
        Ok(response) => {
            println!("✅ Successfully retrieved {} wallets", response.count());
            for (i, wallet) in response.wallets().iter().enumerate() {
                println!("\n👛 Wallet {}: {:#?}", i + 1, wallet);
            }
            let pagination = response.pagination();
            println!("\n📄 Pagination Info:");
            println!("   Has more pages: {}", response.has_more());
            if let Some(next) = response.next_cursor() {
                println!("   Next cursor: {}", next);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to list wallets: {}", e);
        }
    }
    Ok(())
}
