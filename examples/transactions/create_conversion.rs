// Example: Create a conversion transaction
/// Usage:
///   ./run_example.sh examples/transactions/create_conversion -- <destination_wallet_uuid> <amount> <source_symbol> <destination_symbol>
///
/// Required env vars: PORTFOLIO_ID, WALLET_ID
use dotenv;
use env_logger;
use prime_rs_sdk::client::PrimeClient;
use prime_rs_sdk::services::transactions::{CreateConversionRequest, TransactionsService};
use std::env;
use tokio;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    let mut args = env::args().skip(1);
    let destination_wallet = args
        .next()
        .ok_or("destination_wallet_uuid required as first CLI arg")?;
    let amount = args.next().ok_or("amount required as second CLI arg")?;
    let source_symbol = args
        .next()
        .ok_or("source_symbol required as third CLI arg")?;
    let destination_symbol = args
        .next()
        .ok_or("destination_symbol required as fourth CLI arg")?;
    let portfolio_id =
        env::var("PORTFOLIO_ID").map_err(|_| "PORTFOLIO_ID required in environment")?;
    let wallet_id = env::var("WALLET_ID").map_err(|_| "WALLET_ID required in environment")?;
    let idempotency_key = Uuid::new_v4().to_string();

    println!("📁 Portfolio ID: {}", portfolio_id);
    println!("👛 Source Wallet ID: {}", wallet_id);
    println!("➡️  Destination Wallet ID: {}", destination_wallet);
    println!("💸 Amount: {}", amount);
    println!("🔄 Source Symbol: {}", source_symbol);
    println!("🔄 Destination Symbol: {}", destination_symbol);
    println!("🔑 Idempotency Key (generated): {}", idempotency_key);

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let service = TransactionsService::new(prime_client);
    println!("📋 Creating conversion request...");

    let request = CreateConversionRequest::new(
        amount,
        destination_wallet,
        idempotency_key,
        source_symbol,
        destination_symbol,
    );
    println!("📝 Request: {:#?}", request);

    match service
        .create_conversion(&portfolio_id, &wallet_id, request)
        .await
    {
        Ok(response) => {
            println!("🎉 [SUCCESS] Conversion response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ [ERROR] Failed to create conversion: {}", e);
        }
    }
    println!("✅ Done.");
    Ok(())
}
