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
// #docs operationId: PrimeRESTAPI_GetAllocation
// #docs operationName: Get Allocation
use prime_sdk_rs::{
    client::PrimeClient,
    services::allocations::{AllocationService, GetAllocationRequest},
};
use std::env;

/**
 * Usage:
 * ./run_example.sh allocations/get_allocation -- <allocation_id>  # Use CLI argument for allocation_id
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

    // Get allocation ID from command line argument (required)
    let allocation_id = env::args().nth(1).ok_or(
        "ALLOCATION_ID not provided. Usage: ./run_example.sh allocations/get_allocation -- <allocation_id>"
    )?;

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let allocation_service = AllocationService::new(prime_client);
    println!(
        "📋 Fetching allocation: {} for portfolio: {}",
        allocation_id, portfolio_id
    );

    let request = GetAllocationRequest::new(portfolio_id, allocation_id);

    match allocation_service.get_allocation(request).await {
        Ok(response) => {
            if response.is_some() {
                println!("✅ Successfully retrieved allocation");
                println!("📊 Response: {:#?}", response);
            } else {
                println!("📭 No allocation found");
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to get allocation: {}", e);
        }
    }

    Ok(())
}
