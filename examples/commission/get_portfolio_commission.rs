use prime_sdk_rs::{
    client::PrimeClient,
    services::commission::{CommissionService, GetPortfolioCommissionRequest},
};
use std::env;

/// Usage:
///   ./run_example.sh commission/get_portfolio_commission  # Use PORTFOLIO_ID from .env
///   ./run_example.sh commission/get_portfolio_commission -- <product_id>  # Use CLI argument for product_id
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    dotenv::dotenv().ok();

    // Get portfolio ID from environment variable (required)
    let portfolio_id = env::var("PORTFOLIO_ID").map_err(|_| {
        "PORTFOLIO_ID not found in .env file. Please set PORTFOLIO_ID in your .env file."
    })?;

    // Get product ID from command line argument or environment variable (optional)
    let product_id = if let Some(arg) = env::args().nth(1) {
        // Use command line argument if provided
        Some(arg)
    } else {
        // Fall back to environment variable
        env::var("PRODUCT_ID").ok()
    };

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let commission_service = CommissionService::new(prime_client);
    println!("📋 Fetching commission for portfolio: {}", portfolio_id);

    let mut request = GetPortfolioCommissionRequest::new(portfolio_id);
    if let Some(pid) = product_id {
        println!("🔍 Filtering by product ID: {}", pid);
        request = request.with_product_id(pid);
    }

    match commission_service.get_portfolio_commission(request).await {
        Ok(response) => {
            println!("✅ Successfully retrieved portfolio commission");
            println!("\n📊 Commission Details:");
            println!("   Type: {:?}", response.commission_type());
            println!("   Rate: {:?}", response.rate());
            println!("   Trading Volume: {:?}", response.trading_volume());
            println!("\n📋 Full Commission: {:#?}", response.commission());
        }
        Err(e) => {
            eprintln!("❌ Failed to get portfolio commission: {}", e);
        }
    }

    Ok(())
}
