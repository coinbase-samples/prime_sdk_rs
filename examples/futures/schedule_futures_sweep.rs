use prime_sdk_rs::{
    client::PrimeClient,
    services::futures::{FuturesService, ScheduleFuturesSweepRequest},
};
use std::env;
use std::process;

/// Usage:
///   ./run_example.sh futures/schedule_futures_sweep -- <CURRENCY> [AMOUNT]
///
/// Requires ENTITY_ID environment variable.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: {} -- <CURRENCY> [AMOUNT]", args[0]);
        process::exit(1);
    }
    let currency = &args[1];
    let amount = if args.len() == 3 {
        Some(args[2].clone())
    } else {
        None
    };

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let futures_service = FuturesService::new(prime_client);
    let entity_id = env::var("ENTITY_ID").expect("ENTITY_ID env var required");

    let mut request = ScheduleFuturesSweepRequest::new(&entity_id, currency);
    if let Some(amount) = amount {
        request = request.with_amount(amount);
    }
    println!(
        "📋 Submitting schedule_futures_sweep request: {:#?}",
        request
    );
    match futures_service.schedule_futures_sweep(request).await {
        Ok(response) => {
            println!("✅ Schedule futures sweep response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to schedule futures sweep: {}", e);
        }
    }
    Ok(())
}
