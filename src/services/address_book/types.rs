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
    address_book_entry::AddressBookEntry,
    create_portfolio_address_book_entry_response::CreatePortfolioAddressBookEntryResponse,
    custody_activity_type::CustodyActivityType,
    get_portfolio_address_book_response::GetPortfolioAddressBookResponse,
    paginated_response::PaginatedResponse, sort_direction::SortDirection,
};
use crate::utils::PaginatedList;

// ============================================================================
// SHARED TRAITS AND BASE STRUCTS
// ============================================================================

/// Base struct for paginated address book list responses
#[derive(Debug, Clone)]
pub struct AddressBookListResponse {
    pub entries: Vec<AddressBookEntry>,
    pub pagination: PaginatedResponse,
}

impl AddressBookListResponse {
    pub fn new(entries: Vec<AddressBookEntry>, pagination: PaginatedResponse) -> Self {
        Self {
            entries,
            pagination,
        }
    }
}

impl PaginatedList<AddressBookEntry> for AddressBookListResponse {
    fn items(&self) -> &[AddressBookEntry] {
        &self.entries
    }

    fn pagination(&self) -> &PaginatedResponse {
        &self.pagination
    }
}

/// Base struct for address book list requests with common pagination fields
#[derive(Debug, Clone)]
pub struct AddressBookListRequest {
    pub cursor: Option<String>,
    pub limit: Option<u32>,
    pub sort_direction: Option<SortDirection>,
}

impl AddressBookListRequest {
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

/// Request parameters for listing address book entries
#[derive(Debug, Clone)]
pub struct ListAddressBookRequest {
    pub portfolio_id: String,
    pub pagination: AddressBookListRequest,
}

impl ListAddressBookRequest {
    pub fn new(portfolio_id: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            pagination: AddressBookListRequest::new(),
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

/// Request parameters for creating an address book entry
#[derive(Debug, Clone)]
pub struct CreateAddressBookEntryRequest {
    pub portfolio_id: String,
    pub address: String,
    pub currency_symbol: String,
    pub name: String,
    pub account_identifier: Option<String>,
}

impl CreateAddressBookEntryRequest {
    pub fn new(
        portfolio_id: impl Into<String>,
        address: impl Into<String>,
        currency_symbol: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            address: address.into(),
            currency_symbol: currency_symbol.into(),
            name: name.into(),
            account_identifier: None,
        }
    }

    pub fn with_account_identifier(mut self, account_identifier: impl Into<String>) -> Self {
        self.account_identifier = Some(account_identifier.into());
        self
    }
}

// ============================================================================
// RESPONSE STRUCTS
// ============================================================================

/// Wrapper for address book list response
pub type ListAddressBookResponse = AddressBookListResponse;

impl From<GetPortfolioAddressBookResponse> for AddressBookListResponse {
    fn from(response: GetPortfolioAddressBookResponse) -> Self {
        Self {
            entries: response.addresses,
            pagination: *response.pagination,
        }
    }
}

/// Wrapper for address book get response
pub struct GetAddressBookResponse {
    pub entries: Vec<AddressBookEntry>,
    pub pagination: PaginatedResponse,
}

impl From<GetPortfolioAddressBookResponse> for GetAddressBookResponse {
    fn from(response: GetPortfolioAddressBookResponse) -> Self {
        Self {
            entries: response.addresses,
            pagination: *response.pagination,
        }
    }
}

impl GetAddressBookResponse {
    /// Get all address book entries
    pub fn entries(&self) -> &[AddressBookEntry] {
        &self.entries
    }

    /// Get the pagination information
    pub fn pagination(&self) -> &PaginatedResponse {
        &self.pagination
    }
}

/// Wrapper for create address book entry response
pub struct CreateAddressBookEntryResponse {
    pub activity_type: CustodyActivityType,
    pub num_approvals_remaining: u32,
    pub activity_id: String,
}

impl From<CreatePortfolioAddressBookEntryResponse> for CreateAddressBookEntryResponse {
    fn from(response: CreatePortfolioAddressBookEntryResponse) -> Self {
        Self {
            activity_type: response.activity_type,
            num_approvals_remaining: response.num_approvals_remaining as u32,
            activity_id: response.activity_id,
        }
    }
}

impl CreateAddressBookEntryResponse {
    /// Get the activity type
    pub fn activity_type(&self) -> &CustodyActivityType {
        &self.activity_type
    }

    pub fn num_approvals_remaining(&self) -> u32 {
        self.num_approvals_remaining
    }

    pub fn activity_id(&self) -> &String {
        &self.activity_id
    }
}
