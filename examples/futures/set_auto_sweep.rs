/**
 * Copyright 2025-present Coinbase Global, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *  http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
// #docs operationId: PrimeRESTAPI_SetAutoSweep
// #docs operationName: Set Auto Sweep
use prime_sdk_rs::{
    client::PrimeClient,
    services::futures::{FuturesService, SetAutoSweepRequest},
};
use std::env;
use std::process;

/**
 * Usage:
 * ./run_example.sh futures/set_auto_sweep -- <true|false>
 *
 * Requires ENTITY_ID environment variable.
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} -- <true|false>", args[0]);
        process::exit(1);
    }
    let auto_sweep = args[1]
        .parse::<bool>()
        .expect("Argument must be true or false");

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let futures_service = FuturesService::new(prime_client);
    let entity_id = env::var("ENTITY_ID").expect("ENTITY_ID env var required");

    let request = SetAutoSweepRequest::new(&entity_id, auto_sweep);
    println!("📋 Submitting set_auto_sweep request: {:#?}", request);
    match futures_service.set_auto_sweep(request).await {
        Ok(response) => {
            println!("✅ Set auto sweep response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to set auto sweep: {}", e);
        }
    }
    Ok(())
}
