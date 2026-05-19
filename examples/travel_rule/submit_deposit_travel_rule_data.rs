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
// #docs operationId: PrimeRESTAPI_SubmitDepositTravelRuleData
// #docs operationName: Submit Deposit Travel Rule Data
use prime_sdk_rs::services::travel_rule::{
    SubmitDepositTravelRuleDataRequest, TravelRuleService,
};
use prime_sdk_rs::types::generated::generated::request_to_submit_travel_rule_data_for_an_existing_deposit_transaction::RequestToSubmitTravelRuleDataForAnExistingDepositTransaction as SubmitTravelRuleBody;
use prime_sdk_rs::PrimeClient;
use std::env;

/**
 * Usage:
 * ./run_example.sh travel_rule/submit_deposit_travel_rule_data -- <transaction_id>
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let service = TravelRuleService::new(prime_client);
    let portfolio_id = env::var("PORTFOLIO_ID").expect("PORTFOLIO_ID env var required");
    let transaction_id = env::args().nth(1).expect("transaction_id argument required");

    let mut body = SubmitTravelRuleBody::new();
    body.is_self = Some(true);

    let request = SubmitDepositTravelRuleDataRequest::new(&portfolio_id, &transaction_id, body);

    println!("📋 Submitting submit_deposit_travel_rule_data request: {:#?}", request);
    match service.submit_deposit_travel_rule_data(request).await {
        Ok(response) => println!("✅ Travel rule deposit submitted: {:#?}", response),
        Err(e) => eprintln!("❌ Failed to submit deposit travel rule data: {}", e),
    }

    Ok(())
}
