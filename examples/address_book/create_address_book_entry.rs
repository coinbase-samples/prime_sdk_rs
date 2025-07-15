use prime_sdk_rs::{
    services::{AddressBookService, CreateAddressBookEntryRequest},
    PrimeClient,
};
use std::env;

/// Usage:
/// ./run_example.sh create_address_book_entry
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

    println!(
        "📋 Creating address book entry for portfolio: {}",
        portfolio_id
    );

    // Example: Create a new address book entry with the four required fields
    let request = CreateAddressBookEntryRequest::new(
        &portfolio_id,
        "0x1234567890123456789012345678901234567890", // address
        "ETH",                                        // currency_symbol
        "Example ETH Address",                        // name
    );
    //.with_account_identifier("memo123"); // optional account identifier for memo/destination tag based networks

    match address_book_service.create_entry(request).await {
        Ok(response) => {
            println!("✅ Successfully created address book entry!");
            println!("📋 Activity ID: {}", response.activity_id);
            println!("📋 Activity Type: {:?}", response.activity_type);
            println!(
                "📋 Approvals Remaining: {}",
                response.num_approvals_remaining
            );
        }
        Err(e) => {
            eprintln!("❌ Failed to create address book entry: {}", e);
        }
    }

    Ok(())
}
