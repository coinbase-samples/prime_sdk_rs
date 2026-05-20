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
// #docs operationId: PrimeRESTAPI_GetCandles
// #docs operationName: Get Public Product Candles (Beta)
use prime_sdk_rs::services::products::{GetCandlesRequest, ProductsService};
use prime_sdk_rs::types::generated::generated::candles_granularity::CandlesGranularity;
use prime_sdk_rs::PrimeClient;
use std::env;

/**
 * Usage:
 * ./run_example.sh products/get_candles -- BTC-USD
 * ./run_example.sh products/get_candles -- BTC-USD 2026-01-01T00:00:00Z 2026-01-02T00:00:00Z
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let products_service = ProductsService::new(prime_client);
    let portfolio_id = env::var("PORTFOLIO_ID").expect("PORTFOLIO_ID env var required");
    let product_id = env::args().nth(1).expect("product_id argument required (e.g. BTC-USD)");
    let start_time = env::args()
        .nth(2)
        .unwrap_or_else(|| "2026-01-01T00:00:00Z".to_string());
    let end_time = env::args()
        .nth(3)
        .unwrap_or_else(|| "2026-01-02T00:00:00Z".to_string());
    let granularity = CandlesGranularity::OneHour;

    let request = GetCandlesRequest::new(
        &portfolio_id,
        product_id,
        start_time,
        end_time,
        granularity,
    );

    println!("📋 Submitting get_candles request: {:#?}", request);
    match products_service.get_candles(request).await {
        Ok(response) => println!("✅ Candles response: {:#?}", response),
        Err(e) => eprintln!("❌ Failed to get candles: {}", e),
    }

    Ok(())
}
