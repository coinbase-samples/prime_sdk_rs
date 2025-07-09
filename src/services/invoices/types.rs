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
    get_invoices_response::GetInvoicesResponse as GeneratedGetInvoicesResponse, invoice::Invoice,
    invoice_state::InvoiceState, paginated_response::PaginatedResponse,
};
use crate::utils::PaginatedList;

// ============================================================================
// REQUEST STRUCTS
// ============================================================================

/// Request to list entity invoices
#[derive(Debug, Clone)]
pub struct ListInvoicesRequest {
    pub entity_id: String,
    pub state: Option<InvoiceState>,
    pub year: Option<i32>,
    pub limit: Option<i32>,
    pub cursor: Option<String>,
}

impl ListInvoicesRequest {
    /// Create a new request to list entity invoices
    pub fn new(entity_id: impl Into<String>) -> Self {
        Self {
            entity_id: entity_id.into(),
            state: None,
            year: None,
            limit: None,
            cursor: None,
        }
    }

    /// Filter by invoice state
    pub fn with_state(mut self, state: InvoiceState) -> Self {
        self.state = Some(state);
        self
    }

    /// Filter by year
    pub fn with_year(mut self, year: i32) -> Self {
        self.year = Some(year);
        self
    }

    /// Set the number of results to return
    pub fn with_limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the cursor for pagination
    pub fn with_cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}

// ============================================================================
// RESPONSE STRUCTS
// ============================================================================

/// Response for listing entity invoices
#[derive(Debug, Clone)]
pub struct ListInvoicesResponse {
    pub invoices: Vec<Invoice>,
    pub pagination: PaginatedResponse,
}

impl ListInvoicesResponse {
    /// Get the list of invoices
    pub fn invoices(&self) -> &[Invoice] {
        &self.invoices
    }

    /// Get the pagination info
    pub fn pagination(&self) -> &PaginatedResponse {
        &self.pagination
    }

    /// Get the number of invoices
    pub fn count(&self) -> usize {
        self.invoices.len()
    }

    /// Check if there are any invoices
    pub fn is_empty(&self) -> bool {
        self.invoices.is_empty()
    }
}

impl PaginatedList<Invoice> for ListInvoicesResponse {
    fn items(&self) -> &[Invoice] {
        &self.invoices
    }

    fn pagination(&self) -> &PaginatedResponse {
        &self.pagination
    }
}

impl From<GeneratedGetInvoicesResponse> for ListInvoicesResponse {
    fn from(response: GeneratedGetInvoicesResponse) -> Self {
        Self {
            invoices: response.invoices.unwrap_or_default(),
            pagination: PaginatedResponse::default(), // API doesn't return pagination for this endpoint
        }
    }
}
