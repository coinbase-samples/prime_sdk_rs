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
use crate::constants::DEFAULT_LIMIT;
use std::collections::HashMap;

/// Pagination parameters for API requests
#[derive(Debug, Clone)]
pub struct PaginationParams {
    pub cursor: Option<String>,
    pub limit: Option<u32>,
    pub sort_direction: Option<String>,
}

impl PaginationParams {
    /// Create a new PaginationParams instance
    pub fn new() -> Self {
        Self {
            cursor: None,
            limit: None,
            sort_direction: None,
        }
    }

    /// Set the cursor for pagination
    pub fn with_cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    /// Set the limit for pagination
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the sort direction
    pub fn with_sort_direction(mut self, sort_direction: impl Into<String>) -> Self {
        self.sort_direction = Some(sort_direction.into());
        self
    }

    /// Convert pagination parameters to query parameters
    pub fn to_query_params(&self) -> HashMap<String, String> {
        let mut query_params = HashMap::new();

        if let Some(cursor_val) = &self.cursor {
            query_params.insert("cursor".to_string(), cursor_val.clone());
        }
        if let Some(limit_val) = &self.limit {
            query_params.insert("limit".to_string(), limit_val.to_string());
        }
        if let Some(sort) = &self.sort_direction {
            query_params.insert("sort_direction".to_string(), sort.clone());
        }

        query_params
    }

    /// Add pagination parameters to an existing query parameters map
    pub fn add_to_query_params(&self, query_params: &mut HashMap<String, String>) {
        if let Some(cursor_val) = &self.cursor {
            query_params.insert("cursor".to_string(), cursor_val.clone());
        }
        if let Some(limit_val) = &self.limit {
            query_params.insert("limit".to_string(), limit_val.to_string());
        }
        if let Some(sort) = &self.sort_direction {
            query_params.insert("sort_direction".to_string(), sort.clone());
        }
    }
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Common sort directions
pub const SORT_ASC: &str = "ASC";
pub const SORT_DESC: &str = "DESC";

/// Helper function to create pagination params with common defaults
pub fn pagination_with_defaults(
    cursor: Option<String>,
    limit: Option<u32>,
    sort_direction: Option<String>,
) -> PaginationParams {
    PaginationParams::new()
        .with_cursor(cursor.unwrap_or_default())
        .with_limit(limit.unwrap_or(DEFAULT_LIMIT) as u32)
        .with_sort_direction(sort_direction.unwrap_or_else(|| SORT_DESC.to_string()))
}
