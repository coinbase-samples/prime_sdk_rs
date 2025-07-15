use prime_sdk_rs::{
    client::PrimeClient,
    services::orders::{ListOpenOrdersRequest, OrdersService},
    utils::PaginatedList,
    SortDirection,
};
use std::env;

/// Usage:
///   ./run_example.sh orders/list_open_orders -- <cursor>
///   ./run_example.sh orders/list_open_orders
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
    let cursor_arg = env::args().nth(1);

    println!("📋 Fetching open orders for portfolio: {}", portfolio_id);

    let mut request = ListOpenOrdersRequest::new(&portfolio_id)
        .with_limit(100)
        .with_sort_direction(SortDirection::Desc);
    if let Some(cursor) = cursor_arg {
        request = request.with_cursor(&cursor);
    }

    match orders_service.list_open_orders(request).await {
        Ok(response) => {
            println!("✅ Successfully retrieved {} open orders", response.count());
            println!("📄 Has more pages: {}", response.has_more());
            if let Some(cursor) = response.next_cursor() {
                println!("📄 Next cursor: {}", cursor);
            }
            println!(
                "📄 Total open orders in this response: {}",
                response.count()
            );
            for (i, order) in response.items().iter().take(5).enumerate() {
                println!("\n📋 Open Order {}: {:#?}", i + 1, order);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to list open orders: {}", e);
        }
    }
    Ok(())
}
