use prime_sdk_rs::client::PrimeClient;
use prime_sdk_rs::services::onchain_address_groups::{
    types::UpdateOnchainAddressGroupRequest, OnchainAddressGroupsService,
};
use prime_sdk_rs::types::generated::generated::address_entry::AddressEntry;
use prime_sdk_rs::types::generated::generated::address_group::AddressGroup;
use prime_sdk_rs::types::generated::generated::network_type::NetworkType;
use std::env;

/// Usage:
///   ./run_example.sh examples/onchain_address_book/update_onchain_address_group.rs -- <group_id> <group_name> <network_type> [<addresses>]
///   (PORTFOLIO_ID must always be set in the environment)

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    let portfolio_id = env::var("PORTFOLIO_ID").map_err(|_| {
        "PORTFOLIO_ID environment variable is required. Please set it in your .env file."
    })?;

    let args: Vec<String> = std::env::args().collect();
    println!("args: {:?}", args);
    if args.len() < 4 {
        eprintln!(
            "\nUsage: {} <group_id> <group_name> <network_type> [<addresses>]",
            args[0]
        );
        eprintln!("  PORTFOLIO_ID must be set in the environment");
        eprintln!("  <addresses> is a comma-separated list (optional)");
        std::process::exit(1);
    }
    let group_id = args[1].clone();
    let group_name = args[2].clone();
    let network_type_str = args[3].clone();
    let addresses_str = if args.len() > 4 {
        args[4].clone()
    } else {
        String::new()
    };

    let network_type = match network_type_str.to_uppercase().as_str() {
        "NETWORK_TYPE_EVM" | "EVM" | "ETHEREUM" => NetworkType::NetworkTypeEvm,
        "NETWORK_TYPE_SOLANA" | "SOLANA" => NetworkType::NetworkTypeSolana,
        _ => {
            eprintln!(
                "Unsupported network_type: {}. Use one of: EVM, ETHEREUM, SOLANA, NETWORK_TYPE_EVM, NETWORK_TYPE_SOLANA",
                network_type_str
            );
            std::process::exit(1);
        }
    };

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
    println!("  group_id: {}", group_id);
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
        id: Some(group_id),
        name: Some(group_name),
        network_type: Some(network_type),
        addresses: if addresses.is_empty() {
            None
        } else {
            Some(addresses)
        },
        ..Default::default()
    };
    let request = UpdateOnchainAddressGroupRequest {
        portfolio_id,
        address_group,
    };

    println!("📝 Updating Onchain Address Group...");
    match service.update_onchain_address_group(request).await {
        Ok(resp) => {
            println!("✅ Updated Onchain Address Group!");
            println!("{:#?}", resp);
        }
        Err(e) => {
            eprintln!("❌ Failed to update onchain address group: {}", e);
        }
    }
    Ok(())
}
