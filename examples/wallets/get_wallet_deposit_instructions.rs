use prime_sdk_rs::services::{GetWalletDepositInstructionsRequest, WalletsService};
use prime_sdk_rs::PrimeClient;
use std::env;

/// Usage:
///   ./run_example.sh examples/wallets/get_wallet_deposit_instructions  # Use PORTFOLIO_ID and WALLET_ID from .env
///   ./run_example.sh examples/wallets/get_wallet_deposit_instructions -- <wallet_id>  # Use wallet_id from CLI
///   ./run_example.sh examples/wallets/get_wallet_deposit_instructions -- <wallet_id> CRYPTO ethereum mainnet  # Get CRYPTO deposit instructions for Ethereum mainnet
///   ./run_example.sh examples/wallets/get_wallet_deposit_instructions -- <wallet_id> CRYPTO solana mainnet   # Get CRYPTO deposit instructions for Solana mainnet
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    let portfolio_id = env::var("PORTFOLIO_ID").map_err(|_| {
        "PORTFOLIO_ID not found in .env file. Please set PORTFOLIO_ID in your .env file."
    })?;

    // Accept wallet_id as a command-line argument, fallback to env
    let mut args = env::args().skip(1);
    let wallet_id = args
        .next()
        .or_else(|| env::var("WALLET_ID").ok())
        .ok_or("WALLET_ID not provided as CLI arg or in .env file.")?;
    println!("🔑 Using wallet_id: {}", wallet_id);

    // Optionally accept deposit_type, network_id, network_type as CLI args
    let deposit_type = args.next();
    let network_id = args.next();
    let network_type = args.next();

    let deposit_type_enum = deposit_type.as_deref().and_then(|dt| {
        use prime_sdk_rs::types::generated::generated::wallet_deposit_instruction_type::WalletDepositInstructionType;
        match dt.to_uppercase().as_str() {
            "CRYPTO" => Some(WalletDepositInstructionType::Crypto),
            "WIRE" => Some(WalletDepositInstructionType::Wire),
            "SEN" => Some(WalletDepositInstructionType::Sen),
            "SWIFT" => Some(WalletDepositInstructionType::Swift),
            "SEPA" => Some(WalletDepositInstructionType::Sepa),
            _ => None,
        }
    });

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let service = WalletsService::new(prime_client);
    println!(
        "📋 Fetching deposit instructions for wallet {} in portfolio: {}",
        wallet_id, portfolio_id
    );

    let mut request = GetWalletDepositInstructionsRequest::new(portfolio_id, wallet_id);
    if let Some(dt) = deposit_type_enum {
        request = request.with_deposit_type(dt);
    }
    if let Some(nid) = network_id {
        request = request.with_network_id(nid);
    }
    if let Some(ntype) = network_type {
        request = request.with_network_type(ntype);
    }

    match service.get_wallet_deposit_instructions(request).await {
        Ok(response) => {
            println!("✅ Successfully retrieved deposit instructions");
            println!("\n🏦 Deposit Instructions: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to get deposit instructions: {}", e);
        }
    }
    Ok(())
}
