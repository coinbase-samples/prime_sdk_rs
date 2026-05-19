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
    allocation::Allocation,
    create_allocation_request::CreateAllocationRequest as GeneratedCreateAllocationRequest,
    create_allocation_response::CreateAllocationResponse as GeneratedCreateAllocationResponse,
    create_net_allocation_request::CreateNetAllocationRequest as GeneratedCreateNetAllocationRequest,
    create_net_allocation_response::CreateNetAllocationResponse as GeneratedCreateNetAllocationResponse,
    get_allocation_response::GetAllocationResponse as GeneratedGetAllocationResponse,
    get_allocations_by_client_netting_id_response::GetAllocationsByClientNettingIdResponse as GeneratedGetAllocationsByClientNettingIdResponse,
    get_portfolio_allocations_response::GetPortfolioAllocationsResponse as GeneratedGetPortfolioAllocationsResponse,
    paginated_response::PaginatedResponse, sort_direction::SortDirection,
};
use crate::utils::PaginatedList;

// ============================================================================
// REQUEST STRUCTS
// ============================================================================

/// Request to list portfolio allocations
#[derive(Debug, Clone)]
pub struct ListPortfolioAllocationsRequest {
    pub portfolio_id: String,
    pub product_ids: Option<Vec<String>>,
    pub order_side: Option<String>,
    pub start_date: String,
    pub end_date: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<i32>,
    pub sort_direction: Option<SortDirection>,
}

impl ListPortfolioAllocationsRequest {
    /// Create a new request to list portfolio allocations
    pub fn new(portfolio_id: impl Into<String>, start_date: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            start_date: start_date.into(),
            product_ids: None,
            order_side: None,
            end_date: None,
            cursor: None,
            limit: None,
            sort_direction: None,
        }
    }

    /// Filter by product IDs
    pub fn with_product_ids(mut self, product_ids: Vec<String>) -> Self {
        self.product_ids = Some(product_ids);
        self
    }

    /// Filter by order side
    pub fn with_order_side(mut self, order_side: impl Into<String>) -> Self {
        self.order_side = Some(order_side.into());
        self
    }

    /// Set the end date
    pub fn with_end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }

    /// Set the cursor for pagination
    pub fn with_cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    /// Set the number of results to return
    pub fn with_limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the sort direction
    pub fn with_sort_direction(mut self, sort_direction: SortDirection) -> Self {
        self.sort_direction = Some(sort_direction);
        self
    }
}

/// Request to list allocations by client netting ID
#[derive(Debug, Clone)]
pub struct ListAllocationsByClientNettingIdRequest {
    pub portfolio_id: String,
    pub netting_id: String,
    pub allocation_id: Option<String>,
}

impl ListAllocationsByClientNettingIdRequest {
    /// Create a new request to list allocations by client netting ID
    pub fn new(portfolio_id: impl Into<String>, netting_id: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            netting_id: netting_id.into(),
            allocation_id: None,
        }
    }

    /// Set the allocation ID
    pub fn with_allocation_id(mut self, allocation_id: impl Into<String>) -> Self {
        self.allocation_id = Some(allocation_id.into());
        self
    }
}

/// Request to get allocation by ID
#[derive(Debug, Clone)]
pub struct GetAllocationRequest {
    pub portfolio_id: String,
    pub allocation_id: String,
    pub netting_id: Option<String>,
}

impl GetAllocationRequest {
    /// Create a new request to get allocation by ID
    pub fn new(portfolio_id: impl Into<String>, allocation_id: impl Into<String>) -> Self {
        Self {
            portfolio_id: portfolio_id.into(),
            allocation_id: allocation_id.into(),
            netting_id: None,
        }
    }

    /// Set the netting ID
    pub fn with_netting_id(mut self, netting_id: impl Into<String>) -> Self {
        self.netting_id = Some(netting_id.into());
        self
    }
}

// ============================================================================
// RESPONSE STRUCTS
// ============================================================================

/// Response for listing portfolio allocations
#[derive(Debug, Clone)]
pub struct ListPortfolioAllocationsResponse {
    pub allocations: Vec<Allocation>,
    pub pagination: PaginatedResponse,
}

impl ListPortfolioAllocationsResponse {
    /// Get the list of allocations
    pub fn allocations(&self) -> &[Allocation] {
        &self.allocations
    }

    /// Get the pagination info
    pub fn pagination(&self) -> &PaginatedResponse {
        &self.pagination
    }

    /// Get the number of allocations
    pub fn count(&self) -> usize {
        self.allocations.len()
    }

    /// Check if there are any allocations
    pub fn is_empty(&self) -> bool {
        self.allocations.is_empty()
    }
}

impl PaginatedList<Allocation> for ListPortfolioAllocationsResponse {
    fn items(&self) -> &[Allocation] {
        &self.allocations
    }

    fn pagination(&self) -> &PaginatedResponse {
        &self.pagination
    }
}

impl From<GeneratedGetPortfolioAllocationsResponse> for ListPortfolioAllocationsResponse {
    fn from(response: GeneratedGetPortfolioAllocationsResponse) -> Self {
        Self {
            allocations: response.allocations.unwrap_or_default(),
            pagination: *response.pagination.unwrap_or_default(),
        }
    }
}

/// Response for listing allocations by client netting ID
#[derive(Debug, Clone)]
pub struct ListAllocationsByClientNettingIdResponse {
    pub allocations: Vec<Allocation>,
}

impl ListAllocationsByClientNettingIdResponse {
    /// Get the list of allocations
    pub fn allocations(&self) -> &[Allocation] {
        &self.allocations
    }

    /// Get the number of allocations
    pub fn count(&self) -> usize {
        self.allocations.len()
    }

    /// Check if there are any allocations
    pub fn is_empty(&self) -> bool {
        self.allocations.is_empty()
    }
}

impl From<GeneratedGetAllocationsByClientNettingIdResponse>
    for ListAllocationsByClientNettingIdResponse
{
    fn from(response: GeneratedGetAllocationsByClientNettingIdResponse) -> Self {
        Self {
            allocations: response.allocations.unwrap_or_default(),
        }
    }
}

/// Response for getting allocation by ID
#[derive(Debug, Clone)]
pub struct GetAllocationResponse {
    pub allocation: Option<Allocation>,
}

impl GetAllocationResponse {
    /// Get the allocation
    pub fn allocation(&self) -> Option<&Allocation> {
        self.allocation.as_ref()
    }

    /// Check if there is an allocation
    pub fn is_some(&self) -> bool {
        self.allocation.is_some()
    }

    /// Check if there is no allocation
    pub fn is_none(&self) -> bool {
        self.allocation.is_none()
    }
}

impl From<GeneratedGetAllocationResponse> for GetAllocationResponse {
    fn from(response: GeneratedGetAllocationResponse) -> Self {
        Self {
            allocation: response.allocation.map(|a| *a),
        }
    }
}

pub type CreateAllocationRequest = GeneratedCreateAllocationRequest;
pub type CreateAllocationResponse = GeneratedCreateAllocationResponse;

pub type CreateNetAllocationRequest = GeneratedCreateNetAllocationRequest;
pub type CreateNetAllocationResponse = GeneratedCreateNetAllocationResponse;
