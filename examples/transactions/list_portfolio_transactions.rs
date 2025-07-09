use prime_rs_sdk::services::{ListPortfolioTransactionsRequest, TransactionsService};
use prime_rs_sdk::PrimeClient;
use std::env;

/// Usage:
///   ./run_example.sh examples/transactions/list_portfolio_transactions  # Uses PORTFOLIO_ID from .env
///   ./run_example.sh examples/transactions/list_portfolio_transactions -- <cursor>  # Optionally provide a cursor for pagination
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    let mut args = env::args().skip(1);
    let portfolio_id = env::var("PORTFOLIO_ID").or_else(|_| {
        args.next()
            .ok_or("PORTFOLIO_ID required as env var or first CLI arg")
    })?;
    let cursor = args.next();
    println!("📁 Portfolio ID: {}", portfolio_id);

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let service = TransactionsService::new(prime_client);
    println!("📋 Fetching transactions for portfolio: {}", portfolio_id);

    let mut req = ListPortfolioTransactionsRequest::new(portfolio_id).with_limit(20);
    if let Some(cursor) = cursor {
        req = req.with_cursor(cursor);
    }

    match service.list_portfolio_transactions(req).await {
        Ok(resp) => {
            let txs = resp.transactions;
            println!("📄 Found {} transactions:", txs.len());
            for (i, tx) in txs.iter().enumerate() {
                println!("  {}. Transaction: {:#?},", i + 1, tx);
            }
            if let Some(pagination) = resp.pagination {
                println!("📄 Pagination: {:#?}", pagination);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to list transactions: {}", e);
        }
    }
    println!("✅ Done.");
    Ok(())
}
