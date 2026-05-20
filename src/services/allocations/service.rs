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
use crate::client::PrimeClient;
use crate::services::allocations::types::{
    CreateAllocationRequest, CreateAllocationResponse, CreateNetAllocationRequest,
    CreateNetAllocationResponse, GetAllocationRequest, GetAllocationResponse,
    ListAllocationsByClientNettingIdRequest, ListAllocationsByClientNettingIdResponse,
    ListPortfolioAllocationsRequest, ListPortfolioAllocationsResponse,
};
use crate::types::generated::generated::{
    create_allocation_request::CreateAllocationRequest as GeneratedCreateAllocationRequest,
    create_allocation_response::CreateAllocationResponse as GeneratedCreateAllocationResponse,
    create_net_allocation_request::CreateNetAllocationRequest as GeneratedCreateNetAllocationRequest,
    create_net_allocation_response::CreateNetAllocationResponse as GeneratedCreateNetAllocationResponse,
    get_allocation_response::GetAllocationResponse as GeneratedGetAllocationResponse,
    get_allocations_by_client_netting_id_response::GetAllocationsByClientNettingIdResponse as GeneratedGetAllocationsByClientNettingIdResponse,
    get_portfolio_allocations_response::GetPortfolioAllocationsResponse as GeneratedGetPortfolioAllocationsResponse,
};
use core_rs::http_client::HttpClient;
use core_rs::http_method::HttpMethod;
use core_rs::http_request::HttpRequest;
use std::collections::HashMap;

/// Service for managing allocations
pub struct AllocationService {
    client: Box<dyn HttpClient>,
}

impl AllocationService {
    /// Create a new allocation service
    pub fn new(prime_client: PrimeClient) -> Self {
        Self {
            client: prime_client.into_http_client(),
        }
    }

    /// List portfolio allocations
    pub async fn list_portfolio_allocations(
        &self,
        request: ListPortfolioAllocationsRequest,
    ) -> crate::error::HttpResult<ListPortfolioAllocationsResponse> {
        let path = format!("portfolios/{}/allocations", request.portfolio_id);
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;

        // Add query parameters
        let mut query_params = HashMap::new();

        // Required parameter
        query_params.insert("start_date".to_string(), request.start_date);

        // Optional parameters
        if let Some(product_ids) = request.product_ids {
            for product_id in product_ids {
                query_params.insert("product_ids".to_string(), product_id);
            }
        }

        if let Some(order_side) = request.order_side {
            query_params.insert("order_side".to_string(), order_side);
        }

        if let Some(end_date) = request.end_date {
            query_params.insert("end_date".to_string(), end_date);
        }

        if let Some(cursor) = request.cursor {
            query_params.insert("cursor".to_string(), cursor);
        }

        if let Some(limit) = request.limit {
            query_params.insert("limit".to_string(), limit.to_string());
        }

        if let Some(sort_direction) = request.sort_direction {
            query_params.insert("sort_direction".to_string(), sort_direction.to_string());
        }

        req = req.with_query_params(query_params);

        // Make the request
        let resp = self.client.execute(req).await?;
        let response: GeneratedGetPortfolioAllocationsResponse = resp.json().await?;

        Ok(response.into())
    }

    /// List allocations by client netting ID
    pub async fn list_allocations_by_client_netting_id(
        &self,
        request: ListAllocationsByClientNettingIdRequest,
    ) -> crate::error::HttpResult<ListAllocationsByClientNettingIdResponse> {
        let path = format!(
            "portfolios/{}/allocations/net/{}",
            request.portfolio_id, request.netting_id
        );
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;

        // Add query parameters
        if let Some(allocation_id) = request.allocation_id {
            let mut query_params = HashMap::new();
            query_params.insert("allocation_id".to_string(), allocation_id);
            req = req.with_query_params(query_params);
        }

        // Make the request
        let resp = self.client.execute(req).await?;
        let response: GeneratedGetAllocationsByClientNettingIdResponse = resp.json().await?;

        Ok(response.into())
    }

    /// Get allocation by ID
    pub async fn get_allocation(
        &self,
        request: GetAllocationRequest,
    ) -> crate::error::HttpResult<GetAllocationResponse> {
        let path = format!(
            "portfolios/{}/allocations/{}",
            request.portfolio_id, request.allocation_id
        );
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;

        // Add query parameters
        if let Some(netting_id) = request.netting_id {
            let mut query_params = HashMap::new();
            query_params.insert("netting_id".to_string(), netting_id);
            req = req.with_query_params(query_params);
        }

        // Make the request
        let resp = self.client.execute(req).await?;
        let response: GeneratedGetAllocationResponse = resp.json().await?;

        Ok(response.into())
    }

    /// Create an allocation for a portfolio
    pub async fn create_allocation(
        &self,
        request: CreateAllocationRequest,
    ) -> crate::error::HttpResult<CreateAllocationResponse> {
        let body = request;
        let json_body = serde_json::to_value(&body)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let req = HttpRequest::new(HttpMethod::Post, "allocations")
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?
            .with_json_body(json_body);

        let resp = self.client.execute(req).await?;
        let response: GeneratedCreateAllocationResponse = resp.json().await?;
        Ok(response)
    }

    /// Create a net allocation for a portfolio
    pub async fn create_net_allocation(
        &self,
        request: CreateNetAllocationRequest,
    ) -> crate::error::HttpResult<CreateNetAllocationResponse> {
        let body = request;
        let json_body = serde_json::to_value(&body)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let req = HttpRequest::new(HttpMethod::Post, "allocations/net")
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?
            .with_json_body(json_body);

        let resp = self.client.execute(req).await?;
        let response: GeneratedCreateNetAllocationResponse = resp.json().await?;
        Ok(response)
    }
}
