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
// #docs operationId: PrimeRESTAPI_GetEntityAssets
// #docs operationName: List Entity Assets
use prime_sdk_rs::{
    client::PrimeClient,
    services::assets::{AssetsService, ListEntityAssetsRequest},
    utils::PaginatedList,
};
use std::env;

/**
 * Usage:
 * ./run_example.sh assets/list_entity_assets
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    println!("🚀 Creating PrimeClient...");

    // Create a new PrimeClient with default configuration
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    // Create assets service using the PrimeClient
    let assets_service = AssetsService::new(prime_client);

    // Get entity ID from environment variable (required)
    let entity_id = env::var("ENTITY_ID").map_err(|_| {
        "ENTITY_ID environment variable is required. Please set it in your .env file."
    })?;

    println!("📋 Fetching assets for entity: {}", entity_id);

    // Example: List entity assets
    let request = ListEntityAssetsRequest::new(&entity_id);

    match assets_service.list_entity_assets(request).await {
        Ok(response) => {
            println!("✅ Successfully retrieved {} assets", response.count());

            // Display first few assets using PaginatedList trait
            for (i, asset) in response.items().iter().enumerate() {
                println!("\n📋 Asset {}: {:#?}", i + 1, asset);
            }

            // Demonstrate utility methods
            let trading_assets = response.trading_assets();
            let non_trading_assets = response.non_trading_assets();

            println!("\n📊 Summary:");
            println!("   Total assets: {}", response.count());
            println!("   Trading assets: {}", trading_assets.len());
            println!("   Non-trading assets: {}", non_trading_assets.len());

            // Example: Find a specific asset by symbol
            if let Some(btc_asset) = response.get_by_symbol("BTC") {
                println!("\n🔍 Found BTC asset:");
                println!("   Name: {:?}", btc_asset.name);
                println!("   Trading Supported: {:?}", btc_asset.trading_supported);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to list entity assets: {}", e);
        }
    }

    Ok(())
}
