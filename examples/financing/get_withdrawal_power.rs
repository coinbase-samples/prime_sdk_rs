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
// #docs operationId: PrimeRESTAPI_GetWithdrawalPower
// #docs operationName: Get Portfolio Withdrawal Power
use prime_sdk_rs::{
    client::PrimeClient,
    services::financing::{FinancingService, GetWithdrawalPowerRequest},
};
use std::env;
use std::process;

/**
 * Usage:
 * ./run_example.sh financing/get_withdrawal_power [SYMBOL]
 *
 * Requires PORTFOLIO_ID env var. Optionally takes symbol as CLI arg, defaults to ETH.
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    let args: Vec<String> = env::args().collect();
    let portfolio_id = env::var("PORTFOLIO_ID").unwrap_or_else(|_| {
        eprintln!("PORTFOLIO_ID env var required");
        process::exit(1);
    });
    let symbol = if args.len() > 1 {
        args[1].clone()
    } else {
        "ETH".to_string()
    };

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let financing_service = FinancingService::new(prime_client);
    let request = GetWithdrawalPowerRequest::new(&portfolio_id, &symbol);
    println!("📋 Submitting get_withdrawal_power request: {:#?}", request);
    match financing_service.get_withdrawal_power(request).await {
        Ok(response) => {
            println!("✅ Withdrawal power response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to get withdrawal power: {}", e);
        }
    }
    Ok(())
}
