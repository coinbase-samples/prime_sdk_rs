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
// #docs operationId: PrimeRESTAPI_CreateOnchainTransaction
// #docs operationName: Create Onchain Transaction
use prime_sdk_rs::services::transactions::{
    CreateOnchainTransactionRequest, TransactionsService,
};
use prime_sdk_rs::PrimeClient;
use std::env;

/**
 * Usage:
 * ./run_example.sh transactions/create_onchain_transaction -- <raw_unsigned_txn_hex>
 *
 * Requires PORTFOLIO_ID and WALLET_ID in .env.
 * Pass a real unsigned transaction hex string from your chain tooling.
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
    let wallet_id = env::var("WALLET_ID").expect("WALLET_ID env var required");

    let raw_unsigned_txn = env::args().nth(1).unwrap_or_else(|| {
        "0x".to_string() // replace with a valid unsigned transaction hex
    });

    let request = CreateOnchainTransactionRequest::new(&raw_unsigned_txn);

    println!("📋 Submitting create_onchain_transaction request: {:#?}", request);
    match service
        .create_onchain_transaction(&portfolio_id, &wallet_id, request)
        .await
    {
        Ok(response) => println!("✅ Onchain transaction created: {:#?}", response),
        Err(e) => eprintln!("❌ Failed to create onchain transaction: {}", e),
    }

    Ok(())
}
