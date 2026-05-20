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
// #docs operationId: PrimeRESTAPI_GetPortfolioCounterpartyID
// #docs operationName: Get Portfolio Counterparty ID
use prime_sdk_rs::services::advanced_transfers::{
    AdvancedTransfersService, GetPortfolioCounterpartyIdRequest,
};
use prime_sdk_rs::PrimeClient;
use std::env;

/**
 * Usage:
 * ./run_example.sh advanced_transfers/get_portfolio_counterparty_id
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let service = AdvancedTransfersService::new(prime_client);
    let portfolio_id = env::var("PORTFOLIO_ID").expect("PORTFOLIO_ID env var required");

    let request = GetPortfolioCounterpartyIdRequest::new(&portfolio_id);

    println!("📋 Submitting get_portfolio_counterparty_id: {:#?}", request);
    match service.get_portfolio_counterparty_id(request).await {
        Ok(response) => println!("✅ Counterparty: {:#?}", response),
        Err(e) => eprintln!("❌ Failed to get portfolio counterparty ID: {}", e),
    }

    Ok(())
}
