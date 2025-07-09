use prime_rs_sdk::client::PrimeClient;
use prime_rs_sdk::services::onchain_address_groups::{
    types::ListOnchainAddressGroupsRequest, OnchainAddressGroupsService,
};
use std::env;

/// Usage:
///   ./run_example.sh examples/onchain_address_book/list_onchain_address_groups
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Get portfolio ID from environment variable (required)
    let portfolio_id = env::var("PORTFOLIO_ID").map_err(|_| {
        "PORTFOLIO_ID environment variable is required. Please set it in your .env file."
    })?;

    println!("🚀 Creating PrimeClient...");
    let client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let service = OnchainAddressGroupsService::new(client);
    let request = ListOnchainAddressGroupsRequest { portfolio_id };

    println!("🔎 Listing Onchain Address Groups...");
    let response = service.list_onchain_address_groups(request).await?;

    println!("\n📦 Address Groups:");
    for (i, group) in response.address_groups.iter().enumerate() {
        println!("  {}. {:#?}", i + 1, group);
    }

    let pagination = response.pagination();
    println!("\n📄 Pagination Info:");
    println!("  has_more: {}", pagination.has_next);
    println!("  next_cursor: {}", pagination.next_cursor);
    println!("  sort_direction: {:?}", pagination.sort_direction);
    println!("  count: {}", response.address_groups.len());

    Ok(())
}
