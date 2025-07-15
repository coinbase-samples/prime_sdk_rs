use prime_sdk_rs::{
    client::PrimeClient,
    services::financing::{FinancingService, GetWithdrawalPowerRequest},
};
use std::env;
use std::process;

/// Usage:
///   ./run_example.sh financing/get_withdrawal_power [SYMBOL]
///
/// Requires PORTFOLIO_ID env var. Optionally takes symbol as CLI arg, defaults to ETH.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    let args: Vec<String> = env::args().collect();
    let portfolio_id = env::var("PORTFOLIO_ID").unwrap_or_else(|_| {
        eprintln!("PORTFOLIO_ID env var required");
        process::exit(1);
    });
    let symbol = if args.len() > 1 {
        args[1].clone()
    } else {
        "ETH".to_string()
    };

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let financing_service = FinancingService::new(prime_client);
    let request = GetWithdrawalPowerRequest::new(&portfolio_id, &symbol);
    println!("📋 Submitting get_withdrawal_power request: {:#?}", request);
    match financing_service.get_withdrawal_power(request).await {
        Ok(response) => {
            println!("✅ Withdrawal power response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to get withdrawal power: {}", e);
        }
    }
    Ok(())
}
