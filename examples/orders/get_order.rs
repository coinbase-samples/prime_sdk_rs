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
// #docs operationId: PrimeRESTAPI_GetOrder
// #docs operationName: Get Order
use prime_sdk_rs::{
    client::PrimeClient,
    services::orders::{GetOrderRequest, OrdersService},
};
use std::env;

/**
 * Usage:
 * ./run_example.sh orders/get_order -- <order_id>
 */
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
