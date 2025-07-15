use dotenv;
use env_logger;
use prime_sdk_rs::client::PrimeClient;
use prime_sdk_rs::services::transactions::{CreateWalletTransferRequest, TransactionsService};
use std::env;
use tokio;
use uuid::Uuid;

// Usage:
//   ./run_example.sh examples/transactions/create_wallet_transfer -- <destination_wallet_uuid> <amount> <symbol>

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    let mut args = env::args().skip(1);
    let destination = args
        .next()
        .ok_or("destination_wallet_uuid required as first CLI arg")?;
    let amount = args.next().ok_or("amount required as second CLI arg")?;
    let symbol = args.next().ok_or("symbol required as third CLI arg")?;
    let portfolio_id =
        env::var("PORTFOLIO_ID").map_err(|_| "PORTFOLIO_ID required in environment")?;
    let wallet_id = env::var("WALLET_ID").map_err(|_| "WALLET_ID required in environment")?;
    let idempotency_key = Uuid::new_v4().to_string();

    println!("📁 Portfolio ID: {}", portfolio_id);
    println!("👛 Source Wallet ID: {}", wallet_id);
    println!("➡️  Destination Wallet ID: {}", destination);
    println!("💸 Amount: {} {}", amount, symbol);
    println!("🔑 Idempotency Key: {}", idempotency_key);

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let service = TransactionsService::new(prime_client);
    println!("📋 Creating wallet transfer request...");

    let request = CreateWalletTransferRequest::new(amount, destination, idempotency_key, symbol);
    println!("📝 Request: {:#?}", request);

    match service
        .create_wallet_transfer(&portfolio_id, &wallet_id, request)
        .await
    {
        Ok(response) => {
            println!("🎉 [SUCCESS] Wallet transfer response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ [ERROR] Failed to create wallet transfer: {}", e);
        }
    }
    println!("✅ Done.");
    Ok(())
}
