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
// #docs operationId: PrimeRESTAPI_EditOrder
// #docs operationName: Edit Order (Beta)
use prime_sdk_rs::services::orders::{EditOrderRequest, OrdersService};
use prime_sdk_rs::types::generated::generated::edit_order_request::EditOrderRequest as EditOrderBody;
use prime_sdk_rs::PrimeClient;
use std::env;
use uuid::Uuid;

/**
 * Usage:
 * ./run_example.sh orders/edit_order -- <order_id>
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

    let orig_client_order_id = "original-client-order-id".to_string();
    let client_order_id = Uuid::new_v4().to_string();
    let limit_price = "0.32";
    let base_quantity = "5";

    let mut body = EditOrderBody::new(orig_client_order_id, client_order_id);
    body.limit_price = Some(limit_price.to_string());
    body.base_quantity = Some(base_quantity.to_string());

    let request = EditOrderRequest::new(&portfolio_id, &order_id, body);

    println!("📋 Submitting edit_order request: {:#?}", request);
    match orders_service.edit_order(request).await {
        Ok(response) => println!("✅ Order edited! Response: {:#?}", response),
        Err(e) => eprintln!("❌ Failed to edit order: {}", e),
    }

    Ok(())
}
