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
// #docs operationId: PrimeRESTAPI_ListOnchainAddressGroups
// #docs operationName: List Onchain Address Groups
use prime_sdk_rs::client::PrimeClient;
use prime_sdk_rs::services::onchain_address_groups::{
    types::ListOnchainAddressGroupsRequest, OnchainAddressGroupsService,
};
use std::env;

/**
 * Usage:
 * ./run_example.sh examples/onchain_address_book/list_onchain_address_groups
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Get portfolio ID from environment variable (required)
    let portfolio_id = env::var("PORTFOLIO_ID").map_err(|_| {
        "PORTFOLIO_ID environment variable is required. Please set it in your .env file."
    })?;

    println!("🚀 Creating PrimeClient...");
    let client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let service = OnchainAddressGroupsService::new(client);
    let request = ListOnchainAddressGroupsRequest { portfolio_id };

    println!("🔎 Listing Onchain Address Groups...");
    let response = service.list_onchain_address_groups(request).await?;

    println!("\n📦 Address Groups:");
    for (i, group) in response.address_groups.iter().enumerate() {
        println!("  {}. {:#?}", i + 1, group);
    }

    let pagination = response.pagination();
    println!("\n📄 Pagination Info:");
    println!("  has_more: {}", pagination.has_next);
    println!("  next_cursor: {}", pagination.next_cursor);
    println!("  sort_direction: {:?}", pagination.sort_direction);
    println!("  count: {}", response.address_groups.len());

    Ok(())
}
