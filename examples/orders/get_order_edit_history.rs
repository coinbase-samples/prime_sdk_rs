/**
 * Copyright 2026-present Coinbase Global, Inc.
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
// #docs operationId: PrimeRESTAPI_GetOrderEditHistory
// #docs operationName: List Order Edit History
use prime_sdk_rs::services::orders::{GetOrderEditHistoryRequest, OrdersService};
use prime_sdk_rs::PrimeClient;
use std::env;

/**
 * Usage:
 * ./run_example.sh orders/get_order_edit_history -- <order_id>
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let orders_service = OrdersService::new(prime_client);
    let portfolio_id = env::var("PORTFOLIO_ID").expect("PORTFOLIO_ID env var required");
    let order_id = env::args().nth(1).expect("order_id argument required");

    let request = GetOrderEditHistoryRequest::new(&portfolio_id, &order_id);

    println!("📋 Submitting get_order_edit_history request: {:#?}", request);
    match orders_service.get_order_edit_history(request).await {
        Ok(response) => println!("✅ Edit history: {:#?}", response),
        Err(e) => eprintln!("❌ Failed to get order edit history: {}", e),
    }

    Ok(())
}
