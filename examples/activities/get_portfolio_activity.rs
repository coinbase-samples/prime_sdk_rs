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
// #docs operationId: PrimeRESTAPI_GetPortfolioActivity
// #docs operationName: Get Portfolio Activity
use prime_sdk_rs::{
    client::PrimeClient,
    services::activities::{ActivitiesService, GetPortfolioActivityRequest},
};
use std::env;

/**
 * Usage:
 * ./run_example.sh activities/get_portfolio_activity -- <activity_id>  # Use command line argument
 * ./run_example.sh activities/get_portfolio_activity                    # Use PORTFOLIO_ID and ACTIVITY_ID from .env
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    // Load .env file
    dotenv::dotenv().ok();

    // Get portfolio ID from environment variable (required)
    let portfolio_id = env::var("PORTFOLIO_ID").map_err(|_| {
        "PORTFOLIO_ID environment variable is required. Please set it in your .env file."
    })?;

    // Get activity ID from command line argument or environment variable (required)
    let activity_id = if let Some(arg) = env::args().nth(1) {
        // Use command line argument if provided
        arg
    } else {
        // Fall back to environment variable
        env::var("ACTIVITY_ID").map_err(|_| {
            "ACTIVITY_ID not provided as command line argument or found in .env file. \
            Usage: ./run_example.sh activities/get_portfolio_activity -- <activity_id> \
            or set ACTIVITY_ID in your .env file."
        })?
    };

    println!("🚀 Creating PrimeClient...");

    // Create a new PrimeClient with default configuration
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    // Create the activities service
    let activities_service = ActivitiesService::new(prime_client);

    println!("📋 Fetching activity: {}", activity_id);

    // Create request object
    let request = GetPortfolioActivityRequest::new(portfolio_id, activity_id);

    // Example: Get a specific activity by ID
    match activities_service.get_portfolio_activity(request).await {
        Ok(response) => {
            println!("✅ Successfully retrieved activity");

            let activity = response.activity;
            println!("\n📋 Activity Details: {:#?}", activity);
        }
        Err(e) => {
            eprintln!("❌ Failed to get activity: {}", e);
        }
    }

    Ok(())
}
