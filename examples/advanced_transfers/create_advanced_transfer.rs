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
// #docs operationId: PrimeRESTAPI_CreateAdvancedTransfer
// #docs operationName: Create Advanced Transfer
use prime_sdk_rs::services::advanced_transfers::{
    AdvancedTransfersService, CreateAdvancedTransferRequest,
};
use prime_sdk_rs::types::generated::generated::{
    advanced_transfer::AdvancedTransfer,
    advanced_transfer_type::AdvancedTransferType,
    create_advanced_transfer_request::CreateAdvancedTransferRequest as CreateAdvancedTransferBody,
};
use prime_sdk_rs::PrimeClient;
use std::env;

/**
 * Usage:
 * ./run_example.sh advanced_transfers/create_advanced_transfer
 *
 * Builds a minimal advanced transfer body; populate fund_movements and metadata
 * for your use case before calling in production.
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

    let mut advanced_transfer = AdvancedTransfer::new();
    advanced_transfer.r#type = Some(AdvancedTransferType::AdvancedTransferTypeBlindMatch);

    let body = CreateAdvancedTransferBody::new(advanced_transfer);
    let request = CreateAdvancedTransferRequest::new(&portfolio_id, body);

    println!("📋 Submitting create_advanced_transfer: {:#?}", request);
    match service.create_advanced_transfer(request).await {
        Ok(response) => println!("✅ Advanced transfer created: {:#?}", response),
        Err(e) => eprintln!("❌ Failed to create advanced transfer: {}", e),
    }

    Ok(())
}
