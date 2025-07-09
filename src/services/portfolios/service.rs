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
use crate::client::PrimeClient;
use crate::types::generated::generated::get_portfolios_response::GetPortfoliosResponse;
use core_rs::http_client::HttpClient;
use core_rs::http_method::HttpMethod;
use core_rs::http_request::HttpRequest;

use super::types::{GetPortfolioRequest, GetPortfolioResponse, ListPortfoliosResponse};

/// Service for interacting with portfolio-related endpoints
pub struct PortfoliosService {
    client: Box<dyn HttpClient>,
}

impl PortfoliosService {
    /// Create a new PortfoliosService with the given PrimeClient
    pub fn new(prime_client: PrimeClient) -> Self {
        Self {
            client: prime_client.into_http_client(),
        }
    }

    /// List all portfolios for which the current API key has read access
    pub async fn list_portfolios(&self) -> crate::error::HttpResult<ListPortfoliosResponse> {
        let path = "portfolios";
        let req = HttpRequest::new(HttpMethod::Get, path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;

        let resp = self.client.execute(req).await?;
        let response: GetPortfoliosResponse = resp.json().await?;
        Ok(response)
    }

    /// Get a specific portfolio by its ID
    pub async fn get_portfolio(
        &self,
        request: GetPortfolioRequest,
    ) -> crate::error::HttpResult<GetPortfolioResponse> {
        let path = format!("portfolios/{}", request.portfolio_id);
        let req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;

        let resp = self.client.execute(req).await?;
        let response: GetPortfolioResponse = resp.json().await?;
        Ok(response)
    }
}
