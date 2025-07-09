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
use crate::constants::DEFAULT_LIMIT;
use crate::types::generated::generated::{
    get_activity_response::GetActivityResponse,
    get_entity_activities_response::GetEntityActivitiesResponse,
    get_portfolio_activities_response::GetPortfolioActivitiesResponse,
};
use crate::utils::PaginationParams;
use core_rs::http_client::HttpClient;
use core_rs::http_method::HttpMethod;
use core_rs::http_request::HttpRequest;
use std::collections::HashMap;

use super::types::{
    GetActivityRequest, GetPortfolioActivityRequest, ListEntityActivitiesRequest,
    ListEntityActivitiesResponse, ListPortfolioActivitiesRequest, ListPortfolioActivitiesResponse,
};

/// Service for interacting with activity-related endpoints
pub struct ActivitiesService {
    client: Box<dyn HttpClient>,
}

impl ActivitiesService {
    /// Create a new ActivitiesService with the given PrimeClient
    pub fn new(prime_client: PrimeClient) -> Self {
        Self {
            client: prime_client.into_http_client(),
        }
    }

    /// Get an activity by its ID
    ///
    /// This endpoint can retrieve both portfolio and entity activities when passed the appropriate API key
    pub async fn get_activity(
        &self,
        request: GetActivityRequest,
    ) -> crate::error::HttpResult<GetActivityResponse> {
        let path = format!("activities/{}", request.activity_id);
        let req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;

        let resp = self.client.execute(req).await?;
        let response: GetActivityResponse = resp.json().await?;
        Ok(response)
    }

    /// Get a specific activity for a portfolio
    pub async fn get_portfolio_activity(
        &self,
        request: GetPortfolioActivityRequest,
    ) -> crate::error::HttpResult<GetActivityResponse> {
        let path = format!(
            "portfolios/{}/activities/{}",
            request.portfolio_id, request.activity_id
        );
        let req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;

        let resp = self.client.execute(req).await?;
        let response: GetActivityResponse = resp.json().await?;
        Ok(response)
    }

    /// List all activities associated with a given entity
    pub async fn list_entity_activities(
        &self,
        request: ListEntityActivitiesRequest,
    ) -> crate::error::HttpResult<ListEntityActivitiesResponse> {
        let path = format!("entities/{}/activities", request.entity_id);
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;

        // Build query parameters using pagination utility
        let mut query_params = HashMap::new();

        // Add pagination parameters
        let mut pagination = PaginationParams::new()
            .with_limit(request.pagination.limit.unwrap_or(DEFAULT_LIMIT))
            .with_sort_direction(
                request
                    .pagination
                    .sort_direction
                    .as_ref()
                    .map(|sd| match sd {
                        crate::SortDirection::Desc => "DESC",
                        crate::SortDirection::Asc => "ASC",
                    })
                    .unwrap_or("DESC"),
            );

        // Only add cursor if it's not empty
        if let Some(cursor) = &request.pagination.cursor {
            if !cursor.is_empty() {
                pagination = pagination.with_cursor(cursor);
            }
        }

        pagination.add_to_query_params(&mut query_params);

        // Add other parameters
        if let Some(level) = request.activity_level {
            query_params.insert("activity_level".to_string(), level.to_string());
        }
        if let Some(start) = request.pagination.start_date {
            query_params.insert("start_date".to_string(), start);
        }
        if let Some(end) = request.pagination.end_date {
            query_params.insert("end_date".to_string(), end);
        }

        if !query_params.is_empty() {
            req = req.with_query_params(query_params);
        }

        let resp = self.client.execute(req).await?;
        let response: GetEntityActivitiesResponse = resp.json().await?;
        Ok(response.into())
    }

    /// List all activities associated with a given portfolio
    pub async fn list_portfolio_activities(
        &self,
        request: ListPortfolioActivitiesRequest,
    ) -> crate::error::HttpResult<ListPortfolioActivitiesResponse> {
        let path = format!("portfolios/{}/activities", request.portfolio_id);
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;

        // Build query parameters using pagination utility
        let mut query_params = HashMap::new();

        // Add pagination parameters
        let mut pagination = PaginationParams::new()
            .with_limit(request.pagination.limit.unwrap_or(DEFAULT_LIMIT))
            .with_sort_direction(
                request
                    .pagination
                    .sort_direction
                    .as_ref()
                    .map(|sd| match sd {
                        crate::SortDirection::Desc => "DESC",
                        crate::SortDirection::Asc => "ASC",
                    })
                    .unwrap_or("DESC"),
            );

        // Only add cursor if it's not empty
        if let Some(cursor) = &request.pagination.cursor {
            if !cursor.is_empty() {
                pagination = pagination.with_cursor(cursor);
            }
        }

        pagination.add_to_query_params(&mut query_params);

        // Add date parameters
        if let Some(start) = request.pagination.start_date {
            query_params.insert("start_date".to_string(), start);
        }
        if let Some(end) = request.pagination.end_date {
            query_params.insert("end_date".to_string(), end);
        }

        if !query_params.is_empty() {
            req = req.with_query_params(query_params);
        }

        let resp = self.client.execute(req).await?;
        let response: GetPortfolioActivitiesResponse = resp.json().await?;
        Ok(response.into())
    }
}
