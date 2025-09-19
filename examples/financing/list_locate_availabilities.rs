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
// #docs operationId: PrimeRESTAPI_GetLocateAvailabilities
// #docs operationName: Get Entity Locate Availabilities
use prime_sdk_rs::{
    client::PrimeClient,
    services::financing::{FinancingService, ListLocateAvailabilitiesRequest},
};
use std::env;

/**
 * Usage:
 * ./run_example.sh financing/list_locate_availabilities [LOCATE_DATE]
 *
 * Requires ENTITY_ID. LOCATE_DATE is optional, defaults to 2025-01-01. Must be YYYY-MM-DD.
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let financing_service = FinancingService::new(prime_client);
    let entity_id = env::var("ENTITY_ID").expect("ENTITY_ID env var required");
    let locate_date = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "2025-01-01".to_string());
    let mut request = ListLocateAvailabilitiesRequest::new(&entity_id);
    request = request.with_locate_date(locate_date);
    println!(
        "📋 Submitting list_locate_availabilities request: {:#?}",
        request
    );
    match financing_service.list_locate_availabilities(request).await {
        Ok(response) => {
            println!("✅ Locate availabilities response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to list locate availabilities: {}", e);
        }
    }
    Ok(())
}
