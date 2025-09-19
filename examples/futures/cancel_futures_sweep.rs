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
// #docs operationId: PrimeRESTAPI_CancelFuturesSweep
// #docs operationName: Cancel Futures Sweep
use prime_sdk_rs::client::PrimeClient;
use prime_sdk_rs::services::futures::{CancelFuturesSweepRequest, FuturesService};
use std::env;

/**
 * Usage:
 * ./run_example.sh futures/cancel_futures_sweep
 *
 * Requires ENTITY_ID environment variable.
 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    println!("🚀 Creating PrimeClient...");
    let prime_client = PrimeClient::new()?;
    println!("✅ PrimeClient created successfully!");

    let futures_service = FuturesService::new(prime_client);
    let entity_id = env::var("ENTITY_ID").expect("ENTITY_ID env var required");

    let request = CancelFuturesSweepRequest::new(&entity_id);
    println!("📋 Submitting cancel_futures_sweep request: {:#?}", request);
    match futures_service.cancel_futures_sweep(request).await {
        Ok(response) => {
            println!("✅ Cancel futures sweep response: {:#?}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to cancel futures sweep: {}", e);
        }
    }
    Ok(())
}
