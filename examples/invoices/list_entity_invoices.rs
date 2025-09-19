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
// #docs operationId: PrimeRESTAPI_GetInvoices
// #docs operationName: List Invoices
use prime_sdk_rs::{
    client::PrimeClient,
    services::invoices::{InvoiceService, ListInvoicesRequest},
};
use std::env;

/**
 * Usage:
 * ./run_example.sh invoices/list_entity_invoices  # Use ENTITY_ID from .env
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

    let invoice_service = InvoiceService::new(prime_client);
    println!("📋 Fetching invoices for entity: {}", entity_id);

    let request = ListInvoicesRequest::new(entity_id);

    match invoice_service.list_invoices(request).await {
        Ok(response) => {
            println!("✅ Successfully retrieved {} invoices", response.count());
            println!("📊 Response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to get invoices: {}", e);
        }
    }

    Ok(())
}
