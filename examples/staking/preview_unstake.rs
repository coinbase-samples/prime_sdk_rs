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
// #docs operationId: PrimeRESTAPI_PreviewUnstake
// #docs operationName: Preview Unstake
use prime_sdk_rs::services::staking::{PreviewUnstakeRequest, StakingService};
use prime_sdk_rs::types::generated::generated::preview_unstake_request::PreviewUnstakeRequest as PreviewUnstakeBody;
use prime_sdk_rs::PrimeClient;
use std::env;

/**
 * Usage:
 * ./run_example.sh staking/preview_unstake
 *
 * Optionally pass amount as first argument (default "1.0").
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    let prime_client = PrimeClient::new()?;
    let service = StakingService::new(prime_client);
    let portfolio_id = env::var("PORTFOLIO_ID").expect("PORTFOLIO_ID env var required");
    let wallet_id = env::var("WALLET_ID").expect("WALLET_ID env var required");

    let amount = env::args().nth(1).unwrap_or_else(|| "1.0".to_string());
    let body = PreviewUnstakeBody::new(amount);
    let request = PreviewUnstakeRequest::new(&portfolio_id, &wallet_id, body);

    println!("📋 Submitting preview_unstake: {:#?}", request);
    match service.preview_unstake(request).await {
        Ok(response) => println!("✅ Response: {:#?}", response),
        Err(e) => eprintln!("❌ Failed: {}", e),
    }

    Ok(())
}
