use prime_sdk_rs::{
    client::PrimeClient,
    services::address_book::{AddressBookService, ListAddressBookRequest},
    utils::PaginatedList,
    SortDirection,
};
use std::env;

/// Usage:
///   ./run_example.sh address_book/list_address_book -- <cursor>  # Use command line argument for cursor
///   ./run_example.sh address_book/list_address_book              # Use default pagination
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

    // Create address book service using the PrimeClient
    let address_book_service = AddressBookService::new(prime_client);

    // Get portfolio ID from environment variable (required)
    let portfolio_id = env::var("PORTFOLIO_ID").map_err(|_| {
        "PORTFOLIO_ID environment variable is required. Please set it in your .env file."
    })?;

    // Get optional cursor from command line argument
    let cursor_arg = env::args().nth(1);

    println!(
        "📋 Fetching address book entries for portfolio: {}",
        portfolio_id
    );

    // Example: List address book entries
    let mut request = ListAddressBookRequest::new(&portfolio_id)
        .with_limit(100)
        .with_sort_direction(SortDirection::Desc);
    if let Some(cursor) = cursor_arg {
        request = request.with_cursor(&cursor);
    }

    match address_book_service.list_address_book(request).await {
        Ok(response) => {
            println!(
                "✅ Successfully retrieved {} address book entries",
                response.count()
            );

            // Demonstrate the shared PaginatedList trait methods
            println!("📄 Has more pages: {}", response.has_more());
            if let Some(cursor) = response.next_cursor() {
                println!("📄 Next cursor: {}", cursor);
            }
            println!("📄 Total entries in this response: {}", response.count());

            // Display first few entries
            for (i, entry) in response.items().iter().take(3).enumerate() {
                println!("\n📋 Address Book Entry {}: {:#?}", i + 1, entry);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to list address book entries: {}", e);
        }
    }

    Ok(())
}
