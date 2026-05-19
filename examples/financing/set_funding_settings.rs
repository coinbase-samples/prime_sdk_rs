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
// #docs operationId: PrimeBeta_SetFundingSettings
// #docs operationName: Update Funding Settings (Beta)
use prime_sdk_rs::services::financing::{FinancingService, SetFundingSettingsRequest};
use prime_sdk_rs::types::generated::generated::set_funding_settings_request::SetFundingSettingsRequest as SetFundingSettingsBody;
use prime_sdk_rs::PrimeClient;
use std::env;

/**
 * Usage:
 * ./run_example.sh financing/set_funding_settings
 *
 * Requires ENTITY_ID and PORTFOLIO_ID in .env (portfolio used as derivatives funding portfolio).
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let service = FinancingService::new(prime_client);
    let entity_id = env::var("ENTITY_ID").expect("ENTITY_ID env var required");
    let portfolio_id = env::var("PORTFOLIO_ID").expect("PORTFOLIO_ID env var required");

    let body = SetFundingSettingsBody::new(
        portfolio_id,
        true,  // automatic_conversion_enabled
        false, // automatic_loan_enabled
        true,  // automatic_excess_return_enabled
        "1000.00".to_string(),
    );

    let request = SetFundingSettingsRequest::new(&entity_id, body);
    println!("📋 Submitting set_funding_settings request: {:#?}", request);
    match service.set_funding_settings(request).await {
        Ok(response) => println!("✅ Funding settings updated: {:#?}", response),
        Err(e) => eprintln!("❌ Failed to set funding settings: {}", e),
    }

    Ok(())
}
