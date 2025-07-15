use prime_sdk_rs::{
    client::PrimeClient,
    services::financing::{FinancingService, GetPostTradeCreditRequest},
};
use std::env;
use std::process;

/// Usage:
///   ./run_example.sh financing/get_post_trade_credit
///
/// Requires PORTFOLIO_ID env var.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    let portfolio_id = env::var("PORTFOLIO_ID").unwrap_or_else(|_| {
        eprintln!("PORTFOLIO_ID env var required");
        process::exit(1);
    });

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let financing_service = FinancingService::new(prime_client);
    let request = GetPostTradeCreditRequest::new(&portfolio_id);
    println!(
        "📋 Submitting get_post_trade_credit request: {:#?}",
        request
    );
    match financing_service.get_post_trade_credit(request).await {
        Ok(response) => {
            println!("✅ Post-trade credit response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to get post-trade credit: {}", e);
        }
    }
    Ok(())
}
