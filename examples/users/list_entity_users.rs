use prime_rs_sdk::{
    client::PrimeClient,
    services::users::{ListEntityUsersRequest, UsersService},
    utils::PaginatedList,
    SortDirection,
};
use std::env;

/// Usage:
///   ./run_example.sh examples/users/list_entity_users  # Use ENTITY_ID from .env
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
