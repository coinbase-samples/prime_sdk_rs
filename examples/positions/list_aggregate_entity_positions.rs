use prime_rs_sdk::{
    client::PrimeClient,
    services::positions::{ListAggregateEntityPositionsRequest, PositionsService},
};
use std::env;

/// Usage:
///   ./run_example.sh positions/list_aggregate_entity_positions  # Use ENTITY_ID from .env
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    dotenv::dotenv().ok();

    // Get entity ID from environment variable
    let entity_id = env::var("ENTITY_ID").expect("ENTITY_ID environment variable must be set");

    // Create the Prime client
    let client = PrimeClient::new()?;
    let positions_service = PositionsService::new(client);

    // Create request object
    let request = ListAggregateEntityPositionsRequest::new(entity_id).with_limit(10);

    // List aggregate entity positions
    let response = positions_service
        .list_aggregate_entity_positions(request)
        .await?;

    println!("Aggregate Entity Positions:");
    println!("Total positions: {}", response.count());

    for position in response.positions() {
        println!("Position: {:#?}", position);
    }

    // Print pagination info
    if let Some(next_cursor) = response.next_cursor() {
        println!("Next cursor: {}", next_cursor);
    }

    Ok(())
}
