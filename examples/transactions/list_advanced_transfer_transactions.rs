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
// #docs operationId: PrimeRESTAPI_ListAdvancedTransferTransactions
// #docs operationName: List transactions associated with an Advanced Transfer
use prime_sdk_rs::services::transactions::{
    ListAdvancedTransferTransactionsRequest, TransactionsService,
};
use prime_sdk_rs::PrimeClient;
use std::env;

/**
 * Usage:
 * ./run_example.sh transactions/list_advanced_transfer_transactions -- <advanced_transfer_id>
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let service = TransactionsService::new(prime_client);
    let portfolio_id = env::var("PORTFOLIO_ID").expect("PORTFOLIO_ID env var required");
    let advanced_transfer_id = env::args()
        .nth(1)
        .expect("advanced_transfer_id argument required");

    let request =
        ListAdvancedTransferTransactionsRequest::new(&portfolio_id, &advanced_transfer_id);

    println!("📋 Submitting list_advanced_transfer_transactions request: {:#?}", request);
    match service.list_advanced_transfer_transactions(request).await {
        Ok(response) => println!("✅ Advanced transfer transactions: {:#?}", response),
        Err(e) => eprintln!("❌ Failed to list advanced transfer transactions: {}", e),
    }

    Ok(())
}
