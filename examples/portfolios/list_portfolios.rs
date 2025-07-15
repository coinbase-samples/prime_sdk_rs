use prime_sdk_rs::{services::portfolios::PortfoliosService, PrimeClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    dotenv::dotenv().ok();

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let portfolios_service = PortfoliosService::new(prime_client);
    println!("📋 Fetching all portfolios...");

    match portfolios_service.list_portfolios().await {
        Ok(response) => {
            println!("✅ Successfully retrieved portfolios");
            println!("\n📋 Portfolios: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to get portfolios: {}", e);
        }
    }

    Ok(())
}
