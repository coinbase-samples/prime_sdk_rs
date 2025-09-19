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
// #docs operationId: PrimeRESTAPI_GetPortfolio
// #docs operationName: Get Portfolio
use prime_sdk_rs::{
    client::PrimeClient,
    services::portfolios::{GetPortfolioRequest, PortfoliosService},
};
use std::env;

/**
 * Usage:
 * ./run_example.sh portfolios/get_portfolio -- <portfolio_id>  # Use command line argument
 * ./run_example.sh portfolios/get_portfolio                    # Use PORTFOLIO_ID from .env
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    dotenv::dotenv().ok();

    // Get portfolio ID from CLI or env
    let portfolio_id = if let Some(arg) = env::args().nth(1) {
        arg
    } else {
        env::var("PORTFOLIO_ID").map_err(|_| {
            "PORTFOLIO_ID not provided as command line argument or found in .env file. \
            Usage: ./run_example.sh portfolios/get_portfolio -- <portfolio_id> \
            or set PORTFOLIO_ID in your .env file."
        })?
    };

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let portfolios_service = PortfoliosService::new(prime_client);
    println!("📋 Fetching portfolio: {}", portfolio_id);

    // Create request object
    let request = GetPortfolioRequest::new(portfolio_id);

    match portfolios_service.get_portfolio(request).await {
        Ok(response) => {
            println!("✅ Successfully retrieved portfolio");
            println!("\n📋 Portfolio: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to get portfolio: {}", e);
        }
    }

    Ok(())
}
