use prime_rs_sdk::{
    client::PrimeClient,
    services::invoices::{InvoiceService, ListInvoicesRequest},
};
use std::env;

/// Usage:
///   ./run_example.sh invoices/list_entity_invoices  # Use ENTITY_ID from .env
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

    let invoice_service = InvoiceService::new(prime_client);
    println!("📋 Fetching invoices for entity: {}", entity_id);

    let request = ListInvoicesRequest::new(entity_id);

    match invoice_service.list_invoices(request).await {
        Ok(response) => {
            println!("✅ Successfully retrieved {} invoices", response.count());
            println!("📊 Response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to get invoices: {}", e);
        }
    }

    Ok(())
}
