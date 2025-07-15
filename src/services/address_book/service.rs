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
    create_portfolio_address_book_entry_request::CreatePortfolioAddressBookEntryRequest as GeneratedCreatePortfolioAddressBookEntryRequest,
    create_portfolio_address_book_entry_response::CreatePortfolioAddressBookEntryResponse,
    get_portfolio_address_book_response::GetPortfolioAddressBookResponse,
};
use crate::utils::PaginationParams;
use core_rs::http_client::HttpClient;
use core_rs::http_method::HttpMethod;
use core_rs::http_request::HttpRequest;
use std::collections::HashMap;

use super::types::{
    CreateAddressBookEntryRequest, CreateAddressBookEntryResponse, ListAddressBookRequest,
    ListAddressBookResponse,
};

/// Service for interacting with address book-related endpoints
pub struct AddressBookService {
    client: Box<dyn HttpClient>,
}

impl AddressBookService {
    /// Create a new AddressBookService with the given PrimeClient
    pub fn new(prime_client: PrimeClient) -> Self {
        Self {
            client: prime_client.into_http_client(),
        }
    }

    /// List all address book entries for a portfolio
    pub async fn list_address_book(
        &self,
        request: ListAddressBookRequest,
    ) -> crate::error::HttpResult<ListAddressBookResponse> {
        let path = format!("portfolios/{}/address_book", request.portfolio_id);
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
        let response: GetPortfolioAddressBookResponse = resp.json().await?;
        Ok(response.into())
    }

    /// Create a new address book entry for a portfolio
    pub async fn create_entry(
        &self,
        request: CreateAddressBookEntryRequest,
    ) -> crate::error::HttpResult<CreateAddressBookEntryResponse> {
        let path = format!("portfolios/{}/address_book", request.portfolio_id);

        // Create the actual API request body
        let request_body = GeneratedCreatePortfolioAddressBookEntryRequest {
            address: request.address,
            currency_symbol: request.currency_symbol,
            name: request.name,
            account_identifier: request.account_identifier,
        };

        let req = HttpRequest::new(HttpMethod::Post, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?
            .with_json_body(
                serde_json::to_value(&request_body)
                    .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?,
            );

        let resp = self.client.execute(req).await?;
        let response: CreatePortfolioAddressBookEntryResponse = resp.json().await?;
        Ok(response.into())
    }
}
