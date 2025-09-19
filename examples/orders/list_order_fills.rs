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
// #docs operationId: PrimeRESTAPI_GetOrderFills
// #docs operationName: List Order Fills
use prime_sdk_rs::{
    client::PrimeClient,
    services::orders::{ListOrderFillsRequest, OrdersService},
    utils::PaginatedList,
    SortDirection,
};
use std::env;

/**
 * Usage:
 * ./run_example.sh orders/list_order_fills -- <order_id> [cursor]
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
    let mut args = env::args().skip(1);
    let order_id = args.next().expect("order_id argument required");
    let cursor_arg = args.next();

    println!(
        "📋 Fetching fills for order {} in portfolio: {}",
        order_id, portfolio_id
    );

    let mut request = ListOrderFillsRequest::new(&portfolio_id, &order_id)
        .with_limit(100)
        .with_sort_direction(SortDirection::Desc);
    if let Some(cursor) = cursor_arg {
        request = request.with_cursor(&cursor);
    }

    match orders_service.list_order_fills(request).await {
        Ok(response) => {
            println!("✅ Successfully retrieved {} fills", response.count());
            println!("📄 Has more pages: {}", response.has_more());
            if let Some(cursor) = response.next_cursor() {
                println!("📄 Next cursor: {}", cursor);
            }
            println!("📄 Total fills in this response: {}", response.count());
            for (i, fill) in response.items().iter().take(3).enumerate() {
                println!("\n📋 Fill {}: {:#?}", i + 1, fill);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to list order fills: {}", e);
        }
    }
    Ok(())
}
