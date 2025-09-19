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
// #docs operationId: PrimeRESTAPI_GetWalletTransactions
// #docs operationName: List Wallet Transactions
use prime_sdk_rs::services::{ListWalletTransactionsRequest, TransactionsService};
use prime_sdk_rs::PrimeClient;
use std::env;

/**
 * Usage:
 * ./run_example.sh examples/transactions/list_wallet_transactions  # Uses PORTFOLIO_ID from .env, wallet_id from CLI
 * ./run_example.sh examples/transactions/list_wallet_transactions -- <wallet_id> [cursor]
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    let mut args = env::args().skip(1);
    let portfolio_id =
        env::var("PORTFOLIO_ID").map_err(|_| "PORTFOLIO_ID required in environment")?;
    let wallet_id = args.next().ok_or("wallet_id required as first CLI arg")?;
    let cursor = args.next();
    println!("📁 Portfolio ID: {}", portfolio_id);
    println!("👛 Wallet ID: {}", wallet_id);

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let service = TransactionsService::new(prime_client);
    println!("📋 Fetching transactions for wallet: {}", wallet_id);

    let mut req = ListWalletTransactionsRequest::new(portfolio_id, wallet_id).with_limit(20);
    if let Some(cursor) = cursor {
        req = req.with_cursor(cursor);
    }

    match service.list_wallet_transactions(req).await {
        Ok(resp) => {
            let txs = resp.transactions;
            println!("📄 Found {} transactions:", txs.len());
            for (i, tx) in txs.iter().enumerate() {
                println!("  {}. Transaction: {:#?},", i + 1, tx);
            }
            if let Some(pagination) = resp.pagination {
                println!("📄 Pagination: {:#?}", pagination);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to list transactions: {}", e);
        }
    }
    println!("✅ Done.");
    Ok(())
}
