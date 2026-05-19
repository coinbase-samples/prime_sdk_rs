/**
 * Copyright 2026-present Coinbase Global, Inc.
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
// #docs operationId: PrimeRESTAPI_SetFcmSettings
// #docs operationName: Set FCM Settings
use prime_sdk_rs::services::futures::{FuturesService, SetFcmSettingsRequest};
use prime_sdk_rs::types::generated::generated::set_fcm_settings_request::SetFcmSettingsRequest as SetFcmSettingsBody;
use prime_sdk_rs::PrimeClient;
use std::env;

/**
 * Usage:
 * ./run_example.sh futures/set_fcm_settings
 *
 * Optionally pass target_derivatives_excess as the first argument (default "1000.00").
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

    let target_derivatives_excess = env::args()
        .nth(1)
        .unwrap_or_else(|| "1000.00".to_string());

    let mut body = SetFcmSettingsBody::new();
    body.target_derivatives_excess = Some(target_derivatives_excess);

    let request = SetFcmSettingsRequest::new(&entity_id, body);
    println!("📋 Submitting set_fcm_settings request: {:#?}", request);
    match futures_service.set_fcm_settings(request).await {
        Ok(response) => println!("✅ FCM settings updated: {:#?}", response),
        Err(e) => eprintln!("❌ Failed to set FCM settings: {}", e),
    }

    Ok(())
}
