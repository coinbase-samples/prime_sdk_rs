use prime_rs_sdk::{
    client::PrimeClient,
    services::orders::{CancelOrderRequest, OrdersService},
};
use std::env;

/// Usage:
///   ./run_example.sh orders/cancel_order -- <order_id>
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let orders_service = OrdersService::new(prime_client);
    let portfolio_id = env::var("PORTFOLIO_ID").map_err(|_| {
        "PORTFOLIO_ID environment variable is required. Please set it in your .env file."
    })?;
    let order_id = env::args().nth(1).expect("order_id argument required");

    println!(
        "📋 Canceling order {} for portfolio: {}",
        order_id, portfolio_id
    );

    let request = CancelOrderRequest::new(&portfolio_id, &order_id);
    match orders_service.cancel_order(request).await {
        Ok(response) => {
            println!("✅ Successfully canceled order. Response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to cancel order: {}", e);
        }
    }
    Ok(())
}
