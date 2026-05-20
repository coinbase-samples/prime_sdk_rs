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
// #docs operationId: PrimeRESTAPI_StakingInitiate
// #docs operationName: Request to stake or delegate a wallet
use prime_sdk_rs::services::staking::{StakingService, WalletStakingInitiateRequest};
use prime_sdk_rs::types::generated::generated::staking_initiate_request::StakingInitiateRequest as StakingInitiateBody;
use prime_sdk_rs::PrimeClient;
use std::env;
use uuid::Uuid;

/**
 * Usage:
 * ./run_example.sh staking/wallet_staking_initiate
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    let prime_client = PrimeClient::new()?;
    let service = StakingService::new(prime_client);
    let portfolio_id = env::var("PORTFOLIO_ID").expect("PORTFOLIO_ID env var required");
    let wallet_id = env::var("WALLET_ID").expect("WALLET_ID env var required");

    let body = StakingInitiateBody::new(Uuid::new_v4().to_string());
    let request = WalletStakingInitiateRequest::new(&portfolio_id, &wallet_id, body);

    println!("📋 Submitting wallet_staking_initiate: {:#?}", request);
    match service.wallet_staking_initiate(request).await {
        Ok(response) => println!("✅ Response: {:#?}", response),
        Err(e) => eprintln!("❌ Failed: {}", e),
    }

    Ok(())
}
