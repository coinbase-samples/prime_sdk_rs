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
    entity_user::EntityUser,
    get_entity_users_response::GetEntityUsersResponse as GeneratedGetEntityUsersResponse,
    get_portfolio_users_response::GetPortfolioUsersResponse as GeneratedGetPortfolioUsersResponse,
    paginated_response::PaginatedResponse, portfolio_user::PortfolioUser,
    sort_direction::SortDirection,
};
use crate::utils::PaginatedList;

// ============================================================================
// SHARED TRAITS AND BASE STRUCTS
// ============================================================================

/// Base struct for paginated users list responses
#[derive(Debug, Clone)]
pub struct UsersListResponse<T> {
    pub users: Vec<T>,
    pub pagination: PaginatedResponse,
}

impl<T> UsersListResponse<T> {
    pub fn new(users: Vec<T>, pagination: PaginatedResponse) -> Self {
        Self { users, pagination }
    }
}

impl<T> PaginatedList<T> for UsersListResponse<T> {
    fn items(&self) -> &[T] {
        &self.users
    }

    fn pagination(&self) -> &PaginatedResponse {
        &self.pagination
    }
}

/// Base struct for users list requests with common pagination fields
#[derive(Debug, Clone)]
pub struct UsersListRequest {
    pub cursor: Option<String>,
    pub limit: Option<u32>,
    pub sort_direction: Option<SortDirection>,
}

impl UsersListRequest {
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

/// Request to list entity users
#[derive(Debug, Clone)]
pub struct ListEntityUsersRequest {
    pub entity_id: String,
    pub pagination: UsersListRequest,
}

impl ListEntityUsersRequest {
    /// Create a new request to list entity users
    pub fn new(entity_id: impl Into<String>) -> Self {
        Self {
            entity_id: entity_id.into(),
            pagination: UsersListRequest::new(),
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

/// Request to list portfolio users
#[derive(Debug, Clone)]
pub struct ListPortfolioUsersRequest {
    pub portfolio_id: String,
    pub pagination: UsersListRequest,
}

impl ListPortfolioUsersRequest {
    /// Create a new request to list portfolio users
    pub fn new(portfolio_id: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            pagination: UsersListRequest::new(),
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

// ============================================================================
// RESPONSE STRUCTS
// ============================================================================

/// Response for listing entity users
pub type ListEntityUsersResponse = UsersListResponse<EntityUser>;

impl From<GeneratedGetEntityUsersResponse> for ListEntityUsersResponse {
    fn from(response: GeneratedGetEntityUsersResponse) -> Self {
        Self {
            users: response.users.unwrap_or_default(),
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

/// Response for listing portfolio users
pub type ListPortfolioUsersResponse = UsersListResponse<PortfolioUser>;

impl From<GeneratedGetPortfolioUsersResponse> for ListPortfolioUsersResponse {
    fn from(response: GeneratedGetPortfolioUsersResponse) -> Self {
        Self {
            users: response.users.unwrap_or_default(),
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
