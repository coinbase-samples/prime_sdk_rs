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
// #docs operationId: PrimeRESTAPI_CreateAllocation
// #docs operationName: Create Portfolio Allocations
use prime_sdk_rs::services::allocations::{AllocationService, CreateAllocationRequest};
use prime_sdk_rs::types::generated::generated::{
    allocation_leg::AllocationLeg, allocation_size_type::AllocationSizeType,
};
use prime_sdk_rs::PrimeClient;
use std::env;
use uuid::Uuid;

/**
 * Usage:
 * ./run_example.sh allocations/create_allocation
 *
 * Requires PORTFOLIO_ID in .env (source portfolio).
 * Optionally set DESTINATION_PORTFOLIO_ID for allocation leg destinations.
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let service = AllocationService::new(prime_client);

    let source_portfolio_id = env::var("PORTFOLIO_ID").expect("PORTFOLIO_ID env var required");
    let destination_portfolio_id = env::var("DESTINATION_PORTFOLIO_ID")
        .unwrap_or_else(|_| source_portfolio_id.clone());

    // Hard-coded request fields (replace order_ids with real order IDs from your portfolio)
    let product_id = "BTC-USD";
    let order_ids = vec!["00000000-0000-0000-0000-000000000001".to_string()];
    let allocation_id = Uuid::new_v4().to_string();
    let allocation_legs = vec![AllocationLeg::new(
        Uuid::new_v4().to_string(),
        destination_portfolio_id.clone(),
        "50".to_string(),
    )];

    let mut request = CreateAllocationRequest::new();
    request.allocation_id = Some(allocation_id);
    request.source_portfolio_id = Some(source_portfolio_id);
    request.product_id = Some(product_id.to_string());
    request.order_ids = Some(order_ids);
    request.allocation_legs = Some(allocation_legs);
    request.size_type = Some(AllocationSizeType::Percent);
    request.remainder_destination_portfolio = Some(destination_portfolio_id);

    println!("📋 Submitting create_allocation request: {:#?}", request);
    match service.create_allocation(request).await {
        Ok(response) => println!("✅ Allocation created! Response: {:#?}", response),
        Err(e) => eprintln!("❌ Failed to create allocation: {}", e),
    }

    Ok(())
}
