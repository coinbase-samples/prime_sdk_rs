use prime_rs_sdk::orders::{AcceptQuoteRequest, OrdersService};
use prime_rs_sdk::services::orders::OrderSide;
use prime_rs_sdk::PrimeClient;
use std::env;
use uuid::Uuid;

/// Usage:
/// ./run_example.sh orders/accept_quote -- <quote_id>
///
/// You must provide a valid quote_id from a previous create_quote_request response.
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
    let quote_id = env::args().nth(1).expect("quote_id argument required");

    // Hard-coded request fields
    let product_id = "ADA-USD";
    let client_order_id = Uuid::new_v4().to_string();

    let request = AcceptQuoteRequest::new(
        &portfolio_id,
        product_id,
        OrderSide::Buy,
        &client_order_id,
        &quote_id,
    );

    println!("📋 Submitting accept_quote request: {:#?}", request);
    match orders_service.accept_quote(request).await {
        Ok(response) => {
            println!("✅ Quote accepted! Response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to accept quote: {}", e);
        }
    }
    Ok(())
}
