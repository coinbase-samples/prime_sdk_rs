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
// #docs operationId: PrimeRESTAPI_CreateWallet
// #docs operationName: Create Wallet
use prime_sdk_rs::services::{CreateWalletRequest, WalletsService};
use prime_sdk_rs::types::generated::generated::wallet_type::WalletType;
use prime_sdk_rs::PrimeClient;
use std::env;
use uuid::Uuid;

/**
 * Usage:
 * ./run_example.sh examples/wallets/create_wallet  # Use PORTFOLIO_ID from .env
 * ./run_example.sh examples/wallets/create_wallet -- ETH  # Use ETH as symbol
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    let portfolio_id = env::var("PORTFOLIO_ID").map_err(|_| {
        "PORTFOLIO_ID not found in .env file. Please set PORTFOLIO_ID in your .env file."
    })?;

    // Accept symbol as a command-line argument, default to ETH
    let symbol = env::args().nth(1).unwrap_or_else(|| "ETH".to_string());
    println!("🪙 Using symbol: {}", symbol);

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let service = WalletsService::new(prime_client);
    println!("📝 Creating a new wallet in portfolio: {}", portfolio_id);

    let request = CreateWalletRequest::new(format!("My {} Wallet", symbol), symbol.clone())
        .with_wallet_type(WalletType::Vault);

    match service.create_wallet(portfolio_id, request).await {
        Ok(response) => {
            println!("✅ Successfully created wallet");
            println!("\n👛 Wallet creation response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to create wallet: {}", e);
        }
    }
    Ok(())
}
