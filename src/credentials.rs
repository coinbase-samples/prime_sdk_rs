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
use crate::constants::*;
use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use core_rs::error::{HttpError, HttpResult};
use core_rs::http_request::HttpRequest;
use core_rs::interceptor::PreRequestInterceptor;
use dotenv::dotenv;
use hmac::{Hmac, Mac};
use serde::Deserialize;
use sha2::Sha256;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize)]
pub struct Credentials {
    #[serde(rename = "AccessKey")]
    access_key: String,
    #[serde(rename = "SecretKey")]
    secret_key: String,
    #[serde(rename = "Passphrase")]
    passphrase: String,
}

// Interceptor to add HMAC-SHA256 Authorization header
pub struct AuthInterceptor {
    access_key: String,
    secret_key: String,
    passphrase: String,
}

impl AuthInterceptor {
    pub fn new(
        access_key: impl Into<String>,
        secret_key: impl Into<String>,
        passphrase: impl Into<String>,
    ) -> Self {
        Self {
            access_key: access_key.into(),
            secret_key: secret_key.into(),
            passphrase: passphrase.into(),
        }
    }

    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        // Load .env file
        dotenv().ok();

        // Get credentials from environment variable
        let credentials_json = env::var("PRIME_CREDENTIALS")
            .map_err(|_| "PRIME_CREDENTIALS environment variable not found")?;

        // Parse JSON credentials
        let credentials: Credentials = serde_json::from_str(&credentials_json)
            .map_err(|e| format!("Failed to parse PRIME_CREDENTIALS JSON: {}", e))?;

        Ok(Self::new(
            credentials.access_key,
            credentials.secret_key,
            credentials.passphrase,
        ))
    }

    fn generate_signature(
        &self,
        timestamp: &str,
        method: &str,
        url_path: &str,
        body: Option<&str>,
    ) -> String {
        // Build message: timestamp + method + url_path + body (if present)
        let message = if let Some(body_str) = body {
            format!("{}{}{}{}", timestamp, method, url_path, body_str)
        } else {
            format!("{}{}{}", timestamp, method, url_path)
        };

        // Create HMAC-SHA256 signature
        let mut mac = Hmac::<Sha256>::new_from_slice(self.secret_key.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(message.as_bytes());
        let signature = mac.finalize();

        // Encode signature as base64
        BASE64.encode(signature.into_bytes())
    }
}

#[async_trait]
impl PreRequestInterceptor for AuthInterceptor {
    async fn intercept(&self, request: &mut HttpRequest) -> HttpResult<()> {
        let method = request.get_method();
        let url_path = request.get_url_path();

        // Extract body for signature
        let body_for_signature = if let Some(json_body) = &request.json_body {
            Some(json_body.to_string())
        } else {
            None
        };

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();

        let signature = self.generate_signature(
            &timestamp,
            method,
            url_path,
            body_for_signature.as_deref(),
        );

        request
            .add_header(CB_ACCESS_KEY_HEADER, &self.access_key)
            .map_err(|e| {
                HttpError::Custom(format!("Failed to add access key header: {e}"))
            })?;
        request
            .add_header(CB_ACCESS_SIGNATURE_HEADER, &signature)
            .map_err(|e| {
                HttpError::Custom(format!("Failed to add signature header: {e}"))
            })?;
        request
            .add_header(CB_ACCESS_TIMESTAMP_HEADER, &timestamp)
            .map_err(|e| {
                HttpError::Custom(format!("Failed to add timestamp header: {e}"))
            })?;
        request
            .add_header(CB_ACCESS_PHRASE_HEADER, &self.passphrase)
            .map_err(|e| {
                HttpError::Custom(format!("Failed to add passphrase header: {e}"))
            })?;

        Ok(())
    }
}
