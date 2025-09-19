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
// #docs operationId: PrimeRESTAPI_CreateWalletWithdrawal
// #docs operationName: Create Wallet Withdrawal
use dotenv;
use env_logger;
use prime_sdk_rs::client::PrimeClient;
use prime_sdk_rs::services::transactions::{CreateWalletWithdrawalRequest, TransactionsService};
use prime_sdk_rs::types::generated::generated::{
    blockchain_address::BlockchainAddress, destination_type::DestinationType,
};
use std::env;
use tokio;
use uuid::Uuid;

/**
 * Usage:
 * ./run_example.sh examples/transactions/create_wallet_withdrawal.rs -- <amount> <symbol> <blockchain_address>
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    let mut args = env::args().skip(1);
    let amount = args.next().ok_or("amount required as first CLI arg")?;
    let symbol = args.next().ok_or("symbol required as second CLI arg")?;
    let address = args
        .next()
        .ok_or("blockchain_address required as third CLI arg")?;
    let portfolio_id =
        env::var("PORTFOLIO_ID").map_err(|_| "PORTFOLIO_ID required in environment")?;
    let wallet_id = env::var("WALLET_ID").map_err(|_| "WALLET_ID required in environment")?;
    let idempotency_key = Uuid::new_v4().to_string();

    println!("📁 Portfolio ID: {}", portfolio_id);
    println!("👛 Source Wallet ID: {}", wallet_id);
    println!("🌐 Blockchain Address: {}", address);
    println!("💸 Amount: {} {}", amount, symbol);
    println!("🔑 Idempotency Key (generated): {}", idempotency_key);

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let service = TransactionsService::new(prime_client);
    println!("📋 Creating wallet withdrawal request...");

    let blockchain_address = BlockchainAddress {
        address: Some(address),
        account_identifier: None,
        network: None,
    };

    let request = CreateWalletWithdrawalRequest::new(
        amount,
        DestinationType::DestinationBlockchain,
        idempotency_key,
        symbol,
    )
    .with_blockchain_address(blockchain_address);
    println!("📝 Request: {:#?}", request);

    match service
        .create_wallet_withdrawal(&portfolio_id, &wallet_id, request)
        .await
    {
        Ok(response) => {
            println!("🎉 [SUCCESS] Wallet withdrawal response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ [ERROR] Failed to create wallet withdrawal: {}", e);
        }
    }
    println!("✅ Done.");
    Ok(())
}
