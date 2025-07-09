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
use crate::types::generated::generated::get_portfolio_commission_response::GetPortfolioCommissionResponse as GeneratedGetPortfolioCommissionResponse;
use core_rs::http_client::HttpClient;
use core_rs::http_method::HttpMethod;
use core_rs::http_request::HttpRequest;
use std::collections::HashMap;

use super::types::{GetPortfolioCommissionRequest, GetPortfolioCommissionResponse};

/// Service for managing commission
pub struct CommissionService {
    client: Box<dyn HttpClient>,
}

impl CommissionService {
    /// Create a new commission service
    pub fn new(prime_client: PrimeClient) -> Self {
        Self {
            client: prime_client.into_http_client(),
        }
    }

    /// Get commission associated with a given portfolio
    pub async fn get_portfolio_commission(
        &self,
        request: GetPortfolioCommissionRequest,
    ) -> crate::error::HttpResult<GetPortfolioCommissionResponse> {
        let path = format!("portfolios/{}/commission", request.portfolio_id);
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;

        // Add query parameters if product_id is provided
        if let Some(product_id) = request.product_id {
            let mut query_params = HashMap::new();
            query_params.insert("product_id".to_string(), product_id);
            req = req.with_query_params(query_params);
        }

        let resp = self.client.execute(req).await?;
        let response: GeneratedGetPortfolioCommissionResponse = resp.json().await?;
        Ok(response.into())
    }
}
