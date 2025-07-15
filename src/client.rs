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
use crate::credentials::AuthInterceptor;
use crate::debug::DebugInterceptor;
use core_rs::http_client::{HttpClient, ReqwestClient};
use core_rs::http_headers::HttpHeaders;
use std::sync::Arc;

/// A configured HTTP client for interacting with the Coinbase Prime API
///
/// This struct encapsulates the HTTP client configuration including:
/// - Authentication credentials
/// - Base URL and headers
/// - Debug logging
/// - Request interceptors
///
/// # Example
///
/// ```rust
/// use prime_sdk_rs::client::PrimeClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = PrimeClient::new()?;
///     // Use the client...
///     Ok(())
/// }
/// ```
pub struct PrimeClient {
    client: Box<dyn HttpClient>,
}

impl PrimeClient {
    /// Create a new PrimeClient with default configuration
    ///
    /// This method:
    /// - Loads credentials from environment variables
    /// - Sets up default headers including User-Agent
    /// - Configures authentication and debug interceptors
    /// - Uses the default API base URL
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either:
    /// - `Ok(PrimeClient)` - A configured client
    /// - `Err(Box<dyn std::error::Error>)` - An error if setup fails
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// - Required environment variables are missing
    /// - Credentials cannot be loaded
    /// - Client configuration fails
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Load credentials from .env file
        let auth_interceptor = AuthInterceptor::from_env()?;

        // Set up the client with base URL and default headers
        let mut headers = HttpHeaders::new();
        headers.insert("User-Agent", USER_AGENT);

        let client = ReqwestClient::new()
            .with_base_url(API_BASE_PATH)
            .with_default_headers(headers)
            .with_pre_interceptor(Arc::new(DebugInterceptor))
            .with_pre_interceptor(Arc::new(auth_interceptor));

        Ok(Self {
            client: Box::new(client),
        })
    }

    /// Create a new PrimeClient with custom configuration
    ///
    /// This method allows for more fine-grained control over client setup
    /// compared to the basic `new()` method.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL for the API
    /// * `user_agent` - Custom User-Agent string
    /// * `enable_debug` - Whether to enable debug logging
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either:
    /// - `Ok(PrimeClient)` - A configured client
    /// - `Err(Box<dyn std::error::Error>)` - An error if setup fails
    pub fn with_config(
        base_url: &str,
        user_agent: &str,
        enable_debug: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Load credentials from .env file
        let auth_interceptor = AuthInterceptor::from_env()?;

        // Set up the client with custom base URL and headers
        let mut headers = HttpHeaders::new();
        headers.insert("User-Agent", user_agent);

        let mut client = ReqwestClient::new()
            .with_base_url(base_url)
            .with_default_headers(headers)
            .with_pre_interceptor(Arc::new(auth_interceptor));

        // Conditionally add debug interceptor
        if enable_debug {
            client = client.with_pre_interceptor(Arc::new(DebugInterceptor));
        }

        Ok(Self {
            client: Box::new(client),
        })
    }

    /// Get a reference to the underlying HTTP client
    ///
    /// This allows access to the raw HttpClient trait object for advanced use cases.
    pub fn http_client(&self) -> &dyn HttpClient {
        self.client.as_ref()
    }

    /// Get a mutable reference to the underlying HTTP client
    ///
    /// This allows modification of the raw HttpClient trait object for advanced use cases.
    pub fn http_client_mut(&mut self) -> &mut dyn HttpClient {
        self.client.as_mut()
    }

    /// Consume the PrimeClient and return the underlying HTTP client
    ///
    /// This is useful when you need to take ownership of the HTTP client.
    pub fn into_http_client(self) -> Box<dyn HttpClient> {
        self.client
    }
}

// Implement Deref and DerefMut for convenient access to HttpClient methods
impl std::ops::Deref for PrimeClient {
    type Target = dyn HttpClient;

    fn deref(&self) -> &Self::Target {
        self.client.as_ref()
    }
}

impl std::ops::DerefMut for PrimeClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.client.as_mut()
    }
}
