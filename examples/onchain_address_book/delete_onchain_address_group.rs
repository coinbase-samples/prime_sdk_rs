use prime_rs_sdk::client::PrimeClient;
use prime_rs_sdk::services::onchain_address_groups::{
    types::DeleteOnchainAddressGroupRequest, OnchainAddressGroupsService,
};
use std::env;

/// Usage:
///   ./run_example.sh examples/onchain_address_book/delete_onchain_address_group.rs -- <address_group_id>
///   (PORTFOLIO_ID must always be set in the environment)

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    let portfolio_id = env::var("PORTFOLIO_ID").map_err(|_| {
        "PORTFOLIO_ID environment variable is required. Please set it in your .env file."
    })?;

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("\nUsage: {} <address_group_id>", args[0]);
        eprintln!("  PORTFOLIO_ID must be set in the environment");
        std::process::exit(1);
    }
    let address_group_id = args[1].clone();

    println!("✅ Inputs resolved:");
    println!("  portfolio_id: {}", portfolio_id);
    println!("  address_group_id: {}", address_group_id);

    let client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let service = OnchainAddressGroupsService::new(client);
    let request = DeleteOnchainAddressGroupRequest {
        portfolio_id,
        address_group_id,
    };

    println!("🗑️ Deleting Onchain Address Group...");
    match service.delete_onchain_address_group(request).await {
        Ok(resp) => {
            println!("✅ Deleted Onchain Address Group!");
            println!("{:#?}", resp);
        }
        Err(e) => {
            eprintln!("❌ Failed to delete onchain address group: {}", e);
        }
    }
    Ok(())
}
