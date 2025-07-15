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
use super::types::{
    ActivityCreationResponse, CreateOnchainAddressGroupRequest, ListOnchainAddressGroupsRequest,
    ListOnchainAddressGroupsResponse,
};
use crate::client::PrimeClient;
use crate::error::HttpResult;
use core_rs::http_client::HttpClient;
use core_rs::http_method::HttpMethod;
use core_rs::http_request::HttpRequest;

pub struct OnchainAddressGroupsService {
    client: Box<dyn HttpClient>,
}

impl OnchainAddressGroupsService {
    pub fn new(prime_client: PrimeClient) -> Self {
        Self {
            client: prime_client.into_http_client(),
        }
    }

    // Create Onchain Address Group
    pub async fn create_onchain_address_group(
        &self,
        request: CreateOnchainAddressGroupRequest,
    ) -> HttpResult<ActivityCreationResponse> {
        let path = format!("portfolios/{}/onchain_address_group", request.portfolio_id);
        let body = crate::types::generated::generated::change_onchain_address_group_request_is_a_request_to_create_or_update_a_new_onchain_address_group::ChangeOnchainAddressGroupRequestIsARequestToCreateOrUpdateANewOnchainAddressGroup::new(request.address_group);
        let req = HttpRequest::new(HttpMethod::Post, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?
            .with_json_body(
                serde_json::to_value(&body)
                    .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?,
            );
        let resp = self.client.execute(req).await?;
        let response: crate::types::generated::generated::activity_creation_response::ActivityCreationResponse = resp.json().await?;
        Ok(response.into())
    }

    // Update Onchain Address Group
    pub async fn update_onchain_address_group(
        &self,
        request: super::types::UpdateOnchainAddressGroupRequest,
    ) -> HttpResult<super::types::ActivityCreationResponse> {
        let path = format!("portfolios/{}/onchain_address_group", request.portfolio_id);
        let body = crate::types::generated::generated::change_onchain_address_group_request_is_a_request_to_create_or_update_a_new_onchain_address_group::ChangeOnchainAddressGroupRequestIsARequestToCreateOrUpdateANewOnchainAddressGroup::new(request.address_group);
        let req = HttpRequest::new(HttpMethod::Put, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?
            .with_json_body(
                serde_json::to_value(&body)
                    .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?,
            );
        let resp = self.client.execute(req).await?;
        let response: crate::types::generated::generated::activity_creation_response::ActivityCreationResponse = resp.json().await?;
        Ok(response.into())
    }

    // Delete Onchain Address Group
    pub async fn delete_onchain_address_group(
        &self,
        request: super::types::DeleteOnchainAddressGroupRequest,
    ) -> HttpResult<super::types::ActivityCreationResponse> {
        let path = format!(
            "portfolios/{}/onchain_address_group/{}",
            request.portfolio_id, request.address_group_id
        );
        let req = HttpRequest::new(HttpMethod::Delete, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let resp = self.client.execute(req).await?;
        let response: crate::types::generated::generated::activity_creation_response::ActivityCreationResponse = resp.json().await?;
        Ok(response.into())
    }

    // List Onchain Address Groups
    pub async fn list_onchain_address_groups(
        &self,
        request: ListOnchainAddressGroupsRequest,
    ) -> HttpResult<ListOnchainAddressGroupsResponse> {
        let path = format!("portfolios/{}/onchain_address_groups", request.portfolio_id);
        let req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let resp = self.client.execute(req).await?;
        let response: crate::types::generated::generated::list_onchain_address_groups_response::ListOnchainAddressGroupsResponse = resp.json().await?;
        Ok(response.into())
    }
}
