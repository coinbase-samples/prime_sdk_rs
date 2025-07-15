use prime_sdk_rs::{
    client::PrimeClient,
    services::orders::{GetOrderRequest, OrdersService},
};
use std::env;

/// Usage:
///   ./run_example.sh orders/get_order -- <order_id>
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
        "📋 Fetching order {} for portfolio: {}",
        order_id, portfolio_id
    );

    let request = GetOrderRequest::new(&portfolio_id, &order_id);
    match orders_service.get_order(request).await {
        Ok(response) => {
            println!("✅ Successfully retrieved order:");
            println!("{:#?}", response.order);
        }
        Err(e) => {
            eprintln!("❌ Failed to get order: {}", e);
        }
    }
    Ok(())
}
