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
// #docs operationId: PrimeRESTAPI_GetPortfolioActivities
// #docs operationName: List Portfolio Activities
use prime_sdk_rs::{
    client::PrimeClient,
    services::activities::{ActivitiesService, ListPortfolioActivitiesRequest},
    utils::PaginatedList,
    SortDirection,
};
use std::env;

/**
 * Usage:
 * ./run_example.sh activities/list_portfolio_activities -- <cursor>  # Use command line argument for cursor
 * ./run_example.sh activities/list_portfolio_activities              # Use default pagination
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    println!("🚀 Creating PrimeClient...");

    // Create a new PrimeClient with default configuration
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    // Create activities service using the PrimeClient
    let activities_service = ActivitiesService::new(prime_client);

    // Get portfolio ID from environment variable (required)
    let portfolio_id = env::var("PORTFOLIO_ID").map_err(|_| {
        "PORTFOLIO_ID environment variable is required. Please set it in your .env file."
    })?;

    // Get optional cursor from command line argument
    let cursor_arg = env::args().nth(1);

    println!("📋 Fetching activities for portfolio: {}", portfolio_id);

    // Example: List portfolio activities
    let mut request = ListPortfolioActivitiesRequest::new(&portfolio_id)
        .with_limit(100)
        .with_sort_direction(SortDirection::Desc);
    if let Some(cursor) = cursor_arg {
        request = request.with_cursor(&cursor);
    }

    match activities_service.list_portfolio_activities(request).await {
        Ok(response) => {
            println!("✅ Successfully retrieved {} activities", response.count());

            // Demonstrate the shared PaginatedList trait methods
            println!("📄 Has more pages: {}", response.has_more());
            if let Some(cursor) = response.next_cursor() {
                println!("📄 Next cursor: {}", cursor);
            }
            println!("📄 Total activities in this response: {}", response.count());

            // Display first few activities
            for (i, activity) in response.items().iter().take(3).enumerate() {
                println!("\n📋 Activity {}: {:#?}", i + 1, activity);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to list portfolio activities: {}", e);
        }
    }

    Ok(())
}
