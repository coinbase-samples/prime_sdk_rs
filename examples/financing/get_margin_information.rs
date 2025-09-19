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
// #docs operationId: PrimeRESTAPI_GetMarginInformation
// #docs operationName: Get Margin Information
use prime_sdk_rs::{
    client::PrimeClient,
    services::financing::{FinancingService, GetMarginInformationRequest},
};
use std::env;
use std::process;

/**
 * Usage:
 * ./run_example.sh financing/get_margin_information
 *
 * Requires ENTITY_ID env var.
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    let entity_id = env::var("ENTITY_ID").unwrap_or_else(|_| {
        eprintln!("ENTITY_ID env var required");
        process::exit(1);
    });

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let financing_service = FinancingService::new(prime_client);
    let request = GetMarginInformationRequest::new(&entity_id);
    println!(
        "📋 Submitting get_margin_information request: {:#?}",
        request
    );
    match financing_service.get_margin_information(request).await {
        Ok(response) => {
            println!("✅ Margin information response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to get margin information: {}", e);
        }
    }
    Ok(())
}
