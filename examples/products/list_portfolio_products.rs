use prime_rs_sdk::{
    client::PrimeClient,
    services::products::{ListPortfolioProductsRequest, ProductsService},
    SortDirection,
};
use std::env;

/// Usage:
///   ./run_example.sh products/list_portfolio_products -- <cursor>  # Use command line argument for cursor
///   ./run_example.sh products/list_portfolio_products              # Use default pagination
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

    // Create products service using the PrimeClient
    let products_service = ProductsService::new(prime_client);

    // Get portfolio ID from environment variable (required)
    let portfolio_id =
        env::var("PORTFOLIO_ID").expect("PORTFOLIO_ID environment variable is required");

    // Get optional cursor from command line argument
    let cursor = env::args().nth(1);

    println!("📋 Listing products for portfolio: {}", portfolio_id);

    // Build the request
    let mut request =
        ListPortfolioProductsRequest::new(&portfolio_id).with_sort_direction(SortDirection::Desc);

    // Add cursor if provided
    if let Some(cursor_value) = cursor {
        request = request.with_cursor(&cursor_value);
        println!("🔍 Using cursor: {}", cursor_value);
    }

    // Make the API call
    let response = products_service.list_portfolio_products(request).await?;

    println!("✅ Successfully retrieved {} products", response.count());

    // Display pagination info
    if response.has_more() {
        println!(
            "📄 More products available. Next cursor: {}",
            response.next_cursor().unwrap_or("")
        );
    } else {
        println!("📄 All products retrieved");
    }

    // Display first few products
    for (i, product) in response.products().iter().enumerate() {
        println!("\n📋 Product {}: {:#?}", i + 1, product);
    }

    // Show some useful statistics
    let tradable_products = response.tradable_products();
    let lending_products = response.lending_products();
    let rfq_products = response.rfq_products();

    println!("\n📊 Product Statistics:");
    println!("   Total Products: {}", response.count());
    println!("   Tradable Products: {}", tradable_products.len());
    println!("   Lending Products: {}", lending_products.len());
    println!("   RFQ Products: {}", rfq_products.len());

    Ok(())
}
