use prime_sdk_rs::{
    services::{ActivitiesService, ListEntityActivitiesRequest, SortDirection},
    utils::PaginatedList,
    PrimeClient,
};
use std::env;

/// Usage:
/// ./run_example.sh list_entity_activities
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

    // Get entity ID from environment (required)
    let entity_id = env::var("ENTITY_ID").map_err(|_| {
        "ENTITY_ID environment variable is required. Please set it in your .env file."
    })?;

    println!("📋 Fetching activities for entity: {}", entity_id);

    // Example: List entity activities
    let request = ListEntityActivitiesRequest::new(&entity_id)
        .with_limit(100)
        .with_sort_direction(SortDirection::Desc);

    match activities_service.list_entity_activities(request).await {
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
            eprintln!("❌ Failed to list entity activities: {}", e);
        }
    }

    Ok(())
}
