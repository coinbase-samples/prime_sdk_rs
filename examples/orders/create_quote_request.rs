use prime_rs_sdk::orders::{OrdersService, RfqRequest};
use prime_rs_sdk::services::orders::OrderSide;
use prime_rs_sdk::PrimeClient;
use std::env;
use uuid::Uuid;

/// Usage:
/// ./run_example.sh orders/create_quote_request
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    dotenv::dotenv().ok();

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let orders_service = OrdersService::new(prime_client);

    // Only portfolio_id from env
    let portfolio_id = env::var("PORTFOLIO_ID").expect("PORTFOLIO_ID env var required");

    // Hard-coded request fields
    let base_quantity = "250";
    let limit_price = "0.723";
    let product_id = "ADA-USD";
    let client_quote_id = Uuid::new_v4().to_string();

    let request = RfqRequest::new(
        &portfolio_id,
        product_id,
        OrderSide::Buy,
        &client_quote_id,
        limit_price,
    )
    .with_base_quantity(base_quantity);

    println!("📋 Submitting create_quote_request: {:#?}", request);
    match orders_service.create_quote_request(request).await {
        Ok(response) => {
            println!("✅ Quote request created! Response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to create quote request: {}", e);
        }
    }
    Ok(())
}
