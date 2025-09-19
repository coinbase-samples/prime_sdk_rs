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
// #docs operationId: PrimeRESTAPI_CreateNewLocates
// #docs operationName: Create New Locates
use prime_sdk_rs::client::PrimeClient;
use prime_sdk_rs::services::financing::{CreateNewLocatesRequest, FinancingService};
use std::env;
use tokio;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let portfolio_id = env::args()
        .nth(1)
        .or_else(|| env::var("PORTFOLIO_ID").ok())
        .expect("portfolio_id must be provided as first arg or PORTFOLIO_ID env var");
    let symbol = env::args()
        .nth(2)
        .or_else(|| env::var("SYMBOL").ok())
        .unwrap_or_else(|| "BTC".to_string());
    let amount = env::args()
        .nth(3)
        .or_else(|| env::var("AMOUNT").ok())
        .unwrap_or_else(|| "100".to_string());
    let locate_date = env::args().nth(4).or_else(|| env::var("LOCATE_DATE").ok());

    let mut request = CreateNewLocatesRequest::new(portfolio_id, symbol, amount);
    if let Some(date) = locate_date {
        request = request.with_locate_date(date);
    }

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let financing_service = FinancingService::new(prime_client);

    match financing_service.create_new_locates(request).await {
        Ok(response) => {
            println!("✅ Create new locates response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Error: {e}");
        }
    }
}
