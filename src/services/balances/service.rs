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
use crate::types::generated::generated::{
    get_portfolio_balances_response::GetPortfolioBalancesResponse,
    get_wallet_balance_response::GetWalletBalanceResponse as GeneratedGetWalletBalanceResponse,
    list_entity_balances_response::ListEntityBalancesResponse as GeneratedListEntityBalancesResponse,
    list_web3_wallet_balances_response::ListWeb3WalletBalancesResponse as GeneratedListWeb3WalletBalancesResponse,
};
use crate::utils::PaginationParams;
use core_rs::http_client::HttpClient;
use core_rs::http_method::HttpMethod;
use core_rs::http_request::HttpRequest;
use std::collections::HashMap;

use super::types::{
    GetWalletBalanceRequest, GetWalletBalanceResponse, ListEntityBalancesRequest,
    ListEntityBalancesResponse, ListPortfolioBalancesRequest, ListPortfolioBalancesResponse,
    ListWeb3WalletBalancesRequest, ListWeb3WalletBalancesResponse,
};

/// Service for interacting with balance-related endpoints
pub struct BalancesService {
    client: Box<dyn HttpClient>,
}

impl BalancesService {
    /// Create a new BalancesService with the given PrimeClient
    pub fn new(prime_client: PrimeClient) -> Self {
        Self {
            client: prime_client.into_http_client(),
        }
    }

    /// List all balances for a portfolio
    pub async fn list_portfolio_balances(
        &self,
        request: ListPortfolioBalancesRequest,
    ) -> crate::error::HttpResult<ListPortfolioBalancesResponse> {
        let path = format!("portfolios/{}/balances", request.portfolio_id);
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;

        // Build query parameters using pagination utility
        let mut query_params = HashMap::new();

        // Add pagination parameters
        let mut pagination = PaginationParams::new()
            .with_limit(request.pagination.limit.unwrap_or(DEFAULT_LIMIT))
            .with_sort_direction(
                request
                    .pagination
                    .sort_direction
                    .as_ref()
                    .map(|sd| match sd {
                        crate::SortDirection::Desc => "DESC",
                        crate::SortDirection::Asc => "ASC",
                    })
                    .unwrap_or("DESC"),
            );

        // Only add cursor if it's not empty
        if let Some(cursor) = &request.pagination.cursor {
            if !cursor.is_empty() {
                pagination = pagination.with_cursor(cursor);
            }
        }

        pagination.add_to_query_params(&mut query_params);

        if !query_params.is_empty() {
            req = req.with_query_params(query_params);
        }

        let resp = self.client.execute(req).await?;
        let response: GetPortfolioBalancesResponse = resp.json().await?;
        Ok(response.into())
    }

    /// List all balances for an entity
    pub async fn list_entity_balances(
        &self,
        request: ListEntityBalancesRequest,
    ) -> crate::error::HttpResult<ListEntityBalancesResponse> {
        let path = format!("entities/{}/balances", request.entity_id);
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;

        // Build query parameters using pagination utility
        let mut query_params = HashMap::new();

        // Add pagination parameters
        let mut pagination = PaginationParams::new()
            .with_limit(request.pagination.limit.unwrap_or(DEFAULT_LIMIT))
            .with_sort_direction(
                request
                    .pagination
                    .sort_direction
                    .as_ref()
                    .map(|sd| match sd {
                        crate::SortDirection::Desc => "DESC",
                        crate::SortDirection::Asc => "ASC",
                    })
                    .unwrap_or("DESC"),
            );

        // Only add cursor if it's not empty
        if let Some(cursor) = &request.pagination.cursor {
            if !cursor.is_empty() {
                pagination = pagination.with_cursor(cursor);
            }
        }

        pagination.add_to_query_params(&mut query_params);

        if !query_params.is_empty() {
            req = req.with_query_params(query_params);
        }

        let resp = self.client.execute(req).await?;
        let response: GeneratedListEntityBalancesResponse = resp.json().await?;
        Ok(response.into())
    }

    /// Get balance for a specific wallet
    pub async fn get_wallet_balance(
        &self,
        request: GetWalletBalanceRequest,
    ) -> crate::error::HttpResult<GetWalletBalanceResponse> {
        let path = format!(
            "portfolios/{}/wallets/{}/balance",
            request.portfolio_id, request.wallet_id
        );
        let req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;

        let resp = self.client.execute(req).await?;
        let response: GeneratedGetWalletBalanceResponse = resp.json().await?;
        Ok(response.into())
    }

    /// List web3 balances for a specific wallet
    pub async fn list_web3_wallet_balances(
        &self,
        request: ListWeb3WalletBalancesRequest,
    ) -> crate::error::HttpResult<ListWeb3WalletBalancesResponse> {
        let path = format!(
            "portfolios/{}/wallets/{}/web3_balances",
            request.portfolio_id, request.wallet_id
        );
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;

        // Build query parameters
        let mut query_params = HashMap::new();

        // Add pagination parameters
        let mut pagination =
            PaginationParams::new().with_limit(request.pagination.limit.unwrap_or(DEFAULT_LIMIT));

        // Only add cursor if it's not empty
        if let Some(cursor) = &request.pagination.cursor {
            if !cursor.is_empty() {
                pagination = pagination.with_cursor(cursor);
            }
        }

        pagination.add_to_query_params(&mut query_params);

        // Add visibility statuses if provided
        if let Some(visibility_statuses) = &request.visibility_statuses {
            for status in visibility_statuses {
                query_params.insert("visibility_statuses".to_string(), status.clone());
            }
        }

        if !query_params.is_empty() {
            req = req.with_query_params(query_params);
        }

        let resp = self.client.execute(req).await?;
        let response: GeneratedListWeb3WalletBalancesResponse = resp.json().await?;
        Ok(response.into())
    }
}
