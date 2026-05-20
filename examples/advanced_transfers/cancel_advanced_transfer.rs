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
// #docs operationId: PrimeRESTAPI_CancelAdvancedTransfer
// #docs operationName: Cancel Advanced Transfer
use prime_sdk_rs::services::advanced_transfers::{
    AdvancedTransfersService, CancelAdvancedTransferRequest,
};
use prime_sdk_rs::PrimeClient;
use std::env;

/**
 * Usage:
 * ./run_example.sh advanced_transfers/cancel_advanced_transfer -- <advanced_transfer_id>
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
    let advanced_transfer_id = env::args()
        .nth(1)
        .expect("advanced_transfer_id argument required");

    let request = CancelAdvancedTransferRequest::new(&portfolio_id, &advanced_transfer_id);

    println!("📋 Submitting cancel_advanced_transfer: {:#?}", request);
    match service.cancel_advanced_transfer(request).await {
        Ok(response) => println!("✅ Advanced transfer canceled: {:#?}", response),
        Err(e) => eprintln!("❌ Failed to cancel advanced transfer: {}", e),
    }

    Ok(())
}
