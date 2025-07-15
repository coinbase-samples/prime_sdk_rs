use prime_sdk_rs::{
    client::PrimeClient,
    services::activities::{ActivitiesService, ListEntityActivitiesRequest},
    utils::PaginatedList,
};
use std::env;

/// Example demonstrating the new PrimeClient struct
///
/// This example shows how to use the PrimeClient struct for a more
/// object-oriented approach to client management.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    // Load .env file
    dotenv::dotenv().ok();

    // Get entity ID from environment variable (required)
    let entity_id = env::var("ENTITY_ID")
        .map_err(|_| "ENTITY_ID not found in .env file. Please set ENTITY_ID in your .env file.")?;

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    // Create activities service
    let activities_service = ActivitiesService::new(prime_client);

    println!("📋 Fetching activities for entity: {}", entity_id);

    // Example: List entity activities
    let request = ListEntityActivitiesRequest::new(&entity_id).with_limit(5);

    match activities_service.list_entity_activities(request).await {
        Ok(response) => {
            println!("✅ Successfully retrieved {} activities", response.count());

            // Display first few activities using PaginatedList trait
            for (i, activity) in response.items().iter().enumerate() {
                println!("\n📋 Activity {}: {:#?}", i + 1, activity);
            }

            // Print pagination info using PaginatedList trait
            println!("\n📄 Pagination Info:");
            println!("   Has more pages: {}", response.has_more());
            if let Some(cursor) = response.next_cursor() {
                println!("   Next cursor: {}", cursor);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to list entity activities: {}", e);
        }
    }

    // Example 3: Create PrimeClient with custom configuration
    println!("\n⚙️ Creating PrimeClient with custom configuration...");
    let custom_client = PrimeClient::with_config(
        "https://api.prime.coinbase.com",
        "MyCustomApp/1.0",
        true, // enable debug
    )?;
    println!("✅ Custom PrimeClient created successfully!");

    // Example 4: Access underlying HTTP client
    println!("\n🔍 Accessing underlying HTTP client...");
    let http_client_ref = custom_client.http_client();
    println!("✅ Successfully accessed underlying HTTP client");

    Ok(())
}
