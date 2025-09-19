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
// #docs operationId: PrimeRESTAPI_GetEntityUsers
// #docs operationName: List Users
use prime_sdk_rs::{
    client::PrimeClient,
    services::users::{ListEntityUsersRequest, UsersService},
    utils::PaginatedList,
    SortDirection,
};
use std::env;

/**
 * Usage:
 * ./run_example.sh examples/users/list_entity_users  # Use ENTITY_ID from .env
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

    let users_service = UsersService::new(prime_client);
    println!("📋 Fetching users for entity: {}", entity_id);

    let request = ListEntityUsersRequest::new(entity_id)
        .with_limit(10)
        .with_sort_direction(SortDirection::Desc);
    match users_service.list_entity_users(request).await {
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
            eprintln!("❌ Failed to list entity users: {}", e);
        }
    }

    Ok(())
}
