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
use crate::types::generated::generated::{
    candles_granularity::CandlesGranularity,
    get_candles_response::GetCandlesResponse as GeneratedGetCandlesResponse,
    get_portfolio_products_response::GetPortfolioProductsResponse,
    paginated_response::PaginatedResponse, product::Product,
    product_permissions::ProductPermissions, sort_direction::SortDirection,
};
use crate::utils::PaginationParams;

/// Wrapper for the list portfolio products response
pub struct ListPortfolioProductsResponse {
    pub products: Vec<Product>,
    pub pagination: PaginatedResponse,
}

impl From<GetPortfolioProductsResponse> for ListPortfolioProductsResponse {
    fn from(response: GetPortfolioProductsResponse) -> Self {
        Self {
            products: response.products.unwrap_or_default(),
            pagination: response
                .pagination
                .map(|p| *p)
                .unwrap_or_else(|| PaginatedResponse {
                    has_next: false,
                    next_cursor: String::new(),
                    sort_direction: SortDirection::Desc,
                }),
        }
    }
}

impl ListPortfolioProductsResponse {
    /// Get all products from the response
    pub fn products(&self) -> &[Product] {
        &self.products
    }

    /// Get the count of products
    pub fn count(&self) -> usize {
        self.products.len()
    }

    /// Get a product by ID
    pub fn get_by_id(&self, product_id: &str) -> Option<&Product> {
        self.products
            .iter()
            .find(|product| product.id.as_ref().map_or(false, |id| id == product_id))
    }

    /// Get products that have RFQ details
    pub fn rfq_products(&self) -> Vec<&Product> {
        self.products
            .iter()
            .filter(|product| product.rfq_product_details.is_some())
            .collect()
    }

    /// Get products that have specific permissions
    pub fn products_with_permission(&self, permission: ProductPermissions) -> Vec<&Product> {
        self.products
            .iter()
            .filter(|product| {
                product.permissions.as_ref().map_or(false, |permissions| {
                    permissions.iter().any(|p| p == &permission)
                })
            })
            .collect()
    }

    /// Get products that support trading (have trade permission)
    pub fn tradable_products(&self) -> Vec<&Product> {
        self.products_with_permission(ProductPermissions::ProductPermissionTrade)
    }

    /// Get products that support lending (have lending permission)
    pub fn lending_products(&self) -> Vec<&Product> {
        self.products_with_permission(ProductPermissions::ProductPermissionLending)
    }

    /// Check if there are more pages available
    pub fn has_more(&self) -> bool {
        self.pagination.has_next
    }

    /// Get the next cursor for pagination
    pub fn next_cursor(&self) -> Option<&str> {
        if self.pagination.next_cursor.is_empty() {
            None
        } else {
            Some(&self.pagination.next_cursor)
        }
    }

    /// Get the sort direction used
    pub fn sort_direction(&self) -> &SortDirection {
        &self.pagination.sort_direction
    }
}

/// Request for listing portfolio products
pub struct ListPortfolioProductsRequest {
    pub portfolio_id: String,
    pub pagination: PaginationParams,
}

impl ListPortfolioProductsRequest {
    /// Create a new request to list products for a portfolio
    pub fn new(portfolio_id: &str) -> Self {
        Self {
            portfolio_id: portfolio_id.to_string(),
            pagination: PaginationParams::new(),
        }
    }

    /// Set the cursor for pagination
    pub fn with_cursor(mut self, cursor: &str) -> Self {
        self.pagination = self.pagination.with_cursor(cursor);
        self
    }

    /// Set the limit for pagination
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.pagination = self.pagination.with_limit(limit);
        self
    }

    /// Set the sort direction
    pub fn with_sort_direction(mut self, sort_direction: SortDirection) -> Self {
        self.pagination = self.pagination.with_sort_direction(match sort_direction {
            SortDirection::Desc => "DESC",
            SortDirection::Asc => "ASC",
        });
        self
    }
}

#[derive(Debug, Clone)]
pub struct GetCandlesRequest {
    pub portfolio_id: String,
    pub product_id: String,
    pub start_time: String,
    pub end_time: String,
    pub granularity: CandlesGranularity,
}

impl GetCandlesRequest {
    pub fn new(
        portfolio_id: impl Into<String>,
        product_id: impl Into<String>,
        start_time: impl Into<String>,
        end_time: impl Into<String>,
        granularity: CandlesGranularity,
    ) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            product_id: product_id.into(),
            start_time: start_time.into(),
            end_time: end_time.into(),
            granularity,
        }
    }
}

pub type GetCandlesResponse = GeneratedGetCandlesResponse;
