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
// #docs operationId: PrimeRESTAPI_GetPortfolioAllocations
// #docs operationName: Get Portfolio Allocations
use prime_sdk_rs::{
    client::PrimeClient,
    services::allocations::{AllocationService, ListPortfolioAllocationsRequest},
};
use std::env;

/**
 * Usage:
 * ./run_example.sh allocations/list_portfolio_allocations  # Use PORTFOLIO_ID from .env
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    dotenv::dotenv().ok();

    // Get portfolio ID from environment variable (required)
    let portfolio_id = env::var("PORTFOLIO_ID").map_err(|_| {
        "PORTFOLIO_ID not found in .env file. Please set PORTFOLIO_ID in your .env file."
    })?;

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let allocation_service = AllocationService::new(prime_client);
    println!("📋 Fetching allocations for portfolio: {}", portfolio_id);

    // Use a default start date
    let start_date = "2025-01-01T00:00:00Z";
    let request = ListPortfolioAllocationsRequest::new(portfolio_id, start_date);

    match allocation_service.list_portfolio_allocations(request).await {
        Ok(response) => {
            println!("✅ Successfully retrieved {} allocations", response.count());
            println!("📊 Response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to get allocations: {}", e);
        }
    }

    Ok(())
}
