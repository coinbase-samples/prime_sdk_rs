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
// #docs operationId: PrimeRESTAPI_ListTFObligations
// #docs operationName: List Trade Finance Obligations
use prime_sdk_rs::services::financing::{FinancingService, ListTfObligationsRequest};
use prime_sdk_rs::PrimeClient;
use std::env;

/**
 * Usage:
 * ./run_example.sh financing/list_tf_obligations
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

    let request = ListTfObligationsRequest::new(&entity_id);
    println!("📋 Submitting list_tf_obligations request: {:#?}", request);
    match service.list_tf_obligations(request).await {
        Ok(response) => println!("✅ TF obligations: {:#?}", response),
        Err(e) => eprintln!("❌ Failed to list TF obligations: {}", e),
    }

    Ok(())
}
