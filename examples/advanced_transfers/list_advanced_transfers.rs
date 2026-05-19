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
// #docs operationId: PrimeRESTAPI_ListAdvancedTransfers
// #docs operationName: List Advanced Transfers
use prime_sdk_rs::services::advanced_transfers::{
    AdvancedTransfersService, ListAdvancedTransfersRequest,
};
use prime_sdk_rs::PrimeClient;
use std::env;

/**
 * Usage:
 * ./run_example.sh advanced_transfers/list_advanced_transfers
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let service = AdvancedTransfersService::new(prime_client);
    let portfolio_id = env::var("PORTFOLIO_ID").expect("PORTFOLIO_ID env var required");

    let request = ListAdvancedTransfersRequest::new(&portfolio_id);

    println!("📋 Submitting list_advanced_transfers: {:#?}", request);
    match service.list_advanced_transfers(request).await {
        Ok(response) => println!("✅ Advanced transfers: {:#?}", response),
        Err(e) => eprintln!("❌ Failed to list advanced transfers: {}", e),
    }

    Ok(())
}
