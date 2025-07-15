use prime_sdk_rs::{
    client::PrimeClient,
    services::financing::{FinancingService, ListLocateAvailabilitiesRequest},
};
use std::env;

/// Usage:
///   ./run_example.sh financing/list_locate_availabilities [LOCATE_DATE]
///
/// Requires ENTITY_ID. LOCATE_DATE is optional, defaults to 2025-01-01. Must be YYYY-MM-DD.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let financing_service = FinancingService::new(prime_client);
    let entity_id = env::var("ENTITY_ID").expect("ENTITY_ID env var required");
    let locate_date = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "2025-01-01".to_string());
    let mut request = ListLocateAvailabilitiesRequest::new(&entity_id);
    request = request.with_locate_date(locate_date);
    println!(
        "📋 Submitting list_locate_availabilities request: {:#?}",
        request
    );
    match financing_service.list_locate_availabilities(request).await {
        Ok(response) => {
            println!("✅ Locate availabilities response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to list locate availabilities: {}", e);
        }
    }
    Ok(())
}
