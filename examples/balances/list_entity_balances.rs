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
// #docs operationId: PrimeRESTAPI_ListEntityBalances
// #docs operationName: List Entity Balances
use prime_sdk_rs::{
    client::PrimeClient,
    services::balances::{BalancesService, ListEntityBalancesRequest},
    utils::PaginatedList,
};
use std::env;

/**
 * Usage:
 * ./run_example.sh balances/list_entity_balances -- <entity_id>  # Use command line argument
 * ./run_example.sh balances/list_entity_balances                  # Use ENTITY_ID from .env
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    dotenv::dotenv().ok();

    // Get entity ID from command line argument or environment variable (required)
    let entity_id = if let Some(arg) = env::args().nth(1) {
        // Use command line argument if provided
        arg
    } else {
        // Fall back to environment variable
        env::var("ENTITY_ID").map_err(|_| {
            "ENTITY_ID not provided as command line argument or found in .env file. \
            Usage: ./run_example.sh balances/list_entity_balances -- <entity_id> \
            or set ENTITY_ID in your .env file."
        })?
    };

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let balances_service = BalancesService::new(prime_client);
    println!("📋 Fetching entity balances for: {}", entity_id);

    let request = ListEntityBalancesRequest::new(entity_id).with_limit(10);

    match balances_service.list_entity_balances(request).await {
        Ok(response) => {
            println!("✅ Successfully retrieved entity balances");
            println!("Total balances: {}", response.count());

            for balance in response.items() {
                println!(
                    "  - Symbol: {}, Available: {}",
                    balance.symbol.as_deref().unwrap_or("N/A"),
                    balance.available_balance.as_deref().unwrap_or("N/A")
                );
            }

            // Print pagination info using PaginatedList trait
            if response.has_more() {
                if let Some(next_cursor) = response.next_cursor() {
                    println!("Next cursor: {}", next_cursor);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to get entity balances: {}", e);
        }
    }

    Ok(())
}
