use prime_rs_sdk::{
    client::PrimeClient,
    services::portfolios::{GetPortfolioRequest, PortfoliosService},
};
use std::env;

/// Usage:
///   ./run_example.sh portfolios/get_portfolio -- <portfolio_id>  # Use command line argument
///   ./run_example.sh portfolios/get_portfolio                    # Use PORTFOLIO_ID from .env
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    dotenv::dotenv().ok();

    // Get portfolio ID from CLI or env
    let portfolio_id = if let Some(arg) = env::args().nth(1) {
        arg
    } else {
        env::var("PORTFOLIO_ID").map_err(|_| {
            "PORTFOLIO_ID not provided as command line argument or found in .env file. \
            Usage: ./run_example.sh portfolios/get_portfolio -- <portfolio_id> \
            or set PORTFOLIO_ID in your .env file."
        })?
    };

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let portfolios_service = PortfoliosService::new(prime_client);
    println!("📋 Fetching portfolio: {}", portfolio_id);

    // Create request object
    let request = GetPortfolioRequest::new(portfolio_id);

    match portfolios_service.get_portfolio(request).await {
        Ok(response) => {
            println!("✅ Successfully retrieved portfolio");
            println!("\n📋 Portfolio: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to get portfolio: {}", e);
        }
    }

    Ok(())
}
