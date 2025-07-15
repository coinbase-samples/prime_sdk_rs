use prime_rs_sdk::{
    client::PrimeClient,
    services::orders::{ListPortfolioFillsRequest, OrdersService},
    utils::PaginatedList,
    SortDirection,
};
use std::env;

/// Usage:
///   ./run_example.sh orders/list_portfolio_fills -- <start_date> [cursor]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let orders_service = OrdersService::new(prime_client);
    let portfolio_id = env::var("PORTFOLIO_ID").map_err(|_| {
        "PORTFOLIO_ID environment variable is required. Please set it in your .env file."
    })?;
    let mut args = env::args().skip(1);
    let start_date = match args.next() {
        Some(date) => date,
        None => {
            let default = "2025-01-01T00:00:00Z".to_string();
            println!("No start_date provided, using default: {}", default);
            default
        }
    };
    let cursor_arg = args.next();

    println!(
        "📋 Fetching fills for portfolio: {} from {}",
        portfolio_id, start_date
    );

    let mut request = ListPortfolioFillsRequest::new(&portfolio_id, &start_date)
        .with_limit(100)
        .with_sort_direction(SortDirection::Desc);
    if let Some(cursor) = cursor_arg {
        request = request.with_cursor(&cursor);
    }

    match orders_service.list_portfolio_fills(request).await {
        Ok(response) => {
            println!("✅ Successfully retrieved {} fills", response.count());
            println!("📄 Has more pages: {}", response.has_more());
            if let Some(cursor) = response.next_cursor() {
                println!("📄 Next cursor: {}", cursor);
            }
            println!("📄 Total fills in this response: {}", response.count());
            for (i, fill) in response.items().iter().take(3).enumerate() {
                println!("\n📋 Fill {}: {:#?}", i + 1, fill);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to list portfolio fills: {}", e);
        }
    }
    Ok(())
}
