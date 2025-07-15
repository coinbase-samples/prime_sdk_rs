use prime_sdk_rs::client::PrimeClient;
use prime_sdk_rs::services::financing::{CreateNewLocatesRequest, FinancingService};
use std::env;
use tokio;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let portfolio_id = env::args()
        .nth(1)
        .or_else(|| env::var("PORTFOLIO_ID").ok())
        .expect("portfolio_id must be provided as first arg or PORTFOLIO_ID env var");
    let symbol = env::args()
        .nth(2)
        .or_else(|| env::var("SYMBOL").ok())
        .unwrap_or_else(|| "BTC".to_string());
    let amount = env::args()
        .nth(3)
        .or_else(|| env::var("AMOUNT").ok())
        .unwrap_or_else(|| "100".to_string());
    let locate_date = env::args().nth(4).or_else(|| env::var("LOCATE_DATE").ok());

    let mut request = CreateNewLocatesRequest::new(portfolio_id, symbol, amount);
    if let Some(date) = locate_date {
        request = request.with_locate_date(date);
    }

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let financing_service = FinancingService::new(prime_client);

    match financing_service.create_new_locates(request).await {
        Ok(response) => {
            println!("✅ Create new locates response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Error: {e}");
        }
    }
}
