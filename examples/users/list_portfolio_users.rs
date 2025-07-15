use prime_sdk_rs::{
    client::PrimeClient,
    services::users::{ListPortfolioUsersRequest, UsersService},
    utils::PaginatedList,
    SortDirection,
};
use std::env;

/// Usage:
///   ./run_example.sh examples/users/list_portfolio_users  # Use PORTFOLIO_ID from .env
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
