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
use crate::constants::DEFAULT_LIMIT;
use crate::types::generated::generated::get_portfolio_products_response::GetPortfolioProductsResponse;
use crate::utils::PaginationParams;
use core_rs::http_client::HttpClient;
use core_rs::http_method::HttpMethod;
use core_rs::http_request::HttpRequest;
use std::collections::HashMap;

use super::types::{
    GetCandlesRequest, GetCandlesResponse, ListPortfolioProductsRequest,
    ListPortfolioProductsResponse,
};
use crate::types::generated::generated::get_candles_response::GetCandlesResponse as GeneratedGetCandlesResponse;

/// Service for interacting with product-related endpoints
pub struct ProductsService {
    client: Box<dyn HttpClient>,
}

impl ProductsService {
    /// Create a new ProductsService with the given PrimeClient
    pub fn new(prime_client: PrimeClient) -> Self {
        Self {
            client: prime_client.into_http_client(),
        }
    }

    /// List tradable products for a given portfolio
    pub async fn list_portfolio_products(
        &self,
        request: ListPortfolioProductsRequest,
    ) -> crate::error::HttpResult<ListPortfolioProductsResponse> {
        let path = format!("portfolios/{}/products", request.portfolio_id);
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;

        // Build query parameters using pagination utility
        let mut query_params = HashMap::new();

        // Add pagination parameters
        let pagination = PaginationParams::new()
            .with_cursor(request.pagination.cursor.as_deref().unwrap_or_default())
            .with_limit(request.pagination.limit.unwrap_or(DEFAULT_LIMIT))
            .with_sort_direction(
                request
                    .pagination
                    .sort_direction
                    .as_ref()
                    .map(|sd| sd.clone())
                    .unwrap_or("DESC".to_string()),
            );
        pagination.add_to_query_params(&mut query_params);

        if !query_params.is_empty() {
            req = req.with_query_params(query_params);
        }

        let resp = self.client.execute(req).await?;
        let response: GetPortfolioProductsResponse = resp.json().await?;
        Ok(response.into())
    }

    pub async fn get_candles(
        &self,
        request: GetCandlesRequest,
    ) -> crate::error::HttpResult<GetCandlesResponse> {
        let path = format!("portfolios/{}/candles", request.portfolio_id);
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let mut query_params = HashMap::new();
        query_params.insert("product_id".to_string(), request.product_id);
        query_params.insert("start_time".to_string(), request.start_time);
        query_params.insert("end_time".to_string(), request.end_time);
        query_params.insert(
            "granularity".to_string(),
            request.granularity.to_string(),
        );
        req = req.with_query_params(query_params);
        let resp = self.client.execute(req).await?;
        let response: GeneratedGetCandlesResponse = resp.json().await?;
        Ok(response)
    }
}
