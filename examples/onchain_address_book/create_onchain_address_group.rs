use prime_sdk_rs::client::PrimeClient;
use prime_sdk_rs::services::onchain_address_groups::{
    types::CreateOnchainAddressGroupRequest, OnchainAddressGroupsService,
};
use prime_sdk_rs::types::generated::generated::address_entry::AddressEntry;
use prime_sdk_rs::types::generated::generated::address_group::AddressGroup;
use prime_sdk_rs::types::generated::generated::network_type::NetworkType;
use std::env;

/// Usage:
///   ./run_example.sh examples/onchain_address_book/create_onchain_address_group.rs -- <group_name> <network_type> [<addresses>]
///   (PORTFOLIO_ID must always be set in the environment)

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging and load env
    env_logger::init();
    dotenv::dotenv().ok();

    // Get portfolio ID from environment variable (required)
    let portfolio_id = env::var("PORTFOLIO_ID").map_err(|_| {
        "PORTFOLIO_ID environment variable is required. Please set it in your .env file."
    })?;

    // Accept group_name, network_type, and addresses as CLI args or env vars
    let args: Vec<String> = std::env::args().collect();
    let group_name = if args.len() > 1 {
        args[1].clone()
    } else {
        env::var("GROUP_NAME")
            .map_err(|_| "GROUP_NAME env var or CLI arg required (1st argument)")?
    };
    let network_type_str = if args.len() > 2 {
        args[2].clone()
    } else {
        env::var("NETWORK_TYPE")
            .map_err(|_| "NETWORK_TYPE env var or CLI arg required (2nd argument)")?
    };
    let addresses_str = if args.len() > 3 {
        args[3].clone()
    } else {
        env::var("ADDRESSES").unwrap_or_default()
    };
    let network_type = match network_type_str.to_uppercase().as_str() {
        "NETWORK_TYPE_EVM" | "EVM" | "ETHEREUM" => NetworkType::NetworkTypeEvm,
        "NETWORK_TYPE_SOLANA" | "SOLANA" => NetworkType::NetworkTypeSolana,
        _ => panic!(
            "Unsupported network_type: {}. Use one of: EVM, ETHEREUM, SOLANA, NETWORK_TYPE_EVM, NETWORK_TYPE_SOLANA",
            network_type_str
        ),
    };

    // Parse addresses (comma-separated)
    let addresses: Vec<AddressEntry> = if addresses_str.trim().is_empty() {
        vec![]
    } else {
        let chain_ids = match network_type {
            NetworkType::NetworkTypeSolana => vec!["101".to_string()],
            NetworkType::NetworkTypeEvm => vec!["1".to_string(), "8453".to_string()],
            _ => vec![],
        };
        addresses_str
            .split(',')
            .enumerate()
            .map(|(i, addr)| AddressEntry {
                address: Some(addr.trim().to_string()),
                name: Some(format!("address {}", i + 1)),
                chain_ids: Some(chain_ids.clone()),
                ..Default::default()
            })
            .collect()
    };

    println!("✅ Inputs resolved:");
    println!("  portfolio_id: {}", portfolio_id);
    println!("  group_name: {}", group_name);
    println!("  network_type: {:?}", network_type);
    if !addresses.is_empty() {
        println!("  addresses: {:?}", addresses_str);
    } else {
        println!("  addresses: (none)");
    }

    let client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let service = OnchainAddressGroupsService::new(client);
    let address_group = AddressGroup {
        name: Some(group_name),
        network_type: Some(network_type),
        addresses: if addresses.is_empty() {
            None
        } else {
            Some(addresses)
        },
        ..Default::default()
    };
    let request = CreateOnchainAddressGroupRequest {
        portfolio_id,
        address_group,
    };

    println!("📝 Creating Onchain Address Group...");
    match service.create_onchain_address_group(request).await {
        Ok(resp) => {
            println!("✅ Created Onchain Address Group!");
            println!("{:#?}", resp);
        }
        Err(e) => {
            eprintln!("❌ Failed to create onchain address group: {}", e);
        }
    }
    Ok(())
}
