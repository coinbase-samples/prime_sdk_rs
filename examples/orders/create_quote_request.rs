/**
 * Copyright 2025-present Coinbase Global, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *  http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
// #docs operationId: PrimeRESTAPI_CreateQuoteRequest
// #docs operationName: Create Quote Request
use prime_sdk_rs::orders::{OrdersService, RfqRequest};
use prime_sdk_rs::services::orders::OrderSide;
use prime_sdk_rs::PrimeClient;
use std::env;
use uuid::Uuid;

/**
 * Usage:
 * ./run_example.sh orders/create_quote_request
 */
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
