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
// #docs operationId: PrimeRESTAPI_GetStakingStatus
// #docs operationName: Get Staking Status
use prime_sdk_rs::services::staking::{GetStakingStatusRequest, StakingService};
use prime_sdk_rs::PrimeClient;
use std::env;

/**
 * Usage:
 * ./run_example.sh staking/get_staking_status
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    let prime_client = PrimeClient::new()?;
    let service = StakingService::new(prime_client);
    let portfolio_id = env::var("PORTFOLIO_ID").expect("PORTFOLIO_ID env var required");
    let wallet_id = env::var("WALLET_ID").expect("WALLET_ID env var required");

    let request = GetStakingStatusRequest::new(&portfolio_id, &wallet_id);

    println!("📋 Submitting get_staking_status: {:#?}", request);
    match service.get_staking_status(request).await {
        Ok(response) => println!("✅ Response: {:#?}", response),
        Err(e) => eprintln!("❌ Failed: {}", e),
    }

    Ok(())
}
