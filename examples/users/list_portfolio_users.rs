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
// #docs operationId: PrimeRESTAPI_GetPortfolioUsers
// #docs operationName: List Portfolio Users
use prime_sdk_rs::{
    client::PrimeClient,
    services::users::{ListPortfolioUsersRequest, UsersService},
    utils::PaginatedList,
    SortDirection,
};
use std::env;

/**
 * Usage:
 * ./run_example.sh examples/users/list_portfolio_users  # Use PORTFOLIO_ID from .env
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    dotenv::dotenv().ok();

    // Get portfolio ID from environment variable (required)
    let portfolio_id = env::var("PORTFOLIO_ID").map_err(|_| {
        "PORTFOLIO_ID not found in .env file. Please set PORTFOLIO_ID in your .env file."
    })?;

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let users_service = UsersService::new(prime_client);
    println!("📋 Fetching users for portfolio: {}", portfolio_id);

    let request = ListPortfolioUsersRequest::new(portfolio_id)
        .with_limit(10)
        .with_sort_direction(SortDirection::Desc);
    match users_service.list_portfolio_users(request).await {
        Ok(response) => {
            println!("✅ Successfully retrieved {} users", response.count());

            // Display users using PaginatedList trait
            for (i, user) in response.items().iter().enumerate() {
                println!("\n👤 User {}: {:#?}", i + 1, user);
            }

            // Print pagination info using PaginatedList trait
            println!("\n📄 Pagination Info:");
            println!("   Has more pages: {}", response.has_more());
            if let Some(cursor) = response.next_cursor() {
                println!("   Next cursor: {}", cursor);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to list portfolio users: {}", e);
        }
    }

    Ok(())
}
