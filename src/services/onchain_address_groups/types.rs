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
    activity_creation_response::ActivityCreationResponse as GeneratedActivityCreationResponse,
    address_group::AddressGroup,
    list_onchain_address_groups_response::ListOnchainAddressGroupsResponse as GeneratedListOnchainAddressGroupsResponse,
    paginated_response::PaginatedResponse,
};

// ===================== REQUEST TYPES =====================

#[derive(Debug, Clone)]
pub struct CreateOnchainAddressGroupRequest {
    pub portfolio_id: String,
    pub address_group: AddressGroup,
}

#[derive(Debug, Clone)]
pub struct UpdateOnchainAddressGroupRequest {
    pub portfolio_id: String,
    pub address_group: AddressGroup,
}

#[derive(Debug, Clone)]
pub struct DeleteOnchainAddressGroupRequest {
    pub portfolio_id: String,
    pub address_group_id: String,
}

#[derive(Debug, Clone)]
pub struct ListOnchainAddressGroupsRequest {
    pub portfolio_id: String,
}

// ===================== RESPONSE TYPES =====================

#[derive(Debug, Clone)]
pub struct ListOnchainAddressGroupsResponse {
    pub address_groups: Vec<AddressGroup>,
    pub pagination: PaginatedResponse,
}

impl From<GeneratedListOnchainAddressGroupsResponse> for ListOnchainAddressGroupsResponse {
    fn from(resp: GeneratedListOnchainAddressGroupsResponse) -> Self {
        Self {
            address_groups: resp.address_groups,
            pagination: PaginatedResponse::default(), // fallback, since not present in generated type
        }
    }
}

impl ListOnchainAddressGroupsResponse {
    pub fn address_groups(&self) -> &[AddressGroup] {
        &self.address_groups
    }
    pub fn pagination(&self) -> &PaginatedResponse {
        &self.pagination
    }
}

#[derive(Debug, Clone)]
pub struct ActivityCreationResponse {
    pub activity_type: String,
    pub num_approvals_remaining: i32,
    pub activity_id: String,
}

impl From<GeneratedActivityCreationResponse> for ActivityCreationResponse {
    fn from(resp: GeneratedActivityCreationResponse) -> Self {
        Self {
            activity_type: resp.activity_type.to_string(),
            num_approvals_remaining: resp.num_approvals_remaining,
            activity_id: resp.activity_id,
        }
    }
}
