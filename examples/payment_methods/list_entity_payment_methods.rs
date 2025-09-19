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
// #docs operationId: PrimeRESTAPI_GetEntityPaymentMethods
// #docs operationName: List Payment Methods
use prime_sdk_rs::{
    client::PrimeClient,
    services::payment_methods::{ListEntityPaymentMethodsRequest, PaymentMethodsService},
    utils::PaginatedList,
};
use std::env;

/**
 * Usage:
 * ./run_example.sh payment_methods/list_entity_payment_methods  # Use ENTITY_ID from .env
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    dotenv::dotenv().ok();

    // Get entity ID from environment variable (required)
    let entity_id = env::var("ENTITY_ID")
        .map_err(|_| "ENTITY_ID not found in .env file. Please set ENTITY_ID in your .env file.")?;

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let payment_methods_service = PaymentMethodsService::new(prime_client);
    println!("📋 Fetching payment methods for entity: {}", entity_id);

    let request = ListEntityPaymentMethodsRequest::new(entity_id);
    match payment_methods_service
        .list_entity_payment_methods(request)
        .await
    {
        Ok(response) => {
            println!("✅ Successfully retrieved payment methods");
            println!("\n📋 Payment Methods: {:#?}", response);
            println!("\n📊 Total payment methods: {}", response.count());
        }
        Err(e) => {
            eprintln!("❌ Failed to get payment methods: {}", e);
        }
    }

    Ok(())
}
