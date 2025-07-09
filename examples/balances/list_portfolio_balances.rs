use prime_rs_sdk::{
    client::PrimeClient,
    services::balances::{BalancesService, ListPortfolioBalancesRequest},
};
use std::env;

/// Usage:
///   ./run_example.sh balances/list_portfolio_balances -- <portfolio_id>  # Use command line argument
///   ./run_example.sh balances/list_portfolio_balances                    # Use PORTFOLIO_ID from .env
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    dotenv::dotenv().ok();

    // Get portfolio ID from command line argument or environment variable (required)
    let portfolio_id = if let Some(arg) = env::args().nth(1) {
        // Use command line argument if provided
        arg
    } else {
        // Fall back to environment variable
        env::var("PORTFOLIO_ID").map_err(|_| {
            "PORTFOLIO_ID not provided as command line argument or found in .env file. \
            Usage: ./run_example.sh balances/list_portfolio_balances -- <portfolio_id> \
            or set PORTFOLIO_ID in your .env file."
        })?
    };

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let balances_service = BalancesService::new(prime_client);
    println!("📋 Fetching portfolio balances for: {}", portfolio_id);

    let request = ListPortfolioBalancesRequest::new(portfolio_id).with_limit(10);

    match balances_service.list_portfolio_balances(request).await {
        Ok(response) => {
            println!("✅ Successfully retrieved portfolio balances");
            println!("Total balances: {}", response.count());

            for balance in response.items() {
                println!(
                    "  - Symbol: {}, Available: {}",
                    balance.symbol.as_deref().unwrap_or("N/A"),
                    balance.available_balance.as_deref().unwrap_or("N/A")
                );
            }

            // Note: Portfolio balances don't support pagination
            if !response.has_more() {
                println!("📄 All portfolio balances retrieved (no pagination)");
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to get portfolio balances: {}", e);
        }
    }

    Ok(())
}
