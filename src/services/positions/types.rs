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
    list_aggregate_entity_positions_response::ListAggregateEntityPositionsResponse as GeneratedListAggregateEntityPositionsResponse,
    list_entity_positions_response::ListEntityPositionsResponse as GeneratedListEntityPositionsResponse,
    paginated_response::PaginatedResponse, position::Position,
};
use crate::utils::{PaginatedList, PaginationParams};

// ============================================================================
// REQUEST STRUCTS
// ============================================================================

/// Request parameters for listing entity positions
#[derive(Debug, Clone)]
pub struct ListEntityPositionsRequest {
    pub entity_id: String,
    pub pagination: PaginationParams,
}

impl ListEntityPositionsRequest {
    pub fn new(entity_id: impl Into<String>) -> Self {
        Self {
            entity_id: entity_id.into(),
            pagination: PaginationParams::new(),
        }
    }

    pub fn with_cursor(mut self, cursor: &str) -> Self {
        self.pagination = self.pagination.with_cursor(cursor);
        self
    }

    pub fn with_limit(mut self, limit: u32) -> Self {
        self.pagination = self.pagination.with_limit(limit);
        self
    }
}

/// Request parameters for listing aggregate entity positions
#[derive(Debug, Clone)]
pub struct ListAggregateEntityPositionsRequest {
    pub entity_id: String,
    pub pagination: PaginationParams,
}

impl ListAggregateEntityPositionsRequest {
    pub fn new(entity_id: impl Into<String>) -> Self {
        Self {
            entity_id: entity_id.into(),
            pagination: PaginationParams::new(),
        }
    }

    pub fn with_cursor(mut self, cursor: &str) -> Self {
        self.pagination = self.pagination.with_cursor(cursor);
        self
    }

    pub fn with_limit(mut self, limit: u32) -> Self {
        self.pagination = self.pagination.with_limit(limit);
        self
    }
}

// ============================================================================
// RESPONSE STRUCTS
// ============================================================================

/// Wrapper for entity position list response
pub struct ListEntityPositionsResponse {
    pub positions: Vec<Position>,
    pub pagination: PaginatedResponse,
}

impl From<GeneratedListEntityPositionsResponse> for ListEntityPositionsResponse {
    fn from(response: GeneratedListEntityPositionsResponse) -> Self {
        Self {
            positions: response.positions,
            pagination: *response.pagination,
        }
    }
}

impl ListEntityPositionsResponse {
    /// Get all entity positions
    pub fn positions(&self) -> &[Position] {
        &self.positions
    }

    /// Get the count of positions
    pub fn count(&self) -> usize {
        self.positions.len()
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
}

impl PaginatedList<Position> for ListEntityPositionsResponse {
    fn items(&self) -> &[Position] {
        &self.positions
    }

    fn pagination(&self) -> &PaginatedResponse {
        &self.pagination
    }
}

/// Wrapper for aggregate entity position list response
pub struct ListAggregateEntityPositionsResponse {
    pub positions: Vec<Position>,
    pub pagination: PaginatedResponse,
}

impl From<GeneratedListAggregateEntityPositionsResponse> for ListAggregateEntityPositionsResponse {
    fn from(response: GeneratedListAggregateEntityPositionsResponse) -> Self {
        Self {
            positions: response.positions,
            pagination: *response.pagination,
        }
    }
}

impl ListAggregateEntityPositionsResponse {
    /// Get all aggregate entity positions
    pub fn positions(&self) -> &[Position] {
        &self.positions
    }

    /// Get the count of positions
    pub fn count(&self) -> usize {
        self.positions.len()
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
}

impl PaginatedList<Position> for ListAggregateEntityPositionsResponse {
    fn items(&self) -> &[Position] {
        &self.positions
    }

    fn pagination(&self) -> &PaginatedResponse {
        &self.pagination
    }
}
