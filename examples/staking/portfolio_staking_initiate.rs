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
// #docs operationId: PrimeRESTAPI_PortfolioStakingInitiate
// #docs operationName: Request to stake currency in a portfolio
use prime_sdk_rs::services::staking::{PortfolioStakingInitiateRequest, StakingService};
use prime_sdk_rs::types::generated::generated::portfolio_staking_initiate_request::PortfolioStakingInitiateRequest as PortfolioStakingInitiateBody;
use prime_sdk_rs::PrimeClient;
use std::env;
use uuid::Uuid;

/**
 * Usage:
 * ./run_example.sh staking/portfolio_staking_initiate
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    let prime_client = PrimeClient::new()?;
    let service = StakingService::new(prime_client);
    let portfolio_id = env::var("PORTFOLIO_ID").expect("PORTFOLIO_ID env var required");

    let body = PortfolioStakingInitiateBody::new(
        Uuid::new_v4().to_string(),
        "ETH".to_string(),
        "1.0".to_string(),
    );
    let request = PortfolioStakingInitiateRequest::new(&portfolio_id, body);

    println!("📋 Submitting portfolio_staking_initiate: {:#?}", request);
    match service.portfolio_staking_initiate(request).await {
        Ok(response) => println!("✅ Response: {:#?}", response),
        Err(e) => eprintln!("❌ Failed: {}", e),
    }

    Ok(())
}
