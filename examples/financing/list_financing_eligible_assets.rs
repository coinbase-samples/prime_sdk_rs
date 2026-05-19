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
// #docs operationId: PrimeRESTAPI_ListFinancingEligibleAssets
// #docs operationName: List Financing Eligible Assets
use prime_sdk_rs::services::financing::{FinancingService, ListFinancingEligibleAssetsRequest};
use prime_sdk_rs::PrimeClient;

/**
 * Usage:
 * ./run_example.sh financing/list_financing_eligible_assets
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let service = FinancingService::new(prime_client);
    let request = ListFinancingEligibleAssetsRequest::new();

    println!("📋 Submitting list_financing_eligible_assets request");
    match service.list_financing_eligible_assets(request).await {
        Ok(response) => println!("✅ Eligible assets: {:#?}", response),
        Err(e) => eprintln!("❌ Failed to list financing eligible assets: {}", e),
    }

    Ok(())
}
