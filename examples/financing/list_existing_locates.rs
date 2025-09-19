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
// #docs operationId: PrimeRESTAPI_GetExistingLocates
// #docs operationName: List Existing Locates
use prime_sdk_rs::{
    client::PrimeClient,
    services::financing::{FinancingService, ListExistingLocatesRequest},
};
use std::env;
use std::process;
use time::{format_description, OffsetDateTime};

/**
 * Usage:
 * ./run_example.sh financing/list_existing_locates -- [LOCATE_DATE]
 *
 * Requires PORTFOLIO_ID env var. Optionally takes a locate date (YYYY-MM-DD), defaults to 2025-07-01.
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    let args: Vec<String> = env::args().collect();
    let portfolio_id = env::var("PORTFOLIO_ID").unwrap_or_else(|_| {
        eprintln!("PORTFOLIO_ID env var required");
        process::exit(1);
    });
    let locate_date = if args.len() > 1 {
        Some(args[1].clone())
    } else {
        Some("2025-07-01".to_string())
    };

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let financing_service = FinancingService::new(prime_client);
    let mut request = ListExistingLocatesRequest::new(&portfolio_id);
    if let Some(ld) = locate_date {
        request = request.with_locate_date(ld);
    }
    println!(
        "📋 Submitting list_existing_locates request: {:#?}",
        request
    );
    match financing_service.list_existing_locates(request).await {
        Ok(response) => {
            println!("✅ Existing locates response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to list existing locates: {}", e);
        }
    }
    Ok(())
}
