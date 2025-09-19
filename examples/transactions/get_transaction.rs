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
// #docs operationId: PrimeRESTAPI_GetTransaction
// #docs operationName: Get Transaction
use prime_sdk_rs::services::{GetTransactionRequest, TransactionsService};
use prime_sdk_rs::PrimeClient;
use std::env;

/**
 * Usage:
 * ./run_example.sh examples/transactions/get_transaction -- <transaction_id>
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    let mut args = env::args().skip(1);
    let portfolio_id =
        env::var("PORTFOLIO_ID").map_err(|_| "PORTFOLIO_ID required in environment")?;
    let transaction_id = args.next().ok_or("transaction_id required as CLI arg")?;
    println!("📁 Portfolio ID: {}", portfolio_id);
    println!("🔎 Transaction ID: {}", transaction_id);

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let service = TransactionsService::new(prime_client);
    println!("📋 Fetching transaction details...");

    let req = GetTransactionRequest::new(portfolio_id, transaction_id);
    match service.get_transaction(req).await {
        Ok(resp) => {
            println!("🔎 Transaction: {:#?}", resp.transaction);
        }
        Err(e) => {
            eprintln!("❌ Failed to get transaction: {}", e);
        }
    }
    println!("✅ Done.");
    Ok(())
}
