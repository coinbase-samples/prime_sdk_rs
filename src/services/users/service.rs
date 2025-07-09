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
    get_entity_users_response::GetEntityUsersResponse as GeneratedGetEntityUsersResponse,
    get_portfolio_users_response::GetPortfolioUsersResponse as GeneratedGetPortfolioUsersResponse,
};
use crate::utils::PaginationParams;
use core_rs::http_client::HttpClient;
use core_rs::http_method::HttpMethod;
use core_rs::http_request::HttpRequest;
use std::collections::HashMap;

use super::types::{
    ListEntityUsersRequest, ListEntityUsersResponse, ListPortfolioUsersRequest,
    ListPortfolioUsersResponse,
};

/// Service for managing users
pub struct UsersService {
    client: Box<dyn HttpClient>,
}

impl UsersService {
    /// Create a new users service
    pub fn new(prime_client: PrimeClient) -> Self {
        Self {
            client: prime_client.into_http_client(),
        }
    }

    /// List all users associated with a given entity
    pub async fn list_entity_users(
        &self,
        request: ListEntityUsersRequest,
    ) -> crate::error::HttpResult<ListEntityUsersResponse> {
        let path = format!("entities/{}/users", request.entity_id);
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
        let response: GeneratedGetEntityUsersResponse = resp.json().await?;
        Ok(response.into())
    }

    /// List all users associated with a given portfolio
    pub async fn list_portfolio_users(
        &self,
        request: ListPortfolioUsersRequest,
    ) -> crate::error::HttpResult<ListPortfolioUsersResponse> {
        let path = format!("portfolios/{}/users", request.portfolio_id);
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
        let response: GeneratedGetPortfolioUsersResponse = resp.json().await?;
        Ok(response.into())
    }
}
