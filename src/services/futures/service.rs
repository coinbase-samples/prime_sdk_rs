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
use crate::error::HttpResult;
use crate::types::generated::generated::{
    cancel_futures_sweep_response::CancelFuturesSweepResponse as GeneratedCancelFuturesSweepResponse,
    get_fcm_balance_response::GetFcmBalanceResponse as GeneratedGetFcmBalanceResponse,
    get_futures_sweeps_response::GetFuturesSweepsResponse as GeneratedGetFuturesSweepsResponse,
    get_positions_response::GetPositionsResponse as GeneratedGetPositionsResponse,
    schedule_futures_sweep_request::ScheduleFuturesSweepRequest as GeneratedScheduleFuturesSweepRequest,
    schedule_futures_sweep_response::ScheduleFuturesSweepResponse as GeneratedScheduleFuturesSweepResponse,
    set_auto_sweep_request::SetAutoSweepRequest as GeneratedSetAutoSweepRequest,
    set_auto_sweep_response::SetAutoSweepResponse as GeneratedSetAutoSweepResponse,
};
use core_rs::http_client::HttpClient;
use core_rs::http_method::HttpMethod;
use core_rs::http_request::HttpRequest;
use std::collections::HashMap;

use super::types::*;

/// Service for interacting with futures-related endpoints
pub struct FuturesService {
    client: Box<dyn HttpClient>,
}

impl FuturesService {
    /// Create a new FuturesService with the given PrimeClient
    pub fn new(prime_client: PrimeClient) -> Self {
        Self {
            client: prime_client.into_http_client(),
        }
    }

    /// Get FCM balance for an entity
    pub async fn get_fcm_balance(
        &self,
        request: GetFcmBalanceRequest,
    ) -> HttpResult<GetFcmBalanceResponse> {
        let path = format!("entities/{}/futures/balance_summary", request.entity_id);
        let req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let resp = self.client.execute(req).await?;
        let response: GeneratedGetFcmBalanceResponse = resp.json().await?;
        Ok(response.into())
    }

    /// Get FCM positions for an entity (optionally filtered by product_id)
    pub async fn get_positions(
        &self,
        request: GetPositionsRequest,
    ) -> HttpResult<GetPositionsResponse> {
        let path = format!("entities/{}/futures/positions", request.entity_id);
        let mut req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let mut query_params = HashMap::new();
        if let Some(product_id) = &request.product_id {
            query_params.insert("product_id".to_string(), product_id.clone());
        }
        if !query_params.is_empty() {
            req = req.with_query_params(query_params);
        }
        let resp = self.client.execute(req).await?;
        let response: GeneratedGetPositionsResponse = resp.json().await?;
        Ok(response.into())
    }

    /// Get FCM futures sweeps for an entity
    pub async fn get_futures_sweeps(
        &self,
        request: GetFuturesSweepsRequest,
    ) -> HttpResult<GetFuturesSweepsResponse> {
        let path = format!("entities/{}/futures/sweeps", request.entity_id);
        let req = HttpRequest::new(HttpMethod::Get, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let resp = self.client.execute(req).await?;
        let response: GeneratedGetFuturesSweepsResponse = resp.json().await?;
        Ok(response.into())
    }

    /// Set auto sweep for an entity
    pub async fn set_auto_sweep(
        &self,
        request: SetAutoSweepRequest,
    ) -> HttpResult<SetAutoSweepResponse> {
        let path = format!("entities/{}/futures/auto_sweep", request.entity_id);
        let body = GeneratedSetAutoSweepRequest::from(&request);
        let json_body = serde_json::to_value(&body)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let req = HttpRequest::new(HttpMethod::Post, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?
            .with_json_body(json_body);
        let resp = self.client.execute(req).await?;
        let response: GeneratedSetAutoSweepResponse = resp.json().await?;
        Ok(response.into())
    }

    /// Schedule a futures sweep for an entity
    pub async fn schedule_futures_sweep(
        &self,
        request: ScheduleFuturesSweepRequest,
    ) -> HttpResult<ScheduleFuturesSweepResponse> {
        let path = format!("entities/{}/futures/sweeps", request.entity_id);
        let body = GeneratedScheduleFuturesSweepRequest::from(&request);
        let json_body = serde_json::to_value(&body)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let req = HttpRequest::new(HttpMethod::Post, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?
            .with_json_body(json_body);
        let resp = self.client.execute(req).await?;
        let response: GeneratedScheduleFuturesSweepResponse = resp.json().await?;
        Ok(response.into())
    }

    /// Cancel the pending futures sweep for an entity
    pub async fn cancel_futures_sweep(
        &self,
        request: CancelFuturesSweepRequest,
    ) -> HttpResult<CancelFuturesSweepResponse> {
        let path = format!("entities/{}/futures/sweeps", request.entity_id);
        let req = HttpRequest::new(HttpMethod::Delete, &path)
            .map_err(|e| crate::error::HttpError::Custom(e.to_string()))?;
        let resp = self.client.execute(req).await?;
        let response: GeneratedCancelFuturesSweepResponse = resp.json().await?;
        Ok(response.into())
    }
}
