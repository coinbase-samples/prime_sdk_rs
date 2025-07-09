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
    balance::Balance, defi_balance::DefiBalance, entity_balance::EntityBalance,
    get_portfolio_balances_response::GetPortfolioBalancesResponse as GeneratedGetPortfolioBalancesResponse,
    get_wallet_balance_response::GetWalletBalanceResponse as GeneratedGetWalletBalanceResponse,
    list_entity_balances_response::ListEntityBalancesResponse as GeneratedListEntityBalancesResponse,
    list_web3_wallet_balances_response::ListWeb3WalletBalancesResponse as GeneratedListWeb3WalletBalancesResponse,
    paginated_response::PaginatedResponse, sort_direction::SortDirection,
    web3_balance::Web3Balance,
};
use crate::utils::PaginatedList;

// ============================================================================
// SHARED TRAITS AND BASE STRUCTS
// ============================================================================

/// Base struct for paginated balance list responses
#[derive(Debug, Clone)]
pub struct BalanceListResponse {
    pub balances: Vec<Balance>,
    pub pagination: PaginatedResponse,
}

impl BalanceListResponse {
    pub fn new(balances: Vec<Balance>, pagination: PaginatedResponse) -> Self {
        Self {
            balances,
            pagination,
        }
    }
}

impl PaginatedList<Balance> for BalanceListResponse {
    fn items(&self) -> &[Balance] {
        &self.balances
    }

    fn pagination(&self) -> &PaginatedResponse {
        &self.pagination
    }
}

/// Base struct for entity balance list responses
#[derive(Debug, Clone)]
pub struct EntityBalanceListResponse {
    pub balances: Vec<EntityBalance>,
    pub pagination: PaginatedResponse,
}

impl EntityBalanceListResponse {
    pub fn new(balances: Vec<EntityBalance>, pagination: PaginatedResponse) -> Self {
        Self {
            balances,
            pagination,
        }
    }
}

impl PaginatedList<EntityBalance> for EntityBalanceListResponse {
    fn items(&self) -> &[EntityBalance] {
        &self.balances
    }

    fn pagination(&self) -> &PaginatedResponse {
        &self.pagination
    }
}

/// Base struct for balance list requests with common pagination fields
#[derive(Debug, Clone)]
pub struct BalanceListRequest {
    pub cursor: Option<String>,
    pub limit: Option<u32>,
    pub sort_direction: Option<SortDirection>,
}

impl BalanceListRequest {
    pub fn new() -> Self {
        Self {
            cursor: None,
            limit: None,
            sort_direction: None,
        }
    }

    pub fn with_cursor(mut self, cursor: impl Into<String>) -> Self {
        let cursor_str = cursor.into();
        if !cursor_str.is_empty() {
            self.cursor = Some(cursor_str);
        }
        self
    }

    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_sort_direction(mut self, sort_direction: SortDirection) -> Self {
        self.sort_direction = Some(sort_direction);
        self
    }
}

// ============================================================================
// REQUEST STRUCTS
// ============================================================================

/// Request parameters for listing portfolio balances
#[derive(Debug, Clone)]
pub struct ListPortfolioBalancesRequest {
    pub portfolio_id: String,
    pub pagination: BalanceListRequest,
}

impl ListPortfolioBalancesRequest {
    pub fn new(portfolio_id: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            pagination: BalanceListRequest::new(),
        }
    }

    // Delegate pagination methods to the inner struct
    pub fn with_cursor(mut self, cursor: impl Into<String>) -> Self {
        let cursor_str = cursor.into();
        if !cursor_str.is_empty() {
            self.pagination = self.pagination.with_cursor(cursor_str);
        }
        self
    }

    pub fn with_limit(mut self, limit: u32) -> Self {
        self.pagination = self.pagination.with_limit(limit);
        self
    }

    pub fn with_sort_direction(mut self, sort_direction: SortDirection) -> Self {
        self.pagination = self.pagination.with_sort_direction(sort_direction);
        self
    }
}

/// Request parameters for listing entity balances
#[derive(Debug, Clone)]
pub struct ListEntityBalancesRequest {
    pub entity_id: String,
    pub pagination: BalanceListRequest,
}

impl ListEntityBalancesRequest {
    pub fn new(entity_id: impl Into<String>) -> Self {
        Self {
            entity_id: entity_id.into(),
            pagination: BalanceListRequest::new(),
        }
    }

    // Delegate pagination methods to the inner struct
    pub fn with_cursor(mut self, cursor: impl Into<String>) -> Self {
        let cursor_str = cursor.into();
        if !cursor_str.is_empty() {
            self.pagination = self.pagination.with_cursor(cursor_str);
        }
        self
    }

    pub fn with_limit(mut self, limit: u32) -> Self {
        self.pagination = self.pagination.with_limit(limit);
        self
    }

    pub fn with_sort_direction(mut self, sort_direction: SortDirection) -> Self {
        self.pagination = self.pagination.with_sort_direction(sort_direction);
        self
    }
}

/// Request parameters for getting wallet balance
#[derive(Debug, Clone)]
pub struct GetWalletBalanceRequest {
    pub portfolio_id: String,
    pub wallet_id: String,
}

impl GetWalletBalanceRequest {
    pub fn new(portfolio_id: impl Into<String>, wallet_id: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            wallet_id: wallet_id.into(),
        }
    }
}

/// Request parameters for listing web3 wallet balances
#[derive(Debug, Clone)]
pub struct ListWeb3WalletBalancesRequest {
    pub portfolio_id: String,
    pub wallet_id: String,
    pub pagination: BalanceListRequest,
    pub visibility_statuses: Option<Vec<String>>,
}

impl ListWeb3WalletBalancesRequest {
    pub fn new(portfolio_id: impl Into<String>, wallet_id: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            wallet_id: wallet_id.into(),
            pagination: BalanceListRequest::new(),
            visibility_statuses: None,
        }
    }

    // Delegate pagination methods to the inner struct
    pub fn with_cursor(mut self, cursor: impl Into<String>) -> Self {
        let cursor_str = cursor.into();
        if !cursor_str.is_empty() {
            self.pagination = self.pagination.with_cursor(cursor_str);
        }
        self
    }

    pub fn with_limit(mut self, limit: u32) -> Self {
        self.pagination = self.pagination.with_limit(limit);
        self
    }

    pub fn with_sort_direction(mut self, sort_direction: SortDirection) -> Self {
        self.pagination = self.pagination.with_sort_direction(sort_direction);
        self
    }

    pub fn with_visibility_statuses(mut self, visibility_statuses: Vec<String>) -> Self {
        self.visibility_statuses = Some(visibility_statuses);
        self
    }
}

// ============================================================================
// RESPONSE STRUCTS
// ============================================================================

/// Wrapper for portfolio balance list response
pub type ListPortfolioBalancesResponse = BalanceListResponse;

impl From<GeneratedGetPortfolioBalancesResponse> for BalanceListResponse {
    fn from(response: GeneratedGetPortfolioBalancesResponse) -> Self {
        Self {
            balances: response.balances.unwrap_or_default(),
            pagination: PaginatedResponse::default(), // Portfolio balances don't have pagination
        }
    }
}

/// Wrapper for entity balance list response
pub type ListEntityBalancesResponse = EntityBalanceListResponse;

impl From<GeneratedListEntityBalancesResponse> for EntityBalanceListResponse {
    fn from(response: GeneratedListEntityBalancesResponse) -> Self {
        Self {
            balances: response.balances,
            pagination: *response.pagination,
        }
    }
}

/// Wrapper for wallet balance response
#[derive(Debug)]
pub struct GetWalletBalanceResponse {
    pub balance: Option<Balance>,
}

impl From<GeneratedGetWalletBalanceResponse> for GetWalletBalanceResponse {
    fn from(response: GeneratedGetWalletBalanceResponse) -> Self {
        Self {
            balance: response.balance.map(|b| *b),
        }
    }
}

/// Base struct for web3 wallet balance list responses
#[derive(Debug, Clone)]
pub struct Web3WalletBalanceListResponse {
    pub balances: Vec<Web3Balance>,
    pub pagination: Option<PaginatedResponse>,
    pub defi_balances: Vec<DefiBalance>,
}

impl Web3WalletBalanceListResponse {
    pub fn new(
        balances: Vec<Web3Balance>,
        pagination: Option<PaginatedResponse>,
        defi_balances: Vec<DefiBalance>,
    ) -> Self {
        Self {
            balances,
            pagination,
            defi_balances,
        }
    }
}

/// Wrapper for web3 wallet balance list response
pub type ListWeb3WalletBalancesResponse = Web3WalletBalanceListResponse;

impl From<GeneratedListWeb3WalletBalancesResponse> for Web3WalletBalanceListResponse {
    fn from(response: GeneratedListWeb3WalletBalancesResponse) -> Self {
        Self {
            balances: response.balances.unwrap_or_default(),
            pagination: response.pagination.map(|p| *p),
            defi_balances: response.defi_balances.unwrap_or_default(),
        }
    }
}

impl ListPortfolioBalancesResponse {
    /// Get all balances
    pub fn balances(&self) -> &[Balance] {
        &self.balances
    }

    /// Get the pagination information
    pub fn pagination(&self) -> &PaginatedResponse {
        &self.pagination
    }

    /// Get balances by currency symbol
    pub fn get_by_currency(&self, currency: &str) -> Vec<&Balance> {
        self.balances
            .iter()
            .filter(|balance| balance.symbol.as_deref() == Some(currency))
            .collect()
    }

    /// Get total balance for a specific currency
    pub fn get_total_for_currency(&self, currency: &str) -> Option<f64> {
        let currency_balances = self.get_by_currency(currency);
        if currency_balances.is_empty() {
            None
        } else {
            Some(
                currency_balances
                    .iter()
                    .map(|balance| {
                        balance
                            .amount
                            .as_ref()
                            .and_then(|s| s.parse::<f64>().ok())
                            .unwrap_or(0.0)
                    })
                    .sum(),
            )
        }
    }

    /// Get total count of balances (for consistency with PaginatedList)
    pub fn count(&self) -> usize {
        self.balances.len()
    }

    /// Check if there are more pages available (portfolio balances don't have pagination)
    pub fn has_more(&self) -> bool {
        false // Portfolio balances don't support pagination
    }

    /// Get the next cursor for pagination (portfolio balances don't have pagination)
    pub fn next_cursor(&self) -> Option<&str> {
        None // Portfolio balances don't support pagination
    }
}

impl ListEntityBalancesResponse {
    /// Get all entity balances
    pub fn balances(&self) -> &[EntityBalance] {
        &self.balances
    }

    /// Get the pagination information
    pub fn pagination(&self) -> &PaginatedResponse {
        &self.pagination
    }

    /// Get entity balances by currency symbol
    pub fn get_by_currency(&self, currency: &str) -> Vec<&EntityBalance> {
        self.balances
            .iter()
            .filter(|balance| balance.symbol.as_deref() == Some(currency))
            .collect()
    }
}

impl GetWalletBalanceResponse {
    /// Get the wallet balance
    pub fn balance(&self) -> Option<&Balance> {
        self.balance.as_ref()
    }
}

impl ListWeb3WalletBalancesResponse {
    /// Get all web3 balances
    pub fn balances(&self) -> &[Web3Balance] {
        &self.balances
    }

    /// Get the pagination information
    pub fn pagination(&self) -> Option<&PaginatedResponse> {
        self.pagination.as_ref()
    }

    /// Get all DeFi balances
    pub fn defi_balances(&self) -> &[DefiBalance] {
        &self.defi_balances
    }

    /// Get web3 balances by currency symbol
    pub fn get_by_currency(&self, currency: &str) -> Vec<&Web3Balance> {
        self.balances
            .iter()
            .filter(|balance| {
                balance.asset.as_ref().and_then(|a| a.symbol.as_deref()) == Some(currency)
            })
            .collect()
    }

    /// Get DeFi balances by currency symbol
    pub fn get_defi_by_currency(&self, currency: &str) -> Vec<&DefiBalance> {
        self.defi_balances
            .iter()
            .filter(|balance| balance.network.as_deref() == Some(currency))
            .collect()
    }

    /// Get total count of web3 balances (for PaginatedList compatibility)
    pub fn count(&self) -> usize {
        self.balances.len()
    }

    /// Check if there are more pages available
    pub fn has_more(&self) -> bool {
        self.pagination
            .as_ref()
            .map(|p| p.has_next)
            .unwrap_or(false)
    }

    /// Get the next cursor for pagination
    pub fn next_cursor(&self) -> Option<&str> {
        self.pagination.as_ref().and_then(|p| {
            if p.next_cursor.is_empty() {
                None
            } else {
                Some(p.next_cursor.as_str())
            }
        })
    }
}
