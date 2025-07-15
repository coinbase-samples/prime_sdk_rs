use prime_sdk_rs::orders::{OrderPreviewRequest, OrdersService};
use prime_sdk_rs::services::orders::{OrderSide, OrderType, TimeInForceType};
use prime_sdk_rs::PrimeClient;
use std::env;
use time::{Duration, OffsetDateTime};
use time_tz::timezones::get_by_name;
use time_tz::{timezones, OffsetDateTimeExt};

/// Usage:
/// ./run_example.sh orders/order_preview
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    dotenv::dotenv().ok();

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let orders_service = OrdersService::new(prime_client);

    // Only portfolio_id from env
    let portfolio_id = env::var("PORTFOLIO_ID").expect("PORTFOLIO_ID env var required");

    // Hard-coded request fields
    let base_quantity = "5";
    let limit_price = "0.32";
    let product_id = "ADA-USD";

    // Timezone support
    let tz_name = env::var("TIMEZONE").unwrap_or_else(|_| "UTC".to_string());
    let tz = get_by_name(&tz_name).unwrap_or(timezones::db::UTC);
    let expiry_dt = OffsetDateTime::now_utc().to_timezone(tz) + Duration::minutes(5);
    let expiry_time = expiry_dt
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap();

    let request =
        OrderPreviewRequest::new(&portfolio_id, product_id, OrderSide::Buy, OrderType::Limit)
            .with_base_quantity(base_quantity)
            .with_limit_price(limit_price)
            .with_expiry_time(expiry_time)
            .with_time_in_force(TimeInForceType::GoodUntilCancelled);

    println!("📋 Submitting order_preview request: {:#?}", request);
    match orders_service.order_preview(request).await {
        Ok(response) => {
            println!("✅ Order preview successful! Response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to preview order: {}", e);
        }
    }
    Ok(())
}
