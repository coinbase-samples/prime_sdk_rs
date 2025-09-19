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
// #docs operationId: PrimeRESTAPI_GetMarginConversions
// #docs operationName: List Margin Conversions
use prime_sdk_rs::{
    client::PrimeClient,
    services::financing::{FinancingService, ListMarginConversionsRequest},
};
use std::env;
use std::process;
use time::{format_description, OffsetDateTime};

/**
 * Usage:
 * ./run_example.sh financing/list_margin_conversions [START_DATE] [END_DATE]
 *
 * Requires PORTFOLIO_ID env var. Optionally takes a start date (YYYY-MM-DD), defaults to 2025-07-01T00:00:00Z. Optionally takes an end date (YYYY-MM-DD).
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
    let start_date = if args.len() > 1 {
        Some(format!("{}T00:00:00Z", args[1]))
    } else {
        Some("2025-07-01T00:00:00Z".to_string())
    };
    let end_date = if args.len() > 2 {
        Some(format!("{}T00:00:00Z", args[2]))
    } else {
        // Fallback: today at 00:00:00Z in RFC3339
        let today = OffsetDateTime::now_utc().date();
        let today_rfc3339 = format!(
            "{}T00:00:00Z",
            today
                .format(&format_description::parse("[year]-[month]-[day]").unwrap())
                .unwrap()
        );
        Some(today_rfc3339)
    };

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let financing_service = FinancingService::new(prime_client);
    let mut request = ListMarginConversionsRequest::new(&portfolio_id);
    if let Some(sd) = start_date {
        request = request.with_start_date(sd);
    }
    if let Some(ed) = end_date {
        request = request.with_end_date(ed);
    }
    println!(
        "📋 Submitting list_margin_conversions request: {:#?}",
        request
    );
    match financing_service.list_margin_conversions(request).await {
        Ok(response) => {
            println!("✅ Margin conversions response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to list margin conversions: {}", e);
        }
    }
    Ok(())
}
