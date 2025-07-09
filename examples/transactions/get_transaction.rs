use prime_rs_sdk::services::{GetTransactionRequest, TransactionsService};
use prime_rs_sdk::PrimeClient;
use std::env;

/// Usage:
///   ./run_example.sh examples/transactions/get_transaction -- <transaction_id>
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    let mut args = env::args().skip(1);
    let portfolio_id =
        env::var("PORTFOLIO_ID").map_err(|_| "PORTFOLIO_ID required in environment")?;
    let transaction_id = args.next().ok_or("transaction_id required as CLI arg")?;
    println!("📁 Portfolio ID: {}", portfolio_id);
    println!("🔎 Transaction ID: {}", transaction_id);

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let service = TransactionsService::new(prime_client);
    println!("📋 Fetching transaction details...");

    let req = GetTransactionRequest::new(portfolio_id, transaction_id);
    match service.get_transaction(req).await {
        Ok(resp) => {
            println!("🔎 Transaction: {:#?}", resp.transaction);
        }
        Err(e) => {
            eprintln!("❌ Failed to get transaction: {}", e);
        }
    }
    println!("✅ Done.");
    Ok(())
}
