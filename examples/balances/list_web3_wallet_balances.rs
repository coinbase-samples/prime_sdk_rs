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
// #docs operationId: PrimeRESTAPI_ListWeb3WalletBalances
// #docs operationName: List Web3 Wallet Balances
use prime_sdk_rs::{
    client::PrimeClient,
    services::balances::{BalancesService, ListWeb3WalletBalancesRequest},
};
use std::env;

/**
 * Usage:
 * ./run_example.sh balances/list_web3_wallet_balances -- <wallet_id>  # Use command line argument for wallet_id
 * ./run_example.sh balances/list_web3_wallet_balances                  # Use WEB3_WALLET_ID from .env
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    dotenv::dotenv().ok();

    // Get portfolio ID from environment variable (required)
    let portfolio_id = env::var("PORTFOLIO_ID").map_err(|_| {
        "PORTFOLIO_ID not found in .env file. Please set PORTFOLIO_ID in your .env file."
    })?;

    // Get wallet ID from command line argument or environment variable (required)
    let wallet_id = if let Some(arg) = env::args().nth(1) {
        // Use command line argument if provided
        arg
    } else {
        // Fall back to environment variable
        env::var("WEB3_WALLET_ID").map_err(|_| {
            "WEB3_WALLET_ID not provided as command line argument or found in .env file. \
            Usage: ./run_example.sh balances/list_web3_wallet_balances -- <wallet_id> \
            or set WEB3_WALLET_ID in your .env file."
        })?
    };

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let balances_service = BalancesService::new(prime_client);
    println!(
        "📋 Fetching web3 wallet balances for portfolio: {} wallet: {}",
        portfolio_id, wallet_id
    );

    let request = ListWeb3WalletBalancesRequest::new(portfolio_id, wallet_id).with_limit(10);

    match balances_service.list_web3_wallet_balances(request).await {
        Ok(response) => {
            println!("✅ Successfully retrieved web3 wallet balances");
            println!("Total web3 balances: {}", response.count());

            for balance in response.items() {
                println!("Balance: {:#?}", balance);
            }

            println!("Total DeFi balances: {}", response.defi_balances().len());
            for defi_balance in response.defi_balances() {
                println!("DeFi Balance: {:#?}", defi_balance);
            }

            // Print pagination info using PaginatedList trait
            if response.has_more() {
                if let Some(next_cursor) = response.next_cursor() {
                    println!("Next cursor: {}", next_cursor);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to get web3 wallet balances: {}", e);
        }
    }

    Ok(())
}
