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
// #docs operationId: PrimeRESTAPI_GetTFTieredPricingFees
// #docs operationName: Get Trade Finance Tiered Pricing Fees
use prime_sdk_rs::{
    client::PrimeClient,
    services::financing::{FinancingService, ListTFTieredPricingFeesRequest},
};
use std::env;

/**
 * Usage:
 * ./run_example.sh financing/list_tf_tiered_pricing_fees
 *
 * Requires ENTITY_ID. Optionally EFFECTIVE_AT env var.
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let financing_service = FinancingService::new(prime_client);
    let entity_id = env::var("ENTITY_ID").expect("ENTITY_ID env var required");
    let mut request = ListTFTieredPricingFeesRequest::new(&entity_id);
    if let Ok(effective_at) = env::var("EFFECTIVE_AT") {
        request = request.with_effective_at(effective_at);
    }
    println!(
        "📋 Submitting list_tf_tiered_pricing_fees request: {:#?}",
        request
    );
    match financing_service.list_tf_tiered_pricing_fees(request).await {
        Ok(response) => {
            println!("✅ TF tiered pricing fees response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to list TF tiered pricing fees: {}", e);
        }
    }
    Ok(())
}
