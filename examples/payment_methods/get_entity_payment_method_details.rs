use prime_sdk_rs::{
    client::PrimeClient,
    services::payment_methods::{GetEntityPaymentMethodDetailsRequest, PaymentMethodsService},
};
use std::env;

/// Usage:
///   ./run_example.sh payment_methods/get_entity_payment_method_details -- <payment_method_id>  # Use CLI argument for payment_method_id
///   ./run_example.sh payment_methods/get_entity_payment_method_details                          # Use PAYMENT_METHOD_ID from .env
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    dotenv::dotenv().ok();

    // Get entity ID from environment variable (required)
    let entity_id = env::var("ENTITY_ID")
        .map_err(|_| "ENTITY_ID not found in .env file. Please set ENTITY_ID in your .env file.")?;

    // Get payment method ID from command line argument or environment variable (required)
    let payment_method_id = if let Some(arg) = env::args().nth(1) {
        // Use command line argument if provided
        arg
    } else {
        // Fall back to environment variable
        env::var("PAYMENT_METHOD_ID").map_err(|_| {
            "PAYMENT_METHOD_ID not provided as command line argument or found in .env file. \
            Usage: ./run_example.sh payment_methods/get_entity_payment_method_details -- <payment_method_id> \
            or set PAYMENT_METHOD_ID in your .env file."
        })?
    };

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let payment_methods_service = PaymentMethodsService::new(prime_client);
    println!(
        "📋 Fetching payment method details for entity: {} payment method: {}",
        entity_id, payment_method_id
    );

    let request = GetEntityPaymentMethodDetailsRequest::new(entity_id, payment_method_id);
    match payment_methods_service
        .get_entity_payment_method_details(request)
        .await
    {
        Ok(response) => {
            println!("✅ Successfully retrieved payment method details");
            println!("\n📋 Payment Method Details: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to get payment method details: {}", e);
        }
    }

    Ok(())
}
